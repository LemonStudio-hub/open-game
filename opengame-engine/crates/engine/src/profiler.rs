use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Profiler {
    scopes: HashMap<String, ScopeStats>,
    frame_times: VecDeque<f32>,
    max_frame_history: usize,
    current_frame: FrameData,
}

#[derive(Debug, Clone)]
struct ScopeStats {
    total_time_us: f64,
    call_count: u64,
    last_time_us: f64,
    max_time_us: f64,
}

#[derive(Debug, Clone, Default)]
struct FrameData {
    entries: Vec<(String, f64)>,
    frame_start_us: f64,
}

#[derive(Debug, Clone)]
pub struct ScopeGuard {
    name: String,
    start_us: f64,
}

#[derive(Debug, Clone)]
pub struct ProfileReport {
    pub scopes: Vec<ScopeEntry>,
    pub frame_time_ms: f32,
    pub avg_frame_time_ms: f32,
    pub fps: f32,
}

#[derive(Debug, Clone)]
pub struct ScopeEntry {
    pub name: String,
    pub last_us: f64,
    pub avg_us: f64,
    pub max_us: f64,
    pub call_count: u64,
    pub percentage: f64,
}

impl Profiler {
    pub fn new(max_frame_history: usize) -> Self {
        Self {
            scopes: HashMap::new(),
            frame_times: VecDeque::with_capacity(max_frame_history),
            max_frame_history,
            current_frame: FrameData::default(),
        }
    }

    pub fn begin_frame(&mut self, timestamp_us: f64) {
        self.current_frame = FrameData {
            entries: Vec::new(),
            frame_start_us: timestamp_us,
        };
    }

    pub fn end_frame(&mut self, timestamp_us: f64) {
        let frame_time = (timestamp_us - self.current_frame.frame_start_us) as f32;
        if self.frame_times.len() >= self.max_frame_history {
            self.frame_times.pop_front();
        }
        self.frame_times.push_back(frame_time);

        for (name, duration) in &self.current_frame.entries {
            let stats = self
                .scopes
                .entry(name.clone())
                .or_insert_with(|| ScopeStats {
                    total_time_us: 0.0,
                    call_count: 0,
                    last_time_us: 0.0,
                    max_time_us: 0.0,
                });

            stats.last_time_us = *duration;
            stats.total_time_us += *duration;
            stats.call_count += 1;
            if *duration > stats.max_time_us {
                stats.max_time_us = *duration;
            }
        }
    }

    pub fn record_scope(&mut self, name: &str, duration_us: f64) {
        self.current_frame
            .entries
            .push((name.to_string(), duration_us));
    }

    pub fn begin_scope(&self, name: &str) -> ScopeGuard {
        ScopeGuard::new(name)
    }

    pub fn report(&self) -> ProfileReport {
        let frame_time_ms = self.frame_times.back().copied().unwrap_or(0.0) / 1000.0;
        let avg_frame_time_ms = if self.frame_times.is_empty() {
            0.0
        } else {
            self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32 / 1000.0
        };
        let fps = if avg_frame_time_ms > 0.0 {
            1000.0 / avg_frame_time_ms
        } else {
            0.0
        };

        let total_time: f64 = self.scopes.values().map(|s| s.last_time_us).sum();

        let mut scopes: Vec<ScopeEntry> = self
            .scopes
            .iter()
            .map(|(name, stats)| {
                let avg = if stats.call_count > 0 {
                    stats.total_time_us / stats.call_count as f64
                } else {
                    0.0
                };
                let percentage = if total_time > 0.0 {
                    stats.last_time_us / total_time * 100.0
                } else {
                    0.0
                };
                ScopeEntry {
                    name: name.clone(),
                    last_us: stats.last_time_us,
                    avg_us: avg,
                    max_us: stats.max_time_us,
                    call_count: stats.call_count,
                    percentage,
                }
            })
            .collect();

        scopes.sort_by(|a, b| {
            b.last_us
                .partial_cmp(&a.last_us)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        ProfileReport {
            scopes,
            frame_time_ms,
            avg_frame_time_ms,
            fps,
        }
    }

    pub fn frame_time_ms(&self) -> f32 {
        self.frame_times.back().copied().unwrap_or(0.0) / 1000.0
    }

    pub fn avg_frame_time_ms(&self) -> f32 {
        if self.frame_times.is_empty() {
            0.0
        } else {
            self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32 / 1000.0
        }
    }

    pub fn fps(&self) -> f32 {
        let avg = self.avg_frame_time_ms();
        if avg > 0.0 {
            1000.0 / avg
        } else {
            0.0
        }
    }

    pub fn frame_history(&self) -> &VecDeque<f32> {
        &self.frame_times
    }

    pub fn reset(&mut self) {
        self.scopes.clear();
        self.frame_times.clear();
        self.current_frame = FrameData::default();
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new(120)
    }
}

impl ScopeGuard {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start_us: now_us(),
        }
    }

