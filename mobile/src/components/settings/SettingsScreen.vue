<script setup lang="ts">
import StatusBar from '../StatusBar.vue'
import { ref, computed } from 'vue'
import { useSwipeGestures } from '../../composables/useSwipeGestures'

const emit = defineEmits<{
  'go-lock': []
  'go-back': []
  'go-home': []
  'show-cards': []
}>()

// --- Swipe gesture ---
const settingsBody = ref<HTMLElement | null>(null)

const canSwipeDown = () => {
  const el = settingsBody.value
  return !el || el.scrollTop <= 0
}

const canSwipeUp = () => {
  const el = settingsBody.value
  return !el || el.scrollTop + el.clientHeight >= el.scrollHeight - 1
}

// pageHistory is declared below; forward-declare navigateBack
let navigateBackFn: () => void

const { targetRef, dragProgress, swipeDirection, isDragging } =
  useSwipeGestures({
    onSwipeDown: () => emit('go-lock'),
    onSwipeUp: () => emit('go-home'),
    onSwipeRight: () => {
      if (navigateBackFn) navigateBackFn()
    },
    canSwipeVertical: () => {
      const dir = swipeDirection.value
      if (dir === 'down') return canSwipeDown()
      if (dir === 'up') return canSwipeUp()
      return true
    },
  })

// --- State ---
const airplaneMode = ref(false)
const wifiEnabled = ref(true)
const bluetoothEnabled = ref(true)
const darkMode = ref(false)
const notifications = ref(true)
const locationServices = ref(true)
const autoBrightness = ref(true)
const brightness = ref(60)
const fontSize = ref<'small' | 'default' | 'large'>('default')

// --- Navigation ---
type Page = 'main' | 'wifi' | 'bluetooth' | 'display' | 'notifications' | 'privacy' | 'about'
const currentPage = ref<Page>('main')
const pageHistory = ref<Page[]>([])
const pageDirection = ref<'forward' | 'back'>('forward')

const navigateTo = (page: Page) => {
  pageDirection.value = 'forward'
  pageHistory.value.push(currentPage.value)
  currentPage.value = page
}

const navigateBack = () => {
  if (pageHistory.value.length > 0) {
    pageDirection.value = 'back'
    currentPage.value = pageHistory.value.pop()!
  } else {
    emit('go-back')
  }
}

navigateBackFn = navigateBack

// --- Search ---
const searchQuery = ref('')

interface SettingsItem {
  label: string
  keywords: string[]
  page: Page
}

const settingsItems: SettingsItem[] = [
  { label: 'Airplane Mode', keywords: ['airplane', 'flight', 'mode'], page: 'main' },
  { label: 'Wi-Fi', keywords: ['wifi', 'wi-fi', 'wireless', 'network', 'internet'], page: 'wifi' },
  { label: 'Bluetooth', keywords: ['bluetooth', 'wireless', 'pair', 'device'], page: 'bluetooth' },
  { label: 'Display & Brightness', keywords: ['display', 'brightness', 'dark mode', 'screen', 'text', 'font', 'true tone'], page: 'display' },
  { label: 'Notifications', keywords: ['notifications', 'alerts', 'messages', 'mail', 'calendar', 'photos', 'banner'], page: 'notifications' },
  { label: 'Privacy & Security', keywords: ['privacy', 'security', 'location', 'face id', 'passcode', 'lock'], page: 'privacy' },
  { label: 'About', keywords: ['about', 'model', 'version', 'storage', 'device', 'info'], page: 'about' },
]

const filteredItems = computed(() => {
  const q = searchQuery.value.toLowerCase().trim()
  if (!q) return []
  return settingsItems.filter(item =>
    item.label.toLowerCase().includes(q) ||
    item.keywords.some(k => k.includes(q))
  )
})

const handleSearchResultClick = (item: SettingsItem) => {
  if (item.page === 'main') {
    airplaneMode.value = !airplaneMode.value
  } else {
    navigateTo(item.page)
  }
  searchQuery.value = ''
}

