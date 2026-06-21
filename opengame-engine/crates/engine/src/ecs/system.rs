use super::world::World;

type RenderSystemFn = Box<dyn FnMut(&mut World, f32)>;

pub trait System {
    fn update(&mut self, world: &mut World, dt: f32);
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub struct SystemScheduler {
    startup_systems: Vec<Box<dyn FnMut(&mut World)>>,
    update_systems: Vec<Box<dyn System>>,
    render_systems: Vec<RenderSystemFn>,
}

impl SystemScheduler {
    pub fn new() -> Self {
        Self {
            startup_systems: Vec::new(),
            update_systems: Vec::new(),
            render_systems: Vec::new(),
        }
    }

    pub fn add_startup_system(&mut self, system: impl FnMut(&mut World) + 'static) {
        self.startup_systems.push(Box::new(system));
    }

    pub fn add_system(&mut self, system: impl System + 'static) {
        self.update_systems.push(Box::new(system));
    }

    pub fn add_render_system(&mut self, system: impl FnMut(&mut World, f32) + 'static) {
        self.render_systems.push(Box::new(system));
    }

    pub fn run_startup(&mut self, world: &mut World) {
        for system in &mut self.startup_systems {
            (system)(world);
        }
        self.startup_systems.clear();
    }

    pub fn run_update(&mut self, world: &mut World, dt: f32) {
        for system in &mut self.update_systems {
            system.update(world, dt);
        }
    }

    pub fn run_render(&mut self, world: &mut World, alpha: f32) {
        for system in &mut self.render_systems {
            (system)(world, alpha);
        }
    }
}

impl Default for SystemScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl<F> System for F
where
    F: FnMut(&mut World, f32) + 'static,
{
    fn update(&mut self, world: &mut World, dt: f32) {
        (self)(world, dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestSystem {
        pub run_count: u32,
    }

    impl System for TestSystem {
        fn update(&mut self, _world: &mut World, _dt: f32) {
            self.run_count += 1;
        }

        fn name(&self) -> &str {
            "TestSystem"
        }
    }

    #[test]
    fn test_system_scheduler_startup() {
        let mut scheduler = SystemScheduler::new();
        let mut world = World::new();
        let ran = std::rc::Rc::new(std::cell::Cell::new(false));
        let ran_clone = ran.clone();

        scheduler.add_startup_system(move |_: &mut World| {
            ran_clone.set(true);
        });

        scheduler.run_startup(&mut world);
        assert!(ran.get());
    }

    #[test]
    fn test_system_scheduler_startup_clears() {
        let mut scheduler = SystemScheduler::new();
        let mut world = World::new();
        let count = std::rc::Rc::new(std::cell::Cell::new(0));
        let count_clone = count.clone();

        scheduler.add_startup_system(move |_: &mut World| {
            count_clone.set(count_clone.get() + 1);
        });

        scheduler.run_startup(&mut world);
        scheduler.run_startup(&mut world);
        assert_eq!(count.get(), 1);
    }

    #[test]
    fn test_system_scheduler_update() {
        let mut scheduler = SystemScheduler::new();
        let mut world = World::new();

        let sys = TestSystem { run_count: 0 };
        scheduler.add_system(sys);

        scheduler.run_update(&mut world, 0.016);
        scheduler.run_update(&mut world, 0.016);
    }

    #[test]
    fn test_system_scheduler_render() {
        let mut scheduler = SystemScheduler::new();
        let mut world = World::new();
        let called = std::rc::Rc::new(std::cell::Cell::new(false));
        let called_clone = called.clone();

        scheduler.add_render_system(move |_: &mut World, _: f32| {
            called_clone.set(true);
        });

        scheduler.run_render(&mut world, 1.0);
        assert!(called.get());
    }

    #[test]
    fn test_closure_system() {
        let mut scheduler = SystemScheduler::new();
        let mut world = World::new();
        let count = std::rc::Rc::new(std::cell::Cell::new(0));
        let count_clone = count.clone();

        scheduler.add_system(move |_: &mut World, _: f32| {
            count_clone.set(count_clone.get() + 1);
        });

        scheduler.run_update(&mut world, 0.016);
        scheduler.run_update(&mut world, 0.016);
        assert_eq!(count.get(), 2);
    }

    #[test]
    fn test_system_name() {
        let sys = TestSystem { run_count: 0 };
        assert_eq!(sys.name(), "TestSystem");
    }

    #[test]
    fn test_default_scheduler() {
        let mut scheduler = SystemScheduler::default();
        let mut world = World::new();
        scheduler.run_update(&mut world, 0.016);
        scheduler.run_render(&mut world, 1.0);
    }
}
