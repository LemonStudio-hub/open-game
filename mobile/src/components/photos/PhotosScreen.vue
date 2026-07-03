<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { usePhotoStore, type Photo } from '../../composables/usePhotoStore'
import { useSwipeGestures } from '../../composables/useSwipeGestures'

const emit = defineEmits<{
  'go-back': []
  'go-home': []
}>()

const { photos, deletePhoto, deletePhotos, toggleFavorite } = usePhotoStore()

// Swipe gestures (disabled when viewer is open to avoid conflict with viewer swipe)
const viewerOpen = ref(false)

const { targetRef, isDragging } =
  useSwipeGestures({
    onSwipeUp: () => emit('go-home'),
    onSwipeRight: () => emit('go-back'),
    canSwipeVertical: () => !viewerOpen.value,
    canSwipeHorizontal: () => !viewerOpen.value,
  })

// ─── Filter Tabs ────────────────────────────────────────────────
type FilterTab = 'all' | 'favorites'
const activeTab = ref<FilterTab>('all')

const filteredPhotos = computed(() => {
  if (activeTab.value === 'favorites') return photos.value.filter(p => p.favorite)
  return photos.value
})

// ─── Multi-Select ───────────────────────────────────────────────
const selectMode = ref(false)
const selectedIds = ref<Set<string>>(new Set())

function toggleSelectMode() {
  selectMode.value = !selectMode.value
  if (!selectMode.value) selectedIds.value.clear()
}

function toggleSelect(id: string) {
  const s = new Set(selectedIds.value)
  if (s.has(id)) s.delete(id)
  else s.add(id)
  selectedIds.value = s
}

function selectAll() {
  selectedIds.value = new Set(filteredPhotos.value.map(p => p.id))
}

function deselectAll() {
  selectedIds.value.clear()
}

function deleteSelected() {
  deletePhotos(Array.from(selectedIds.value))
  selectedIds.value.clear()
  selectMode.value = false
}

function favoriteSelected() {
  for (const id of selectedIds.value) {
    const photo = photos.value.find(p => p.id === id)
    if (photo && !photo.favorite) toggleFavorite(id)
  }
  selectedIds.value.clear()
  selectMode.value = false
}

// ─── Full-Screen Viewer ─────────────────────────────────────────
const viewerIndex = ref(0)

function openViewer(index: number) {
  if (selectMode.value) {
    toggleSelect(filteredPhotos.value[index].id)
    return
  }
  viewerIndex.value = index
  viewerOpen.value = true
}

function closeViewer() {
  viewerOpen.value = false
}

const viewerPhoto = computed(() => filteredPhotos.value[viewerIndex.value])

// Swipe between photos in viewer
const viewerStartX = ref(0)
const viewerCurrentX = ref(0)
const viewerDragging = ref(false)
const viewerDragOffset = ref(0)

function handleViewerTouchStart(e: TouchEvent) {
  viewerStartX.value = e.touches[0].clientX
  viewerCurrentX.value = viewerStartX.value
  viewerDragging.value = true
}

function handleViewerTouchMove(e: TouchEvent) {
  if (!viewerDragging.value) return
  viewerCurrentX.value = e.touches[0].clientX
  viewerDragOffset.value = viewerCurrentX.value - viewerStartX.value
}

function handleViewerTouchEnd() {
  if (!viewerDragging.value) return
  viewerDragging.value = false
  if (viewerDragOffset.value > 60 && viewerIndex.value > 0) {
    viewerIndex.value--
  } else if (viewerDragOffset.value < -60 && viewerIndex.value < filteredPhotos.value.length - 1) {
    viewerIndex.value++
  }
  viewerDragOffset.value = 0
}

function handleViewerMouseDown(e: MouseEvent) {
  viewerStartX.value = e.clientX
  viewerCurrentX.value = viewerStartX.value
  viewerDragging.value = true
}

function handleViewerMouseMove(e: MouseEvent) {
  if (!viewerDragging.value) return
  viewerCurrentX.value = e.clientX
  viewerDragOffset.value = viewerCurrentX.value - viewerStartX.value
}