// --- WiFi ---
const selectedWifi = ref('Home Network')
const wifiNetworks = [
  { name: 'Home Network', secured: true, connected: true, signal: 3 },
  { name: 'Office_5G', secured: true, connected: false, signal: 3 },
  { name: 'Coffee Shop', secured: false, connected: false, signal: 2 },
  { name: 'Neighbor_Network', secured: true, connected: false, signal: 1 },
]

// --- Bluetooth ---
const bluetoothDevices = [
  { name: 'AirPods Pro', connected: true, type: 'headphones' as const },
  { name: 'Car Audio', connected: false, type: 'car' as const },
  { name: 'Smart Watch', connected: true, type: 'watch' as const },
]

// --- About ---
const deviceInfo = {
  name: 'iPhone',
  model: 'Mobile Prototype',
  os: 'iOS 18.0',
  version: '1.0.0',
  storage: '128 GB',
  storageUsed: '47.3 GB',
}
</script>

<template>
  <div
    :ref="targetRef"
    class="settings-screen"
    :class="{ dragging: isDragging }"
  >
    <div class="wallpaper"></div>
    <div
      class="content"
      :style="{
        transform: swipeDirection === 'right'
          ? `translateX(${dragProgress * 40}px)`
          : `translateY(${swipeDirection === 'down' && canSwipeDown() ? dragProgress * 30 : swipeDirection === 'up' && canSwipeUp() ? -dragProgress * 30 : 0}px)`,
        opacity: 1 - (swipeDirection ? dragProgress * 0.2 : 0),
      }"
    >
      <StatusBar />

      <!-- Header -->
      <div class="settings-header">
        <button v-if="currentPage !== 'main'" class="back-btn" @click="navigateBack">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M15 18l-6-6 6-6"/>
          </svg>
          <span>Back</span>
        </button>
        <div class="header-title">
          <h1 v-if="currentPage === 'main'">Settings</h1>
          <h1 v-else-if="currentPage === 'wifi'">Wi-Fi</h1>
          <h1 v-else-if="currentPage === 'bluetooth'">Bluetooth</h1>
          <h1 v-else-if="currentPage === 'display'">Display &amp; Brightness</h1>
          <h1 v-else-if="currentPage === 'notifications'">Notifications</h1>
          <h1 v-else-if="currentPage === 'privacy'">Privacy &amp; Security</h1>
          <h1 v-else-if="currentPage === 'about'">About</h1>
        </div>
      </div>

      <!-- Scrollable content -->
      <div ref="settingsBody" class="settings-body">
        <Transition :name="pageDirection === 'forward' ? 'slide-left' : 'slide-right'" mode="out-in">

          <!-- Main page -->
          <div v-if="currentPage === 'main'" key="main" class="page">
            <!-- Search bar -->
            <div class="search-bar" :class="{ focused: searchQuery.length > 0 }">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="11" cy="11" r="8"/>
                <path d="M21 21l-4.35-4.35"/>
              </svg>
              <input
                type="text"
                class="search-input"
                placeholder="Search"
                v-model="searchQuery"
              />
              <button v-if="searchQuery" class="search-clear" @click="searchQuery = ''">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <circle cx="12" cy="12" r="10" fill="rgba(0,0,0,0.15)"/>
                  <path d="M15.59 7L12 10.59 8.41 7 7 8.41 10.59 12 7 15.59 8.41 17 12 13.41 15.59 17 17 15.59 13.41 12 17 8.41z" fill="white"/>
                </svg>
              </button>
            </div>

            <!-- Search results -->
            <template v-if="searchQuery">
              <div v-if="filteredItems.length > 0" class="settings-group search-results">
                <div
                  v-for="item in filteredItems"
                  :key="item.label"
                  class="settings-row"
                  @click="handleSearchResultClick(item)"
                >
                  <span class="row-label">{{ item.label }}</span>
                  <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
                </div>
              </div>
              <div v-else class="no-results">
                No Results
              </div>
            </template>

            <!-- Normal settings list (hidden during search) -->
            <template v-else>
              <!-- Airplane / Wi-Fi / Bluetooth -->
              <div class="settings-group stagger-1">
                <div class="settings-row" @click="airplaneMode = !airplaneMode">
                  <div class="row-icon airplane-icon" :class="{ active: airplaneMode }">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M22 2L11 13"/>
                      <path d="M22 2l-7 20-4-9-9-4 20-7z"/>
                    </svg>
                  </div>
                  <span class="row-label">Airplane Mode</span>
                  <label class="toggle" @click.stop>
                    <input type="checkbox" v-model="airplaneMode" />
                    <span class="toggle-slider"></span>
                  </label>
                </div>
                <div class="settings-row" @click="navigateTo('wifi')">
                  <div class="row-icon wifi-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M5 12.55a11 11 0 0 1 14.08 0"/>
                      <path d="M1.42 9a16 16 0 0 1 21.16 0"/>
                      <path d="M8.53 16.11a6 6 0 0 1 6.95 0"/>
                      <circle cx="12" cy="20" r="1"/>
                    </svg>
                  </div>
                  <span class="row-label">Wi-Fi</span>
                  <span class="row-value">{{ wifiEnabled ? selectedWifi : 'Off' }}</span>
                  <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
                </div>
                <div class="settings-row" @click="navigateTo('bluetooth')">
                  <div class="row-icon bluetooth-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M6.5 6.5l11 11L12 23V1l5.5 5.5-11 11"/>
                    </svg>
                  </div>
                  <span class="row-label">Bluetooth</span>
                  <span class="row-value">{{ bluetoothEnabled ? 'On' : 'Off' }}</span>
                  <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
                </div>
              </div>

              <!-- Display / Notifications -->
              <div class="settings-group stagger-2">
                <div class="settings-row" @click="navigateTo('display')">
                  <div class="row-icon display-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <circle cx="12" cy="12" r="5"/>
                      <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
                    </svg>
                  </div>
                  <span class="row-label">Display &amp; Brightness</span>
                  <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
                </div>
                <div class="settings-row" @click="navigateTo('notifications')">
                  <div class="row-icon notifications-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
                      <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
                    </svg>
                  </div>
                  <span class="row-label">Notifications</span>
                  <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
                </div>
                <div class="settings-row" @click="navigateTo('privacy')">
                  <div class="row-icon privacy-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                    </svg>
                  </div>
                  <span class="row-label">Privacy &amp; Security</span>
                  <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
                </div>
              </div>

              <!-- About -->
              <div class="settings-group stagger-3">
                <div class="settings-row" @click="navigateTo('about')">
                  <div class="row-icon about-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <circle cx="12" cy="12" r="10"/>
                      <path d="M12 16v-4M12 8h.01"/>
                    </svg>
                  </div>
                  <span class="row-label">About</span>
                  <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
                </div>
              </div>
            </template>
          </div>

          <!-- Wi-Fi page -->
          <div v-else-if="currentPage === 'wifi'" key="wifi" class="page">
            <div class="settings-group">
              <div class="settings-row">
                <span class="row-label">Wi-Fi</span>
                <label class="toggle" @click.stop>
                  <input type="checkbox" v-model="wifiEnabled" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>
            <template v-if="wifiEnabled">
              <div class="group-header">Choose a Network</div>
              <div class="settings-group">
                <div
                  v-for="(network, i) in wifiNetworks"
                  :key="network.name"
                  class="settings-row"
                  :style="{ animationDelay: `${i * 50}ms` }"
                  @click="selectedWifi = network.name"
                >
                  <div class="row-icon-small">
                    <svg viewBox="0 0 24 24" fill="none" :stroke="network.signal >= 2 ? 'currentColor' : '#999'" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path v-if="network.signal >= 3" d="M5 12.55a11 11 0 0 1 14.08 0"/>
                      <path v-if="network.signal >= 2" d="M8.53 16.11a6 6 0 0 1 6.95 0"/>
                      <circle cx="12" cy="20" r="1"/>
                      <path v-if="network.signal >= 1" d="M1.42 9a16 16 0 0 1 21.16 0"/>
                    </svg>
                  </div>
                  <span class="row-label">{{ network.name }}</span>
                  <div class="row-end">
                    <svg v-if="network.secured" class="lock-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                      <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                    </svg>
                    <svg v-if="selectedWifi === network.name" class="checkmark" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6L9 17l-5-5"/></svg>
                    <svg v-else class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
                  </div>
                </div>
              </div>
              <div class="group-header">Other Networks</div>
              <div class="settings-group">
                <div class="settings-row">
                  <span class="row-label" style="color: var(--color-text-secondary)">Ask to Join Networks</span>
                  <label class="toggle" @click.stop>
                    <input type="checkbox" checked disabled />
                    <span class="toggle-slider"></span>
                  </label>
                </div>
              </div>
            </template>
          </div>

          <!-- Bluetooth page -->
          <div v-else-if="currentPage === 'bluetooth'" key="bluetooth" class="page">
            <div class="settings-group">
              <div class="settings-row">
                <span class="row-label">Bluetooth</span>
                <label class="toggle" @click.stop>
                  <input type="checkbox" v-model="bluetoothEnabled" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>
            <template v-if="bluetoothEnabled">
              <div class="group-header">My Devices</div>
              <div class="settings-group">
                <div v-for="(device, i) in bluetoothDevices" :key="device.name" class="settings-row" :style="{ animationDelay: `${i * 60}ms` }">
                  <div class="row-icon-small">
                    <svg v-if="device.type === 'headphones'" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M3 18v-6a9 9 0 0 1 18 0v6"/>
                      <path d="M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z"/>
                    </svg>
                    <svg v-else-if="device.type === 'car'" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M14 16H9m10 0h3v-3.15a1 1 0 0 0-.84-.99L16 11l-2.7-3.6a1 1 0 0 0-.8-.4H5.24a2 2 0 0 0-1.8 1.1l-.8 1.63A6 6 0 0 0 2 12.42V16h3"/>
                      <circle cx="6.5" cy="16.5" r="2.5"/><circle cx="16.5" cy="16.5" r="2.5"/>
                    </svg>
                    <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <circle cx="12" cy="12" r="3"/><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2"/>
                    </svg>
                  </div>
                  <span class="row-label">{{ device.name }}</span>
                  <span class="row-value" :class="{ connected: device.connected }">
                    <span v-if="device.connected" class="connected-dot"></span>
                    {{ device.connected ? 'Connected' : 'Not Connected' }}
                  </span>
                </div>
              </div>
            </template>
          </div>

          <!-- Display & Brightness page -->
          <div v-else-if="currentPage === 'display'" key="display" class="page">
            <div class="settings-group">
              <div class="settings-row">
                <span class="row-label">Dark Mode</span>
                <label class="toggle" @click.stop>
                  <input type="checkbox" v-model="darkMode" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
              <div class="settings-row">
                <span class="row-label">True Tone</span>
                <label class="toggle" @click.stop>
                  <input type="checkbox" checked />
                  <span class="toggle-slider"></span>
                </label>
              </div>
              <div class="settings-row">
                <span class="row-label">Auto-Brightness</span>
                <label class="toggle" @click.stop>
                  <input type="checkbox" v-model="autoBrightness" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>

            <div class="group-header">Brightness</div>
            <div class="settings-group slider-group">
              <div class="slider-row">
                <svg class="slider-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2"/>
                </svg>
                <div class="slider-track">
                  <div class="slider-fill" :style="{ width: brightness + '%' }"></div>
                  <input type="range" min="0" max="100" v-model.number="brightness" class="slider" />
                </div>
                <svg class="slider-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M12 6v12M6 12h12"/>
                </svg>
              </div>
            </div>

            <div class="group-header">Text Size</div>
            <div class="settings-group">
              <div class="font-size-picker">
                <button
                  v-for="size in (['small', 'default', 'large'] as const)"
                  :key="size"
                  class="font-size-btn"
                  :class="{ active: fontSize === size }"
                  @click="fontSize = size"
                >
                  <span :style="{ fontSize: size === 'small' ? '13px' : size === 'large' ? '20px' : '16px' }">Aa</span>
                </button>
              </div>
            </div>
          </div>

          <!-- Notifications page -->
          <div v-else-if="currentPage === 'notifications'" key="notifications" class="page">
            <div class="settings-group">
              <div class="settings-row">
                <span class="row-label">Allow Notifications</span>
                <label class="toggle" @click.stop>
                  <input type="checkbox" v-model="notifications" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>
            <template v-if="notifications">
              <div class="group-header">Notification Style</div>
              <div class="settings-group">
                <div class="settings-row">
                  <span class="row-label">Show Previews</span>
                  <span class="row-value">Always</span>
                  <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
                </div>
                <div class="settings-row">
                  <span class="row-label">Scheduled Summary</span>
                  <label class="toggle" @click.stop>
                    <input type="checkbox" />
                    <span class="toggle-slider"></span>
                  </label>
                </div>
              </div>
              <div class="group-header">App Notifications</div>
              <div class="settings-group">
                <div class="settings-row">
                  <div class="row-icon-small app-badge">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
                    </svg>
                  </div>
                  <span class="row-label">Messages</span>
                  <label class="toggle" @click.stop>
                    <input type="checkbox" checked />
                    <span class="toggle-slider"></span>
                  </label>
                </div>
                <div class="settings-row">
                  <div class="row-icon-small app-badge">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <rect x="2" y="4" width="20" height="16" rx="2"/>
                      <path d="M22 4l-10 8L2 4"/>
                    </svg>
                  </div>
                  <span class="row-label">Mail</span>
                  <label class="toggle" @click.stop>
                    <input type="checkbox" checked />
                    <span class="toggle-slider"></span>
                  </label>
                </div>
                <div class="settings-row">
                  <div class="row-icon-small app-badge">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
                      <path d="M16 2v4M8 2v4M3 10h18"/>
                      <circle cx="12" cy="16" r="1"/>
                    </svg>
                  </div>
                  <span class="row-label">Calendar</span>
                  <label class="toggle" @click.stop>
                    <input type="checkbox" checked />
                    <span class="toggle-slider"></span>
                  </label>
                </div>
                <div class="settings-row">
                  <div class="row-icon-small app-badge">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                      <circle cx="8.5" cy="8.5" r="1.5"/>
                      <path d="M21 15l-5-5L5 21"/>
                    </svg>
                  </div>
                  <span class="row-label">Photos</span>
                  <label class="toggle" @click.stop>
                    <input type="checkbox" />
                    <span class="toggle-slider"></span>
                  </label>
                </div>
              </div>
            </template>
          </div>

          <!-- Privacy & Security page -->
          <div v-else-if="currentPage === 'privacy'" key="privacy" class="page">
            <div class="settings-group">
              <div class="settings-row">
                <span class="row-label">Location Services</span>
                <label class="toggle" @click.stop>
                  <input type="checkbox" v-model="locationServices" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>
            <template v-if="locationServices">
              <div class="group-header">Location Access</div>
              <div class="settings-group">
                <div class="settings-row">
                  <div class="row-icon-small app-badge">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"/>
                      <circle cx="12" cy="10" r="3"/>
                    </svg>
                  </div>
                  <span class="row-label">Maps</span>
                  <span class="row-value">While Using</span>
                  <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
                </div>
                <div class="settings-row">
                  <div class="row-icon-small app-badge">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"/>
                      <circle cx="12" cy="13" r="4"/>
                    </svg>
                  </div>
                  <span class="row-label">Camera</span>
                  <span class="row-value">Never</span>
                  <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
                </div>
              </div>
            </template>
            <div class="group-header">Security</div>
            <div class="settings-group">
              <div class="settings-row">
                <span class="row-label">Face ID</span>
                <span class="row-value">On</span>
                <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
              </div>
              <div class="settings-row">
                <span class="row-label">Passcode</span>
                <span class="row-value">On</span>
                <svg class="row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18l6-6-6-6"/></svg>
              </div>
            </div>
          </div>

          <!-- About page -->
          <div v-else-if="currentPage === 'about'" key="about" class="page">
            <div class="settings-group">
              <div class="settings-row">
                <span class="row-label">Name</span>
                <span class="row-value">{{ deviceInfo.name }}</span>
              </div>
              <div class="settings-row">
                <span class="row-label">Model</span>
                <span class="row-value">{{ deviceInfo.model }}</span>
              </div>
              <div class="settings-row">
                <span class="row-label">OS</span>
                <span class="row-value">{{ deviceInfo.os }}</span>
              </div>
              <div class="settings-row">
                <span class="row-label">Version</span>
                <span class="row-value">{{ deviceInfo.version }}</span>
              </div>
            </div>
            <div class="group-header">Storage</div>
            <div class="settings-group">
              <div class="settings-row column">
                <div class="storage-header">
                  <span class="row-label">Storage</span>
                  <span class="row-value">{{ deviceInfo.storageUsed }} of {{ deviceInfo.storage }} used</span>
                </div>
                <div class="storage-bar">
                  <div class="storage-used" :style="{ width: '37%' }"></div>
                </div>
              </div>
            </div>
          </div>

        </Transition>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* === Page transitions === */