    pub fn finish(self, profiler: &mut Profiler) {
        let elapsed = now_us() - self.start_us;
        profiler.record_scope(&self.name, elapsed);
    }
}

fn now_us() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now() * 1000.0)
            .unwrap_or(0.0)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::Instant;
        static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
        let start = START.get_or_init(Instant::now);
        start.elapsed().as_micros() as f64
    }
}

impl ProfileReport {
    pub fn format_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "FPS: {:.1} | Frame: {:.2}ms | Avg: {:.2}ms\n",
            self.fps, self.frame_time_ms, self.avg_frame_time_ms
        ));
        out.push_str("--- Profiler ---\n");
        for entry in &self.scopes {
            out.push_str(&format!(
                "  {:<20} {:>8.1}us ({:>5.1}%) [avg:{:>8.1}us max:{:>8.1}us]\n",
                entry.name, entry.last_us, entry.percentage, entry.avg_us, entry.max_us
            ));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_new() {
        let p = Profiler::new(60);
        assert_eq!(p.frame_history().len(), 0);
    }

    #[test]
    fn test_profiler_default() {
        let p = Profiler::default();
        assert_eq!(p.frame_history().len(), 0);
    }

    #[test]
    fn test_profiler_record_scope() {
        let mut p = Profiler::new(60);
        p.begin_frame(0.0);
        p.record_scope("render", 1000.0);
        p.record_scope("update", 500.0);
        p.end_frame(2000.0);

        let report = p.report();
        assert_eq!(report.scopes.len(), 2);
    }

    #[test]
    fn test_profiler_frame_time() {
        let mut p = Profiler::new(60);
        p.begin_frame(0.0);
        p.end_frame(16000.0);

        assert!((p.frame_time_ms() - 16.0).abs() < 0.01);
    }

    #[test]
    fn test_profiler_fps() {
        let mut p = Profiler::new(60);
        for i in 0..10 {
            let start = i as f64 * 16000.0;
            p.begin_frame(start);
            p.end_frame(start + 16000.0);
        }

        let fps = p.fps();
        assert!((fps - 62.5).abs() < 1.0);
    }

    #[test]
    fn test_profiler_reset() {
        let mut p = Profiler::new(60);
        p.begin_frame(0.0);
        p.record_scope("test", 100.0);
        p.end_frame(1000.0);

        p.reset();
        assert_eq!(p.frame_history().len(), 0);
        assert_eq!(p.report().scopes.len(), 0);
    }

    #[test]
    fn test_profiler_max_history() {
        let mut p = Profiler::new(3);
        for i in 0..5 {
            p.begin_frame(i as f64 * 1000.0);
            p.end_frame(i as f64 * 1000.0 + 16000.0);
        }
        assert_eq!(p.frame_history().len(), 3);
    }

    #[test]
    fn test_report_format() {
        let mut p = Profiler::new(60);
        p.begin_frame(0.0);
        p.record_scope("render", 8000.0);
        p.end_frame(16000.0);

        let report = p.report();
        let text = report.format_text();
        assert!(text.contains("FPS:"));
        assert!(text.contains("render"));
    }
}