function handleViewerMouseUp() {
  if (!viewerDragging.value) return
  viewerDragging.value = false
  if (viewerDragOffset.value > 60 && viewerIndex.value > 0) {
    viewerIndex.value--
  } else if (viewerDragOffset.value < -60 && viewerIndex.value < filteredPhotos.value.length - 1) {
    viewerIndex.value++
  }
  viewerDragOffset.value = 0
}

onMounted(() => {
  window.addEventListener('mousemove', handleViewerMouseMove)
  window.addEventListener('mouseup', handleViewerMouseUp)
})

onUnmounted(() => {
  window.removeEventListener('mousemove', handleViewerMouseMove)
  window.removeEventListener('mouseup', handleViewerMouseUp)
})

// ─── Info Panel ─────────────────────────────────────────────────
const showInfo = ref(false)

function toggleInfo() {
  showInfo.value = !showInfo.value
}

function formatTimestamp(ts: number): string {
  return new Date(ts).toLocaleString(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

function estimateSize(data: string): string {
  // Rough estimate: base64 is ~4/3 of binary size
  const bytes = Math.floor((data.length - data.indexOf(',') - 1) * 3 / 4)
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

// ─── Actions ────────────────────────────────────────────────────
async function sharePhoto(photo: Photo) {
  if (navigator.share) {
    try {
      const res = await fetch(photo.data)
      const blob = await res.blob()
      const file = new File([blob], photo.type === 'photo' ? 'photo.jpg' : 'video.webm', {
        type: blob.type,
      })
      await navigator.share({ files: [file] })
    } catch {
      // user cancelled or share failed
    }
  } else {
    // Fallback: copy to clipboard
    try {
      await navigator.clipboard.writeText(photo.data)
      showToast('Copied to clipboard')
    } catch {
      showToast('Unable to share')
    }
  }
}

function deleteCurrent() {
  if (!viewerPhoto.value) return
  deletePhoto(viewerPhoto.value.id)
  if (filteredPhotos.value.length === 0) {
    closeViewer()
  } else if (viewerIndex.value >= filteredPhotos.value.length) {
    viewerIndex.value = filteredPhotos.value.length - 1
  }
}

// ─── Toast ──────────────────────────────────────────────────────
const toastMessage = ref('')
let toastTimeout: ReturnType<typeof setTimeout> | null = null

function showToast(msg: string) {
  toastMessage.value = msg
  if (toastTimeout) clearTimeout(toastTimeout)
  toastTimeout = setTimeout(() => { toastMessage.value = '' }, 2000)
}

// ─── Double-tap zoom ────────────────────────────────────────────
const zoomed = ref(false)
let lastTap = 0

function handleDoubleTap() {
  const now = Date.now()
  if (now - lastTap < 300) {
    zoomed.value = !zoomed.value
  }
  lastTap = now
}

// ─── Close ──────────────────────────────────────────────────────
function handleClose() {
  if (viewerOpen.value) {
    if (showInfo.value) {
      showInfo.value = false
    } else if (zoomed.value) {
      zoomed.value = false
    } else {
      closeViewer()
    }
  } else {
    emit('go-back')
  }
}
</script>

<template>
  <div
    :ref="targetRef"
    class="photos-screen"
    :class="{ dragging: isDragging }"
  >
    <!-- Header -->
    <div class="header">
      <div class="header-top">
        <button class="back-btn" @click="handleClose" aria-label="Back">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M15 18l-6-6 6-6" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </button>
        <h1 class="header-title">{{ selectMode ? `${selectedIds.size} selected` : 'Photos' }}</h1>
        <button
          v-if="!selectMode"
          class="select-btn"
          @click="toggleSelectMode"
          :disabled="photos.length === 0"
        >
          Select
        </button>
        <button v-else class="select-btn cancel" @click="toggleSelectMode">
          Cancel
        </button>
      </div>

      <!-- Filter Tabs -->
      <div v-if="!selectMode" class="tab-bar">
        <button
          class="tab-btn"
          :class="{ active: activeTab === 'all' }"
          @click="activeTab = 'all'"
        >
          All
        </button>
        <button
          class="tab-btn"
          :class="{ active: activeTab === 'favorites' }"
          @click="activeTab = 'favorites'"
        >
          Favorites
        </button>
      </div>

      <!-- Select mode actions -->
      <div v-if="selectMode" class="select-actions">
        <button class="select-action-btn" @click="selectAll">Select All</button>
        <button class="select-action-btn" @click="deselectAll">Deselect</button>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="photos.length === 0" class="empty-state">
      <div class="empty-icon">
        <svg viewBox="0 0 80 80" fill="none" stroke="currentColor" stroke-width="1.5">
          <rect x="8" y="12" width="64" height="52" rx="8" />
          <circle cx="28" cy="32" r="6" />
          <path d="M8 52l18-18 12 12 10-10 24 24" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </div>
      <h2 class="empty-title">No Photos Yet</h2>
      <p class="empty-desc">Photos you take with the camera will appear here.</p>
    </div>

    <!-- Empty Filter State -->
    <div v-else-if="filteredPhotos.length === 0" class="empty-state">
      <div class="empty-icon">
        <svg viewBox="0 0 80 80" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M40 16l5.5 11.2 12.4 1.8-9 8.8 2.1 12.3L40 44.8l-11 5.3 2.1-12.3-9-8.8 12.4-1.8z" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </div>
      <h2 class="empty-title">No Favorites</h2>
      <p class="empty-desc">Tap the heart icon on a photo to add it to your favorites.</p>
    </div>

    <!-- Photo Grid -->
    <div v-else class="photo-grid" :class="{ 'has-bottom-bar': selectMode }">
      <button
        v-for="(photo, i) in filteredPhotos"
        :key="photo.id"
        class="photo-cell"
        :class="{ selected: selectedIds.has(photo.id) }"
        @click="openViewer(i)"
        @dblclick.prevent="openViewer(i)"
      >
        <img
          v-if="photo.type === 'photo'"
          :src="photo.data"
          alt=""
          loading="lazy"
        />
        <div v-else class="video-thumb">
          <img :src="photo.data" alt="" loading="lazy" />
          <div class="video-play-icon">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M8 5v14l11-7z" />
            </svg>
          </div>
        </div>

        <!-- Favorite badge -->
        <div v-if="photo.favorite" class="fav-badge">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
          </svg>
        </div>

        <!-- Select checkmark -->
        <div v-if="selectMode" class="select-check" :class="{ checked: selectedIds.has(photo.id) }">
          <svg v-if="selectedIds.has(photo.id)" viewBox="0 0 24 24" fill="currentColor">
            <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z" />
          </svg>
        </div>
      </button>
    </div>

    <!-- Select Mode Bottom Bar -->
    <Transition name="slide-up">
      <div v-if="selectMode && selectedIds.size > 0" class="select-bottom-bar">
        <button class="bar-action" @click="favoriteSelected" aria-label="Favorite">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
          <span>Favorite</span>
        </button>
        <button class="bar-action danger" @click="deleteSelected" aria-label="Delete">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <polyline points="3 6 5 6 21 6" stroke-linecap="round" />
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" stroke-linecap="round" />
          </svg>
          <span>Delete</span>
        </button>
      </div>
    </Transition>

    <!-- Full-Screen Viewer -->
    <Transition name="viewer">
      <div
        v-if="viewerOpen && viewerPhoto"
        class="viewer"
        @touchstart="handleViewerTouchStart"
        @touchmove="handleViewerTouchMove"
        @touchend="handleViewerTouchEnd"
        @mousedown="handleViewerMouseDown"
        @click="handleDoubleTap"
      >
        <!-- Viewer Header -->
        <div class="viewer-header">
          <button class="viewer-close" @click.stop="closeViewer">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M18 6L6 18M6 6l12 12" stroke-linecap="round" />
            </svg>
          </button>
          <span class="viewer-count">{{ viewerIndex + 1 }} / {{ filteredPhotos.length }}</span>
          <button class="viewer-more" @click.stop="toggleInfo">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <circle cx="12" cy="12" r="1" />
              <circle cx="12" cy="5" r="1" />
              <circle cx="12" cy="19" r="1" />
            </svg>
          </button>
        </div>

        <!-- Image -->
        <div
          class="viewer-image-wrap"
          :style="{ transform: `translateX(${viewerDragOffset}px)`, transition: viewerDragging ? 'none' : 'transform 0.3s ease' }"
        >
          <img
            v-if="viewerPhoto.type === 'photo'"
            :src="viewerPhoto.data"
            class="viewer-image"
            :class="{ zoomed }"
            alt=""
          />
          <video
            v-else
            :src="viewerPhoto.data"
            class="viewer-video"
            controls
            playsinline
          ></video>
        </div>

        <!-- Viewer Bottom Bar -->
        <div class="viewer-bottom">
          <button
            class="viewer-action"
            @click.stop="sharePhoto(viewerPhoto)"
            aria-label="Share"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" stroke-linecap="round" />
              <polyline points="16 6 12 2 8 6" stroke-linecap="round" stroke-linejoin="round" />
              <line x1="12" y1="2" x2="12" y2="15" stroke-linecap="round" />
            </svg>
          </button>
          <button
            class="viewer-action"
            :class="{ favorited: viewerPhoto.favorite }"
            @click.stop="toggleFavorite(viewerPhoto.id)"
            aria-label="Favorite"
          >
            <svg viewBox="0 0 24 24" :fill="viewerPhoto.favorite ? 'currentColor' : 'none'" stroke="currentColor" stroke-width="1.5">
              <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          </button>
          <button
            class="viewer-action danger"
            @click.stop="deleteCurrent"
            aria-label="Delete"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <polyline points="3 6 5 6 21 6" stroke-linecap="round" />
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" stroke-linecap="round" />
            </svg>
          </button>
        </div>

        <!-- Info Panel -->
        <Transition name="slide-up-panel">
          <div v-if="showInfo" class="info-panel" @click.stop>
            <div class="info-handle"></div>
            <h3 class="info-title">Photo Details</h3>
            <div class="info-row">
              <span class="info-label">Type</span>
              <span class="info-value">{{ viewerPhoto.type === 'photo' ? 'Photo' : 'Video' }}</span>
            </div>
            <div class="info-row">
              <span class="info-label">Captured</span>
              <span class="info-value">{{ formatTimestamp(viewerPhoto.timestamp) }}</span>
            </div>
            <div v-if="viewerPhoto.type === 'photo'" class="info-row">
              <span class="info-label">Size</span>
              <span class="info-value">{{ estimateSize(viewerPhoto.data) }}</span>
            </div>
            <div class="info-row">
              <span class="info-label">Favorite</span>
              <span class="info-value">{{ viewerPhoto.favorite ? 'Yes' : 'No' }}</span>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>

    <!-- Toast -->
    <Transition name="toast">
      <div v-if="toastMessage" class="toast">
        {{ toastMessage }}
      </div>
    </Transition>

    <!-- Home Bar -->
    <div class="home-bar-area" v-if="!viewerOpen">
      <div class="home-bar"></div>
    </div>
  </div>
</template>

<style scoped>
.photos-screen {
  width: 100%;
  height: 100%;
  background: var(--color-bg);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  user-select: none;
  -webkit-user-select: none;
}

/* ─── Header ─────────────────────────────────────────────────── */
.header {
  padding: calc(env(safe-area-inset-top, 12px) + 8px) 16px 0;
  background: var(--color-bg);
  z-index: 10;
  flex-shrink: 0;
}

.header-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 44px;
}

.back-btn {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: none;
  border: none;
  color: var(--color-text);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 0.15s ease;
}

.back-btn:active {
  background: var(--color-border);
}

.back-btn svg {
  width: 22px;
  height: 22px;
}

.header-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--color-text);
  letter-spacing: -0.3px;
}