.slide-left-enter-active,
.slide-left-leave-active,
.slide-right-enter-active,
.slide-right-leave-active {
  transition: all 0.28s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.slide-left-enter-from {
  transform: translateX(40%);
  opacity: 0;
}

.slide-left-leave-to {
  transform: translateX(-20%);
  opacity: 0;
}

.slide-right-enter-from {
  transform: translateX(-40%);
  opacity: 0;
}

.slide-right-leave-to {
  transform: translateX(20%);
  opacity: 0;
}

/* === Staggered entry animations === */
@keyframes group-enter {
  from {
    opacity: 0;
    transform: translateY(16px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes row-enter {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.page .stagger-1 { animation: group-enter 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94) both; animation-delay: 0ms; }
.page .stagger-2 { animation: group-enter 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94) both; animation-delay: 80ms; }
.page .stagger-3 { animation: group-enter 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94) both; animation-delay: 160ms; }

.page .settings-group:not(.stagger-1):not(.stagger-2):not(.stagger-3) {
  animation: group-enter 0.35s cubic-bezier(0.25, 0.46, 0.45, 0.94) both;
}

.page .settings-group .settings-row {
  animation: row-enter 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94) both;
}

.page .settings-group .settings-row:nth-child(1) { animation-delay: 40ms; }
.page .settings-group .settings-row:nth-child(2) { animation-delay: 80ms; }
.page .settings-group .settings-row:nth-child(3) { animation-delay: 120ms; }
.page .settings-group .settings-row:nth-child(4) { animation-delay: 160ms; }

.page .group-header {
  animation: group-enter 0.35s cubic-bezier(0.25, 0.46, 0.45, 0.94) both;
  animation-delay: 60ms;
}

/* === Screen === */
.settings-screen {
  width: 100%;
  height: 100%;
  position: relative;
  overflow: hidden;
  user-select: none;
  cursor: grab;
}

.settings-screen.dragging {
  cursor: grabbing;
}

.wallpaper {
  position: absolute;
  inset: 0;
  background: var(--color-bg-warm);
}

.content {
  position: relative;
  z-index: 1;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  transition: transform 0.1s ease-out, opacity 0.1s ease-out;
}

/* === Header === */
.settings-header {
  display: flex;
  align-items: center;
  padding: 8px 16px 12px;
  gap: 4px;
  min-height: 44px;
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  background: rgba(250, 249, 246, 0.85);
  position: relative;
  z-index: 10;
}

.back-btn {
  display: flex;
  align-items: center;
  gap: 2px;
  color: #007aff;
  font-size: 16px;
  font-weight: 400;
  padding: 4px 0;
  cursor: pointer;
  background: none;
  border: none;
  flex-shrink: 0;
  transition: opacity 0.15s ease;
}

.back-btn:active {
  opacity: 0.5;
}

.back-btn svg {
  width: 20px;
  height: 20px;
}

.header-title h1 {
  font-size: 17px;
  font-weight: 600;
  color: var(--color-text);
  white-space: nowrap;
}

/* === Scrollable body === */
.settings-body {
  flex: 1;
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
  padding-bottom: 40px;
}

/* === Search bar === */
.search-bar {
  margin: 0 16px 16px;
  padding: 9px 12px;
  background: rgba(0, 0, 0, 0.04);
  border-radius: 10px;
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--color-text-tertiary);
  font-size: 15px;
  transition: background 0.2s ease, box-shadow 0.2s ease;
}

