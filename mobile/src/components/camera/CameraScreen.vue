<script setup lang="ts">
import { ref, reactive, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
import StatusBar from '../StatusBar.vue'
import { usePhotoStore } from '../../composables/usePhotoStore'
import { useSwipeGestures } from '../../composables/useSwipeGestures'

const emit = defineEmits<{
  'go-back': []
  'go-home': []
}>()

const { targetRef, isDragging } =
  useSwipeGestures({
    onSwipeUp: () => emit('go-home'),
    onSwipeRight: () => emit('go-back'),
  })

// ─── Camera State ───────────────────────────────────────────────
const videoRef = ref<HTMLVideoElement | null>(null)
const canvasRef = ref<HTMLCanvasElement | null>(null)
const stream = ref<MediaStream | null>(null)
const facingMode = ref<'user' | 'environment'>('environment')
const cameraReady = ref(false)
const cameraError = ref('')

// ─── Mode: Photo / Video ────────────────────────────────────────
type CaptureMode = 'photo' | 'video'
const captureMode = ref<CaptureMode>('photo')

// ─── Photo Capture ──────────────────────────────────────────────
const { photos, addPhoto, deletePhoto: storeDeletePhoto } = usePhotoStore()
const showShutterFlash = ref(false)

// ─── Video Recording ────────────────────────────────────────────
const isRecording = ref(false)
const mediaRecorder = ref<MediaRecorder | null>(null)
const recordedChunks = ref<Blob[]>([])
const recordingTime = ref(0)
let recordingInterval: ReturnType<typeof setInterval> | null = null

const recordingTimeFormatted = computed(() => {
  const m = Math.floor(recordingTime.value / 60)
  const s = recordingTime.value % 60
  return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`
})

// ─── Flash ──────────────────────────────────────────────────────
type FlashMode = 'off' | 'on' | 'torch'
const flashMode = ref<FlashMode>('off')
const flashSupported = ref(false)
const flashMenuOpen = ref(false)

// ─── Grid ───────────────────────────────────────────────────────
const showGrid = ref(false)

// ─── Timer ──────────────────────────────────────────────────────
type TimerOption = 0 | 3 | 10
const timerDuration = ref<TimerOption>(0)
const timerMenuOpen = ref(false)
const countdown = ref(0)
let countdownInterval: ReturnType<typeof setInterval> | null = null

// ─── Zoom ───────────────────────────────────────────────────────
const zoomLevel = ref(1)
const zoomSupported = ref(false)
const zoomMin = ref(1)
const zoomMax = ref(5)

// ─── Filters ────────────────────────────────────────────────────
interface FilterOption {
  name: string
  css: string
}
const filters: FilterOption[] = [
  { name: 'None', css: 'none' },
  { name: 'Vivid', css: 'saturate(1.4) contrast(1.1)' },
  { name: 'Warm', css: 'sepia(0.25) saturate(1.3) brightness(1.05)' },
  { name: 'Cool', css: 'hue-rotate(20deg) saturate(0.9) brightness(1.05)' },
  { name: 'Mono', css: 'grayscale(1)' },
  { name: 'Noir', css: 'grayscale(1) contrast(1.3) brightness(0.9)' },
  { name: 'Fade', css: 'saturate(0.6) brightness(1.1) contrast(0.85)' },
  { name: 'Dramatic', css: 'contrast(1.4) saturate(0.8) brightness(0.95)' },
]
const activeFilterIndex = ref(0)

// ─── Focus ──────────────────────────────────────────────────────
const focusPoint = reactive({ x: 0, y: 0, visible: false })
let focusTimeout: ReturnType<typeof setTimeout> | null = null

// ─── Gallery ────────────────────────────────────────────────────
const showGallery = ref(false)
const galleryIndex = ref(0)

// ─── More options ───────────────────────────────────────────────
const showMoreMenu = ref(false)

// ─── Helpers ────────────────────────────────────────────────────
const activeFilterCss = computed(() => filters[activeFilterIndex.value].css)

// ─── Camera Initialization ──────────────────────────────────────
async function startCamera() {
  cameraError.value = ''
  cameraReady.value = false

  try {
    const constraints: MediaStreamConstraints = {
      video: {
        facingMode: facingMode.value,
        width: { ideal: 1920 },
        height: { ideal: 1080 },
      },
      audio: captureMode.value === 'video',
    }

    const newStream = await navigator.mediaDevices.getUserMedia(constraints)

    // Stop old tracks
    if (stream.value) {
      stream.value.getTracks().forEach(t => t.stop())
    }

    stream.value = newStream

    await nextTick()
    if (videoRef.value) {
      videoRef.value.srcObject = newStream
      await videoRef.value.play()
    }

    // Check flash support
    const videoTrack = newStream.getVideoTracks()[0]
    if (videoTrack) {
      const capabilities = videoTrack.getCapabilities?.() as any
      flashSupported.value = !!(capabilities && capabilities.torch)
      zoomSupported.value = !!(capabilities && capabilities.zoom)
      if (zoomSupported.value) {
        zoomMin.value = capabilities.zoom.min || 1
        zoomMax.value = capabilities.zoom.max || 5
      }
    }

    cameraReady.value = true
  } catch (err: any) {
    cameraError.value = err.name === 'NotAllowedError'
      ? 'Camera access denied. Please allow camera permissions.'
      : err.name === 'NotFoundError'
        ? 'No camera found on this device.'
        : `Camera error: ${err.message}`
  }
}

function stopCamera() {
  if (stream.value) {
    stream.value.getTracks().forEach(t => t.stop())
    stream.value = null
  }
  if (videoRef.value) {
    videoRef.value.srcObject = null
  }
  cameraReady.value = false
}

async function switchCamera() {
  facingMode.value = facingMode.value === 'environment' ? 'user' : 'environment'
  await startCamera()
}

// ─── Photo Capture ──────────────────────────────────────────────
function capturePhoto() {
  if (!cameraReady.value || !videoRef.value || !canvasRef.value) return

  if (timerDuration.value > 0 && countdown.value === 0) {
    startCountdown(() => doCapturePhoto())
    return
  }

  doCapturePhoto()
}

function doCapturePhoto() {
  if (!videoRef.value || !canvasRef.value) return

  const video = videoRef.value
  const canvas = canvasRef.value
  canvas.width = video.videoWidth
  canvas.height = video.videoHeight

  const ctx = canvas.getContext('2d')!
  // Apply filter to canvas
  ctx.filter = activeFilterCss.value === 'none' ? 'none' : activeFilterCss.value
  ctx.drawImage(video, 0, 0, canvas.width, canvas.height)

  const dataUrl = canvas.toDataURL('image/jpeg', 0.92)
  addPhoto(dataUrl, 'photo')

  // Shutter flash
  showShutterFlash.value = true
  setTimeout(() => { showShutterFlash.value = false }, 150)
}

// ─── Video Recording ────────────────────────────────────────────
function toggleRecording() {
  if (isRecording.value) {
    stopRecording()
  } else {
    startRecording()
  }
}

function startRecording() {
  if (!stream.value) return

  recordedChunks.value = []
  const mimeType = MediaRecorder.isTypeSupported('video/webm;codecs=vp9')
    ? 'video/webm;codecs=vp9'
    : 'video/webm'

  try {
    mediaRecorder.value = new MediaRecorder(stream.value, { mimeType })
  } catch {
    mediaRecorder.value = new MediaRecorder(stream.value)
  }

  mediaRecorder.value.ondataavailable = (e) => {
    if (e.data.size > 0) recordedChunks.value.push(e.data)
  }

  mediaRecorder.value.onstop = () => {
    const blob = new Blob(recordedChunks.value, { type: 'video/webm' })
    const url = URL.createObjectURL(blob)
    addPhoto(url, 'video')
  }

  mediaRecorder.value.start(100)
  isRecording.value = true
  recordingTime.value = 0

  recordingInterval = setInterval(() => {
    recordingTime.value++
  }, 1000)
}

function stopRecording() {
  if (mediaRecorder.value && mediaRecorder.value.state !== 'inactive') {
    mediaRecorder.value.stop()
  }
  isRecording.value = false
  if (recordingInterval) {
    clearInterval(recordingInterval)
    recordingInterval = null
  }
}

// ─── Flash Control ──────────────────────────────────────────────
function toggleFlash() {
  const modes: FlashMode[] = ['off', 'on', 'torch']
  const idx = modes.indexOf(flashMode.value)
  flashMode.value = modes[(idx + 1) % modes.length]
  applyFlash()
}

function setFlashMode(mode: FlashMode) {
  flashMode.value = mode
  flashMenuOpen.value = false
  applyFlash()
}

function applyFlash() {
  if (!stream.value) return
  const track = stream.value.getVideoTracks()[0]
  if (!track) return

  try {
    track.applyConstraints({
      advanced: [{ torch: flashMode.value === 'torch' } as any]
    })
  } catch {
    // Torch not supported on this device
  }
}

// ─── Grid ───────────────────────────────────────────────────────
function toggleGrid() {
  showGrid.value = !showGrid.value
}

// ─── Timer ──────────────────────────────────────────────────────
function setTimer(duration: TimerOption) {
  timerDuration.value = duration
  timerMenuOpen.value = false
}

function startCountdown(callback: () => void) {
  countdown.value = timerDuration.value
  countdownInterval = setInterval(() => {
    countdown.value--
    if (countdown.value <= 0) {
      if (countdownInterval) clearInterval(countdownInterval)
      countdownInterval = null
      callback()
    }
  }, 1000)
}

// ─── Zoom ───────────────────────────────────────────────────────
function setZoom() {
  if (!stream.value || !zoomSupported.value) return
  const track = stream.value.getVideoTracks()[0]
  if (!track) return

  try {
    track.applyConstraints({
      advanced: [{ zoom: zoomLevel.value } as any]
    })
  } catch {
    // Zoom not supported
  }
}

watch(zoomLevel, () => setZoom())

// ─── Focus ──────────────────────────────────────────────────────
function handleViewfinderTap(e: MouseEvent | TouchEvent) {
  if (showGallery.value || showMoreMenu.value) return

  const target = (e.currentTarget as HTMLElement).getBoundingClientRect()
  const clientX = 'touches' in e ? e.touches[0].clientX : e.clientX
  const clientY = 'touches' in e ? e.touches[0].clientY : e.clientY

  focusPoint.x = clientX - target.left
  focusPoint.y = clientY - target.top
  focusPoint.visible = true

  if (focusTimeout) clearTimeout(focusTimeout)
  focusTimeout = setTimeout(() => {
    focusPoint.visible = false
  }, 1200)

  // Try to set exposure point if supported
  if (stream.value) {
    const track = stream.value.getVideoTracks()[0]
    try {
      track.applyConstraints({
        advanced: [{ pointsOfInterest: [{ x: focusPoint.x / target.width, y: focusPoint.y / target.height }] } as any]
      })
    } catch {
      // Not supported
    }
  }
}

// ─── Gallery ────────────────────────────────────────────────────
function openGallery() {
  if (photos.value.length === 0) return
  galleryIndex.value = 0
  showGallery.value = true
}

function closeGallery() {
  showGallery.value = false
}

function galleryPrev() {
  if (galleryIndex.value > 0) galleryIndex.value--
}

function galleryNext() {
  if (galleryIndex.value < photos.value.length - 1) galleryIndex.value++
}

function deleteCapture(index: number) {
  const photo = photos.value[index]
  if (photo) storeDeletePhoto(photo.id)
  if (photos.value.length === 0) {
    showGallery.value = false
  } else if (galleryIndex.value >= photos.value.length) {
    galleryIndex.value = photos.value.length - 1
  }
}

function downloadCapture(index: number) {
  const photo = photos.value[index]
  if (!photo) return
  const a = document.createElement('a')
  a.href = photo.data
  a.download = photo.type === 'photo' ? `photo-${Date.now()}.jpg` : `video-${Date.now()}.webm`
  a.click()
}

// ─── Close / Back ───────────────────────────────────────────────
function handleClose() {
  stopCamera()
  stopRecording()
  emit('go-back')
}

// ─── Close menus on outside click ───────────────────────────────
function closeAllMenus() {
  flashMenuOpen.value = false
  timerMenuOpen.value = false
  showMoreMenu.value = false
}

// ─── Lifecycle ──────────────────────────────────────────────────
onMounted(() => {
  startCamera()
})

onUnmounted(() => {
  stopCamera()
  if (recordingInterval) clearInterval(recordingInterval)
  if (countdownInterval) clearInterval(countdownInterval)
  if (focusTimeout) clearTimeout(focusTimeout)
})
</script>

<template>
  <div
    :ref="targetRef"
    class="camera-screen"
    :class="{ dragging: isDragging }"
    @click="closeAllMenus"
  >
    <!-- Status Bar -->
    <div class="camera-status-bar">
      <StatusBar />
    </div>

    <!-- Viewfinder -->
    <div class="viewfinder-container" @click="handleViewfinderTap">
      <video
        ref="videoRef"
        class="viewfinder"
        :class="{ mirrored: facingMode === 'user' }"
        :style="{ filter: activeFilterCss }"
        autoplay
        playsinline
        muted
      ></video>

      <!-- Grid Overlay -->
      <div v-if="showGrid" class="grid-overlay">
        <div class="grid-line grid-h1"></div>
        <div class="grid-line grid-h2"></div>
        <div class="grid-line grid-v1"></div>
        <div class="grid-line grid-v2"></div>
      </div>

      <!-- Focus Ring -->
      <Transition name="focus-ring">
        <div
          v-if="focusPoint.visible"
          class="focus-ring"
          :style="{ left: focusPoint.x + 'px', top: focusPoint.y + 'px' }"
        >
          <svg viewBox="0 0 60 60" fill="none" stroke="currentColor" stroke-width="1.5">
            <rect x="4" y="4" width="52" height="52" rx="4" />
          </svg>
        </div>
      </Transition>

      <!-- Countdown Overlay -->
      <Transition name="countdown">
        <div v-if="countdown > 0" class="countdown-overlay">
          <span class="countdown-number">{{ countdown }}</span>
        </div>
      </Transition>

      <!-- Shutter Flash -->
      <Transition name="flash">
        <div v-if="showShutterFlash" class="shutter-flash"></div>
      </Transition>

      <!-- Recording Indicator -->
      <div v-if="isRecording" class="recording-indicator">
        <span class="rec-dot"></span>
        <span class="rec-time">{{ recordingTimeFormatted }}</span>
      </div>

      <!-- Camera Error -->
      <div v-if="cameraError" class="camera-error">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <circle cx="12" cy="12" r="10" />
          <path d="M15 9l-6 6M9 9l6 6" stroke-linecap="round" />
        </svg>
        <p>{{ cameraError }}</p>
      </div>
    </div>

    <!-- Top Bar -->
    <div class="top-bar">
      <div class="top-bar-left">
        <!-- Close button -->
        <button class="top-btn close-btn" @click.stop="handleClose" aria-label="Close camera">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M18 6L6 18M6 6l12 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" fill="none" />
          </svg>
        </button>
      </div>

      <div class="top-bar-right">
        <!-- Flash button -->
        <div class="top-btn-wrapper" @click.stop>
          <button
            class="top-btn"
            :class="{ active: flashMode !== 'off' }"
            @click.stop="toggleFlash"
            aria-label="Flash"
          >
            <svg v-if="flashMode === 'off'" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" stroke-linecap="round" stroke-linejoin="round" />
              <path d="M2 2l20 20" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
            </svg>
            <svg v-else-if="flashMode === 'on'" viewBox="0 0 24 24" fill="currentColor">
              <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
            </svg>
            <svg v-else viewBox="0 0 24 24" fill="currentColor">
              <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
              <circle cx="18" cy="6" r="3" fill="#ffd60a" />
            </svg>
          </button>
          <!-- Flash dropdown -->
          <Transition name="dropdown">
            <div v-if="flashMenuOpen" class="dropdown-menu">
              <button :class="{ selected: flashMode === 'off' }" @click.stop="setFlashMode('off')">Off</button>
              <button :class="{ selected: flashMode === 'on' }" @click.stop="setFlashMode('on')">On</button>
              <button :class="{ selected: flashMode === 'torch' }" @click.stop="setFlashMode('torch')">Torch</button>
            </div>
          </Transition>
        </div>

        <!-- Timer button -->
        <div class="top-btn-wrapper" @click.stop>
          <button
            class="top-btn"
            :class="{ active: timerDuration > 0 }"
            @click.stop="timerMenuOpen = !timerMenuOpen"
            aria-label="Timer"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <circle cx="12" cy="13" r="8" />
              <path d="M12 9v4l2 2" stroke-linecap="round" />
              <path d="M9 2h6" stroke-linecap="round" />
              <path d="M12 2v2" stroke-linecap="round" />
            </svg>
            <span v-if="timerDuration > 0" class="btn-badge">{{ timerDuration }}s</span>
          </button>
          <Transition name="dropdown">
            <div v-if="timerMenuOpen" class="dropdown-menu">
              <button :class="{ selected: timerDuration === 0 }" @click.stop="setTimer(0)">Off</button>
              <button :class="{ selected: timerDuration === 3 }" @click.stop="setTimer(3)">3s</button>
              <button :class="{ selected: timerDuration === 10 }" @click.stop="setTimer(10)">10s</button>
            </div>
          </Transition>
        </div>

        <!-- Grid toggle -->
        <button
          class="top-btn"
          :class="{ active: showGrid }"
          @click.stop="toggleGrid"
          aria-label="Grid"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <rect x="3" y="3" width="18" height="18" rx="2" />
            <line x1="9" y1="3" x2="9" y2="21" />
            <line x1="15" y1="3" x2="15" y2="21" />
            <line x1="3" y1="9" x2="21" y2="9" />
            <line x1="3" y1="15" x2="21" y2="15" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Zoom Slider -->
    <div v-if="zoomSupported && zoomMax > 1" class="zoom-control">
      <span class="zoom-label">{{ zoomLevel.toFixed(1) }}x</span>
      <input
        type="range"
        class="zoom-slider"
        :min="zoomMin"
        :max="zoomMax"
        :step="0.1"
        v-model.number="zoomLevel"
        @input="setZoom"
      />
    </div>

    <!-- Mode Switcher -->
    <div class="mode-switcher">
      <button
        class="mode-btn"
        :class="{ active: captureMode === 'photo' }"
        @click.stop="captureMode = 'photo'"
      >
        PHOTO
      </button>
      <button
        class="mode-btn"
        :class="{ active: captureMode === 'video' }"
        @click.stop="captureMode = 'video'"
      >
        VIDEO
      </button>
    </div>

    <!-- Filter Picker -->
    <div class="filter-picker">
      <button
        v-for="(filter, i) in filters"
        :key="filter.name"
        class="filter-chip"
        :class="{ active: activeFilterIndex === i }"
        @click.stop="activeFilterIndex = i"
      >
        {{ filter.name }}
      </button>
    </div>

    <!-- Bottom Controls -->
    <div class="bottom-controls">
      <!-- Gallery thumbnail -->
      <button
        class="gallery-thumb"
        :class="{ hasCapture: photos.length > 0 }"
        @click.stop="openGallery"
        aria-label="Gallery"
      >
        <img
          v-if="photos.length > 0 && photos[0].type === 'photo'"
          :src="photos[0].data"
          alt="Last capture"
        />
        <div v-else-if="photos.length > 0 && photos[0].type === 'video'" class="video-thumb-icon">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M8 5v14l11-7z" />
          </svg>
        </div>
        <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="empty-gallery">
          <rect x="3" y="3" width="18" height="18" rx="2" />
          <circle cx="8.5" cy="8.5" r="1.5" />
          <path d="M21 15l-5-5L5 21" />
        </svg>
      </button>

      <!-- Shutter Button -->
      <button
        class="shutter-btn"
        :class="{ recording: isRecording, videoMode: captureMode === 'video' }"
        @click.stop="captureMode === 'photo' ? capturePhoto() : toggleRecording()"
        aria-label="Capture"
      >
        <div class="shutter-inner"></div>
      </button>

      <!-- Switch Camera -->
      <button
        class="switch-camera-btn"
        @click.stop="switchCamera"
        aria-label="Switch camera"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M20 7l-4-4v3H8a4 4 0 0 0 0 8h1" stroke-linecap="round" stroke-linejoin="round" />
          <path d="M4 17l4 4v-3h8a4 4 0 0 0 0-8h-1" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>
    </div>

    <!-- Gallery Overlay -->
    <Transition name="slide-up">
      <div v-if="showGallery" class="gallery-overlay" @click.stop>
        <div class="gallery-header">
          <button class="gallery-close" @click="closeGallery">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M15 18l-6-6 6-6" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          </button>
          <span class="gallery-count">{{ galleryIndex + 1 }} / {{ photos.length }}</span>
          <div class="gallery-actions">
            <button class="gallery-action" @click="downloadCapture(galleryIndex)" aria-label="Download">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" stroke-linecap="round" />
                <polyline points="7 10 12 15 17 10" stroke-linecap="round" stroke-linejoin="round" />
                <line x1="12" y1="15" x2="12" y2="3" stroke-linecap="round" />
              </svg>
            </button>
            <button class="gallery-action delete" @click="deleteCapture(galleryIndex)" aria-label="Delete">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <polyline points="3 6 5 6 21 6" stroke-linecap="round" />
                <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" stroke-linecap="round" />
              </svg>
            </button>
          </div>
        </div>
        <div class="gallery-viewer">
          <button
            class="gallery-nav prev"
            @click="galleryPrev"
            :disabled="galleryIndex === 0"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M15 18l-6-6 6-6" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          </button>
          <div class="gallery-image-container">
            <img
              v-if="photos[galleryIndex]?.type === 'photo'"
              :src="photos[galleryIndex].data"
              class="gallery-image"
              alt="Captured photo"
            />
            <video
              v-else
              :src="photos[galleryIndex]?.data"
              class="gallery-video"
              controls
              playsinline
            ></video>
          </div>
          <button
            class="gallery-nav next"
            @click="galleryNext"
            :disabled="galleryIndex >= photos.length - 1"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M9 18l6-6-6-6" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          </button>
        </div>
      </div>
    </Transition>

    <!-- Hidden canvas for photo capture -->
    <canvas ref="canvasRef" class="capture-canvas"></canvas>
  </div>
</template>

<style scoped>
.camera-screen {
  width: 100%;
  height: 100%;
  position: relative;
  background: #000;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  user-select: none;
  -webkit-user-select: none;
}

/* ─── Status Bar ─────────────────────────────────────────────── */
.camera-status-bar {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  z-index: 30;
  padding: 0 8px;
}

/* ─── Viewfinder ─────────────────────────────────────────────── */
.viewfinder-container {
  flex: 1;
  position: relative;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

.viewfinder {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.viewfinder.mirrored {
  transform: scaleX(-1);
}

/* ─── Grid Overlay ───────────────────────────────────────────── */
.grid-overlay {
  position: absolute;
  inset: 0;
  pointer-events: none;
}

.grid-line {
  position: absolute;
  background: rgba(255, 255, 255, 0.25);
}

.grid-h1 {
  top: 33.33%;
  left: 0;
  right: 0;
  height: 0.5px;
}

.grid-h2 {
  top: 66.66%;
  left: 0;
  right: 0;
  height: 0.5px;
}

.grid-v1 {
  left: 33.33%;
  top: 0;
  bottom: 0;
  width: 0.5px;
}

.grid-v2 {
  left: 66.66%;
  top: 0;
  bottom: 0;
  width: 0.5px;
}

/* ─── Focus Ring ─────────────────────────────────────────────── */
.focus-ring {
  position: absolute;
  width: 60px;
  height: 60px;
  transform: translate(-50%, -50%);
  color: #ffd60a;
  pointer-events: none;
  z-index: 10;
}

.focus-ring svg {
  width: 100%;
  height: 100%;
}

.focus-ring-enter-active {
  transition: all 0.2s ease-out;
}

.focus-ring-leave-active {
  transition: all 0.8s ease-in;
}

.focus-ring-enter-from {
  transform: translate(-50%, -50%) scale(1.8);
  opacity: 0;
}

.focus-ring-enter-to {
  transform: translate(-50%, -50%) scale(1);
  opacity: 1;
}

.focus-ring-leave-to {
  transform: translate(-50%, -50%) scale(0.8);
  opacity: 0;
}

/* ─── Countdown Overlay ──────────────────────────────────────── */
.countdown-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.3);
  z-index: 15;
}

.countdown-number {
  font-size: 96px;
  font-weight: 200;
  color: #fff;
  font-variant-numeric: tabular-nums;
  text-shadow: 0 2px 20px rgba(0, 0, 0, 0.5);
  animation: countdownPulse 1s ease-in-out infinite;
}

@keyframes countdownPulse {
  0%, 100% { transform: scale(1); opacity: 1; }
  50% { transform: scale(1.1); opacity: 0.8; }
}

.countdown-enter-active {
  transition: all 0.3s ease-out;
}

.countdown-leave-active {
  transition: all 0.2s ease-in;
}

.countdown-enter-from {
  opacity: 0;
}

.countdown-leave-to {
  opacity: 0;
}

/* ─── Shutter Flash ──────────────────────────────────────────── */
.shutter-flash {
  position: absolute;
  inset: 0;
  background: #fff;
  z-index: 20;
  pointer-events: none;
}

.flash-enter-active {
  transition: opacity 0.05s ease-out;
}

.flash-leave-active {
  transition: opacity 0.12s ease-in;
}

.flash-enter-from,
.flash-leave-to {
  opacity: 0;
}

/* ─── Recording Indicator ────────────────────────────────────── */
.recording-indicator {
  position: absolute;
  top: 56px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 8px;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  padding: 6px 16px;
  border-radius: 20px;
  z-index: 15;
}

.rec-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: #ff3b30;
  animation: recPulse 1s ease-in-out infinite;
}

@keyframes recPulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

.rec-time {
  font-size: 14px;
  font-weight: 600;
  color: #fff;
  font-variant-numeric: tabular-nums;
  letter-spacing: 0.5px;
}

/* ─── Camera Error ───────────────────────────────────────────── */
.camera-error {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  background: rgba(0, 0, 0, 0.85);
  z-index: 15;
  padding: 40px;
}

.camera-error svg {
  width: 48px;
  height: 48px;
  color: #ff3b30;
}

.camera-error p {
  font-size: 15px;
  color: rgba(255, 255, 255, 0.8);
  text-align: center;
  line-height: 1.5;
  max-width: 280px;
}

/* ─── Top Bar ────────────────────────────────────────────────── */
.top-bar {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: calc(env(safe-area-inset-top, 12px) + 36px) 16px 12px;
  z-index: 25;
  background: linear-gradient(to bottom, rgba(0, 0, 0, 0.4) 0%, transparent 100%);
}

.top-bar-left,
.top-bar-right {
  display: flex;
  gap: 8px;
  align-items: center;
}

.top-btn {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: none;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
}

.top-btn:active {
  transform: scale(0.9);
}

.top-btn.active {
  background: rgba(255, 214, 10, 0.9);
  color: #000;
}

.top-btn svg {
  width: 20px;
  height: 20px;
}

.close-btn {
  background: rgba(0, 0, 0, 0.4);
}

.close-btn svg {
  width: 18px;
  height: 18px;
}

.top-btn-wrapper {
  position: relative;
}

.btn-badge {
  position: absolute;
  bottom: -2px;
  right: -2px;
  background: #ffd60a;
  color: #000;
  font-size: 9px;
  font-weight: 700;
  padding: 1px 4px;
  border-radius: 6px;
  line-height: 1.2;
}

/* ─── Dropdown Menu ──────────────────────────────────────────── */
.dropdown-menu {
  position: absolute;
  top: calc(100% + 8px);
  right: 0;
  background: rgba(30, 30, 30, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 12px;
  padding: 4px;
  min-width: 100px;
  z-index: 30;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
}

.dropdown-menu button {
  display: block;
  width: 100%;
  padding: 10px 16px;
  font-size: 14px;
  color: #fff;
  background: none;
  border: none;
  border-radius: 8px;
  text-align: left;
  cursor: pointer;
  transition: background 0.15s ease;
}

.dropdown-menu button:hover {
  background: rgba(255, 255, 255, 0.1);
}

.dropdown-menu button.selected {
  color: #ffd60a;
}

.dropdown-enter-active {
  transition: all 0.2s ease-out;
}

.dropdown-leave-active {
  transition: all 0.15s ease-in;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.95);
}

/* ─── Zoom Control ───────────────────────────────────────────── */
.zoom-control {
  position: absolute;
  bottom: 240px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 12px;
  z-index: 15;
  padding: 8px 20px;
  background: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border-radius: 20px;
}

.zoom-label {
  font-size: 12px;
  font-weight: 600;
  color: #ffd60a;
  min-width: 32px;
  text-align: center;
  font-variant-numeric: tabular-nums;
}

.zoom-slider {
  width: 120px;
  height: 4px;
  -webkit-appearance: none;
  appearance: none;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 2px;
  outline: none;
}

.zoom-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: #fff;
  cursor: pointer;
  box-shadow: 0 1px 6px rgba(0, 0, 0, 0.3);
}

/* ─── Mode Switcher ──────────────────────────────────────────── */
.mode-switcher {
  display: flex;
  justify-content: center;
  gap: 24px;
  padding: 8px 0 4px;
  z-index: 15;
}

.mode-btn {
  font-size: 13px;
  font-weight: 600;
  letter-spacing: 1px;
  color: rgba(255, 255, 255, 0.5);
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px 8px;
  transition: color 0.2s ease;
  position: relative;
}

.mode-btn.active {
  color: #ffd60a;
}

.mode-btn.active::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: #ffd60a;
}

/* ─── Filter Picker ──────────────────────────────────────────── */
.filter-picker {
  display: flex;
  gap: 8px;
  padding: 6px 20px;
  overflow-x: auto;
  scrollbar-width: none;
  -ms-overflow-style: none;
  z-index: 15;
  justify-content: center;
}

.filter-picker::-webkit-scrollbar {
  display: none;
}

.filter-chip {
  flex-shrink: 0;
  padding: 6px 14px;
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.12);
  border: none;
  color: rgba(255, 255, 255, 0.6);
  font-size: 12px;
  font-weight: 500;
  letter-spacing: 0.3px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.filter-chip:active {
  transform: scale(0.95);
}

.filter-chip.active {
  background: rgba(255, 214, 10, 0.9);
  color: #000;
}

/* ─── Bottom Controls ────────────────────────────────────────── */
.bottom-controls {
  display: flex;
  align-items: center;
  justify-content: space-around;
  padding: 16px 32px calc(env(safe-area-inset-bottom, 12px) + 16px);
  z-index: 15;
  background: linear-gradient(to top, rgba(0, 0, 0, 0.5) 0%, transparent 100%);
}

/* Gallery Thumbnail */
.gallery-thumb {
  width: 50px;
  height: 50px;
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.15);
  border: 2px solid rgba(255, 255, 255, 0.3);
  overflow: hidden;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  padding: 0;
}

.gallery-thumb:active {
  transform: scale(0.9);
}

.gallery-thumb img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.gallery-thumb .empty-gallery {
  width: 24px;
  height: 24px;
  color: rgba(255, 255, 255, 0.5);
}

.video-thumb-icon {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.1);
}

.video-thumb-icon svg {
  width: 24px;
  height: 24px;
  color: rgba(255, 255, 255, 0.7);
}

/* Shutter Button */
.shutter-btn {
  width: 72px;
  height: 72px;
  border-radius: 50%;
  background: none;
  border: 4px solid #fff;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
  padding: 0;
}

.shutter-btn:active {
  transform: scale(0.92);
}

.shutter-inner {
  width: 58px;
  height: 58px;
  border-radius: 50%;
  background: #fff;
  transition: all 0.2s ease;
}

.shutter-btn.videoMode .shutter-inner {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  background: #ff3b30;
}

.shutter-btn.recording .shutter-inner {
  width: 28px;
  height: 28px;
  border-radius: 6px;
  background: #ff3b30;
  animation: recBtnPulse 1s ease-in-out infinite;
}

@keyframes recBtnPulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.6; }
}

/* Switch Camera Button */
.switch-camera-btn {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.15);
  border: none;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.switch-camera-btn:active {
  transform: scale(0.9) rotate(180deg);
}

.switch-camera-btn svg {
  width: 24px;
  height: 24px;
}

/* ─── Gallery Overlay ────────────────────────────────────────── */
.gallery-overlay {
  position: absolute;
  inset: 0;
  background: #000;
  z-index: 40;
  display: flex;
  flex-direction: column;
}

.gallery-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: calc(env(safe-area-inset-top, 12px) + 8px) 16px 12px;
}

.gallery-close {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.gallery-close svg {
  width: 24px;
  height: 24px;
}

.gallery-count {
  font-size: 14px;
  font-weight: 600;
  color: #fff;
}

.gallery-actions {
  display: flex;
  gap: 8px;
}

.gallery-action {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 0.2s ease;
}

.gallery-action:hover {
  background: rgba(255, 255, 255, 0.2);
}

.gallery-action.delete:hover {
  background: rgba(255, 59, 48, 0.3);
}

.gallery-action svg {
  width: 20px;
  height: 20px;
}

.gallery-viewer {
  flex: 1;
  display: flex;
  align-items: center;
  position: relative;
  overflow: hidden;
}

.gallery-image-container {
  flex: 1;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 16px;
}

.gallery-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  border-radius: 8px;
}

.gallery-video {
  max-width: 100%;
  max-height: 100%;
  border-radius: 8px;
}

.gallery-nav {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  width: 44px;
  height: 44px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  z-index: 5;
  transition: background 0.2s ease;
}

.gallery-nav:disabled {
  opacity: 0.3;
  cursor: default;
}

.gallery-nav:not(:disabled):hover {
  background: rgba(255, 255, 255, 0.2);
}

.gallery-nav.prev {
  left: 8px;
}

.gallery-nav.next {
  right: 8px;
}

.gallery-nav svg {
  width: 24px;
  height: 24px;
}

/* ─── Hidden Canvas ──────────────────────────────────────────── */
.capture-canvas {
  display: none;
}

/* ─── Responsive ─────────────────────────────────────────────── */
@media (max-width: 380px) {
  .bottom-controls {
    padding: 12px 24px calc(env(safe-area-inset-bottom, 8px) + 12px);
  }

  .shutter-btn {
    width: 64px;
    height: 64px;
    border-width: 3px;
  }

  .shutter-inner {
    width: 52px;
    height: 52px;
  }

  .zoom-control {
    bottom: 210px;
  }
}

@media (min-width: 768px) {
  .bottom-controls {
    padding: 24px 48px calc(env(safe-area-inset-bottom, 16px) + 24px);
  }

  .shutter-btn {
    width: 80px;
    height: 80px;
  }

  .shutter-inner {
    width: 64px;
    height: 64px;
  }

  .top-btn {
    width: 44px;
    height: 44px;
  }

  .gallery-thumb {
    width: 56px;
    height: 56px;
  }

  .switch-camera-btn {
    width: 56px;
    height: 56px;
  }

  .zoom-control {
    bottom: 280px;
  }
}
</style>