.select-btn {
  font-size: 15px;
  font-weight: 500;
  color: #007aff;
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px 8px;
}

.select-btn:disabled {
  color: var(--color-text-muted);
  cursor: default;
}

.select-btn.cancel {
  color: #ff3b30;
}

/* ─── Tab Bar ────────────────────────────────────────────────── */
.tab-bar {
  display: flex;
  gap: 0;
  padding: 12px 0 8px;
  border-bottom: 1px solid var(--color-border);
}

.tab-btn {
  flex: 1;
  font-size: 13px;
  font-weight: 600;
  letter-spacing: 0.3px;
  color: var(--color-text-tertiary);
  background: none;
  border: none;
  padding: 8px 0;
  cursor: pointer;
  position: relative;
  transition: color 0.2s ease;
}

.tab-btn.active {
  color: var(--color-text);
}

.tab-btn.active::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 24px;
  height: 2px;
  border-radius: 1px;
  background: var(--color-text);
}

/* ─── Select Actions ─────────────────────────────────────────── */
.select-actions {
  display: flex;
  gap: 16px;
  padding: 12px 0 8px;
  border-bottom: 1px solid var(--color-border);
}

.select-action-btn {
  font-size: 13px;
  font-weight: 500;
  color: #007aff;
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px 0;
}