.search-bar.focused {
  background: rgba(0, 0, 0, 0.06);
  box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.12);
}

.search-bar svg {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  background: none;
  border: none;
  outline: none;
  font-size: 15px;
  color: var(--color-text);
  font-family: inherit;
  padding: 0;
  line-height: 1.4;
}

.search-input::placeholder {
  color: var(--color-text-tertiary);
}

.search-clear {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: opacity 0.15s ease;
}

.search-clear:active {
  opacity: 0.5;
}

.search-clear svg {
  width: 18px;
  height: 18px;
}

.search-results {
  animation: group-enter 0.25s cubic-bezier(0.25, 0.46, 0.45, 0.94) both;
}

.no-results {
  text-align: center;
  padding: 32px 16px;
  color: var(--color-text-secondary);
  font-size: 15px;
}

/* === Groups === */
.settings-group {
  background: white;
  border-radius: 12px;
  margin: 0 16px 24px;
  overflow: hidden;
  box-shadow: 0 0.5px 1px rgba(0, 0, 0, 0.04), 0 2px 8px rgba(0, 0, 0, 0.02);
}

.group-header {
  font-size: 13px;
  font-weight: 400;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.4px;
  padding: 0 0 6px 32px;
  margin-top: 8px;
}

/* === Rows === */
.settings-row {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  gap: 12px;
  cursor: pointer;
  transition: background 0.15s ease, transform 0.1s ease;
  position: relative;
  min-height: 44px;
}