/* ─── Empty State ────────────────────────────────────────────── */
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  gap: 16px;
}

.empty-icon {
  width: 80px;
  height: 80px;
  color: var(--color-text-muted);
}

.empty-icon svg {
  width: 100%;
  height: 100%;
}

.empty-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.empty-desc {
  font-size: 14px;
  color: var(--color-text-tertiary);
  text-align: center;
  line-height: 1.5;
  max-width: 260px;
}

/* ─── Photo Grid ─────────────────────────────────────────────── */
.photo-grid {
  flex: 1;
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 2px;
  padding: 2px;
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
}

.photo-grid.has-bottom-bar {
  padding-bottom: 80px;
}

.photo-cell {
  aspect-ratio: 1;
  overflow: hidden;
  background: var(--color-bg-warm);
  border: none;
  padding: 0;
  cursor: pointer;
  position: relative;
  transition: opacity 0.15s ease;
}

.photo-cell:active {
  opacity: 0.8;
}

.photo-cell img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.video-thumb {
  position: relative;
  width: 100%;
  height: 100%;
}

.video-thumb img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.video-play-icon {
  position: absolute;
  bottom: 6px;
  right: 6px;
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
}

.video-play-icon svg {
  width: 14px;
  height: 14px;
  color: #fff;
}

.fav-badge {
  position: absolute;
  bottom: 6px;
  left: 6px;
  width: 20px;
  height: 20px;
  color: #fff;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.4));
}

.fav-badge svg {
  width: 100%;
  height: 100%;
}

/* Select checkmark */
.select-check {
  position: absolute;
  top: 6px;
  right: 6px;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 2px solid rgba(255, 255, 255, 0.8);
  background: rgba(0, 0, 0, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.select-check.checked {
  background: #007aff;
  border-color: #007aff;
}

.select-check svg {
  width: 16px;
  height: 16px;
  color: #fff;
}

/* ─── Select Bottom Bar ──────────────────────────────────────── */
.select-bottom-bar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  justify-content: center;
  gap: 32px;
  padding: 16px 32px calc(env(safe-area-inset-bottom, 12px) + 16px);
  background: var(--color-bg);
  border-top: 1px solid var(--color-border);
  z-index: 20;
}

.bar-action {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  background: none;
  border: none;
  color: var(--color-text);
  cursor: pointer;
  font-size: 11px;
  font-weight: 500;
}

.bar-action svg {
  width: 24px;
  height: 24px;
}

.bar-action.danger {
  color: #ff3b30;
}

/* ─── Viewer ─────────────────────────────────────────────────── */
.viewer {
  position: fixed;
  inset: 0;
  background: #000;
  z-index: 50;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.viewer-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: calc(env(safe-area-inset-top, 12px) + 8px) 16px 12px;
  z-index: 5;
}

.viewer-close,
.viewer-more {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.12);
  border: none;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.viewer-close svg,
.viewer-more svg {
  width: 22px;
  height: 22px;
}

.viewer-count {
  font-size: 14px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.8);
  font-variant-numeric: tabular-nums;
}