.settings-row:active {
  background: rgba(0, 0, 0, 0.03);
  transform: scale(0.995);
}

.settings-row + .settings-row {
  border-top: 0.5px solid rgba(0, 0, 0, 0.06);
}

.settings-row.column {
  flex-direction: column;
  align-items: stretch;
  gap: 10px;
}

/* === Icons === */
.row-icon {
  width: 30px;
  height: 30px;
  border-radius: 7px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  background: transparent;
  color: #1c1c1e;
  transition: color 0.25s ease, transform 0.2s ease;
}

.row-icon svg {
  width: 18px;
  height: 18px;
}

.airplane-icon.active {
  color: #34c759;
}

.row-icon-small {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  color: var(--color-text-secondary);
}

.row-icon-small svg {
  width: 18px;
  height: 18px;
}

.app-badge svg {
  width: 18px;
  height: 18px;
}

/* === Labels & values === */
.row-label {
  font-size: 16px;
  color: var(--color-text);
  flex: 1;
}

.row-value {
  font-size: 15px;
  color: var(--color-text-secondary);
  display: flex;
  align-items: center;
  gap: 6px;
}

.row-value.connected {
  color: #34c759;
}

.connected-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #34c759;
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(0.85); }
}

.row-chevron {
  width: 16px;
  height: 16px;
  color: var(--color-text-muted);
  flex-shrink: 0;
  transition: transform 0.15s ease;
}