.viewer-image-wrap {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  cursor: grab;
}

.viewer-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  transition: transform 0.3s ease;
}

.viewer-image.zoomed {
  transform: scale(2);
  cursor: zoom-out;
}

.viewer-video {
  max-width: 100%;
  max-height: 100%;
}

.viewer-bottom {
  display: flex;
  justify-content: center;
  gap: 40px;
  padding: 16px 32px calc(env(safe-area-inset-bottom, 12px) + 16px);
  z-index: 5;
}

.viewer-action {
  width: 44px;
  height: 44px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.12);
  border: none;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.15s ease;
}

.viewer-action:active {
  transform: scale(0.9);
}

.viewer-action svg {
  width: 22px;
  height: 22px;
}

.viewer-action.favorited {
  color: #ff3b30;
}

.viewer-action.danger:active {
  background: rgba(255, 59, 48, 0.3);
}

/* ─── Info Panel ─────────────────────────────────────────────── */
.info-panel {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: rgba(20, 20, 20, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 16px 16px 0 0;
  padding: 12px 24px calc(env(safe-area-inset-bottom, 12px) + 24px);
  z-index: 10;
}

.info-handle {
  width: 36px;
  height: 4px;
  border-radius: 2px;
  background: rgba(255, 255, 255, 0.2);
  margin: 0 auto 16px;
}

.info-title {
  font-size: 16px;
  font-weight: 600;
  color: #fff;
  margin-bottom: 16px;
}

.info-row {
  display: flex;
  justify-content: space-between;
  padding: 10px 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.info-row:last-child {
  border-bottom: none;
}

.info-label {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.5);
}

.info-value {
  font-size: 14px;
  color: #fff;
  font-weight: 500;
}

/* ─── Toast ──────────────────────────────────────────────────── */
.toast {
  position: fixed;
  bottom: 100px;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(30, 30, 30, 0.9);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  color: #fff;
  font-size: 14px;
  font-weight: 500;
  padding: 10px 24px;
  border-radius: 20px;
  z-index: 60;
  pointer-events: none;
}

/* ─── Home Bar ───────────────────────────────────────────────── */
.home-bar-area {
  padding: 8px 0 calc(env(safe-area-inset-bottom, 8px) + 4px);
  display: flex;
  justify-content: center;
  flex-shrink: 0;
}

.home-bar {
  width: 134px;
  height: 5px;
  border-radius: 3px;
  background: rgba(0, 0, 0, 0.15);
}

/* ─── Transitions ────────────────────────────────────────────── */
.slide-up-enter-active {
  transition: all 0.3s ease-out;
}

.slide-up-leave-active {
  transition: all 0.2s ease-in;
}

.slide-up-enter-from,
.slide-up-leave-to {
  transform: translateY(100%);
  opacity: 0;
}

.viewer-enter-active {
  transition: all 0.3s ease-out;
}

.viewer-leave-active {
  transition: all 0.25s ease-in;
}

.viewer-enter-from {
  opacity: 0;
  transform: scale(0.95);
}

.viewer-leave-to {
  opacity: 0;
  transform: scale(0.95);
}

.slide-up-panel-enter-active {
  transition: all 0.3s ease-out;
}

.slide-up-panel-leave-active {
  transition: all 0.2s ease-in;
}

.slide-up-panel-enter-from {
  transform: translateY(100%);
}

.slide-up-panel-leave-to {
  transform: translateY(100%);
}

.toast-enter-active {
  transition: all 0.3s ease-out;
}

.toast-leave-active {
  transition: all 0.2s ease-in;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(-50%) translateY(16px);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(-8px);
}

/* ─── Responsive ─────────────────────────────────────────────── */
@media (min-width: 768px) {
  .photo-grid {
    grid-template-columns: repeat(4, 1fr);
    gap: 3px;
    padding: 3px;
  }

  .header-title {
    font-size: 24px;
  }

  .viewer-bottom {
    gap: 48px;
  }
}
</style>