.settings-row:active .row-chevron {
  transform: translateX(2px);
}

.row-end {
  display: flex;
  align-items: center;
  gap: 8px;
}

.lock-icon {
  width: 14px;
  height: 14px;
  color: var(--color-text-muted);
}

.checkmark {
  width: 18px;
  height: 18px;
  color: #007aff;
  flex-shrink: 0;
  animation: check-in 0.25s cubic-bezier(0.34, 1.56, 0.64, 1) both;
}

@keyframes check-in {
  from { opacity: 0; transform: scale(0.5); }
  to { opacity: 1; transform: scale(1); }
}

/* === Toggle === */
.toggle {
  position: relative;
  display: inline-block;
  width: 51px;
  height: 31px;
  flex-shrink: 0;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  inset: 0;
  background: #e9e9ea;
  border-radius: 31px;
  transition: background 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.toggle-slider::before {
  content: '';
  position: absolute;
  width: 27px;
  height: 27px;
  left: 2px;
  bottom: 2px;
  background: white;
  border-radius: 50%;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15), 0 0 1px rgba(0, 0, 0, 0.06);
  transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.toggle input:checked + .toggle-slider {
  background: #34c759;
}

.toggle input:checked + .toggle-slider::before {
  transform: translateX(20px);
}

/* === Slider === */
.slider-group {
  padding: 14px 16px;
}

.slider-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.slider-icon {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
  color: var(--color-text-secondary);
}

.slider-track {
  flex: 1;
  position: relative;
  height: 28px;
  display: flex;
  align-items: center;
}

.slider-fill {
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  height: 4px;
  background: linear-gradient(90deg, #007aff, #5ac8fa);
  border-radius: 2px;
  pointer-events: none;
  transition: width 0.05s linear;
}

.slider {
  width: 100%;
  -webkit-appearance: none;
  appearance: none;
  height: 4px;
  background: #e5e5ea;
  border-radius: 2px;
  outline: none;
  position: relative;
  z-index: 1;
}

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 28px;
  height: 28px;
  background: white;
  border-radius: 50%;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.2), 0 0 1px rgba(0, 0, 0, 0.08);
  cursor: pointer;
  transition: box-shadow 0.2s ease;
}

.slider::-webkit-slider-thumb:active {
  box-shadow: 0 1px 6px rgba(0, 0, 0, 0.25), 0 0 0 8px rgba(0, 122, 255, 0.08);
}

/* === Font size picker === */
.font-size-picker {
  display: flex;
  padding: 12px 16px;
  gap: 0;
}

.font-size-btn {
  flex: 1;
  padding: 10px 0;
  border: 1px solid rgba(0, 0, 0, 0.08);
  background: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text);
  transition: all 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.font-size-btn:first-child {
  border-radius: 8px 0 0 8px;
}

.font-size-btn:last-child {
  border-radius: 0 8px 8px 0;
}

.font-size-btn:not(:first-child) {
  border-left: none;
}

.font-size-btn:active {
  transform: scale(0.97);
}

.font-size-btn.active {
  background: #007aff;
  color: white;
  border-color: #007aff;
  box-shadow: 0 2px 8px rgba(0, 122, 255, 0.25);
}

/* === Storage === */
.storage-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.storage-bar {
  width: 100%;
  height: 6px;
  background: #e5e5ea;
  border-radius: 3px;
  overflow: hidden;
}

.storage-used {
  height: 100%;
  background: linear-gradient(90deg, #007aff, #5ac8fa);
  border-radius: 3px;
  animation: storage-fill 0.8s cubic-bezier(0.25, 0.46, 0.45, 0.94) both;
  animation-delay: 200ms;
}

@keyframes storage-fill {
  from { width: 0% !important; }
}

/* === Responsive === */
@media (min-width: 768px) {
  .settings-group {
    margin: 0 24px 24px;
  }

  .search-bar {
    margin: 0 24px 16px;
  }

  .group-header {
    padding-left: 40px;
  }
}
</style>
