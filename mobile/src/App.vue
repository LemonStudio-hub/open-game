<script setup lang="ts">
import { ref, computed } from 'vue'
import LockScreen from './components/LockScreen.vue'
import HomeScreen from './components/home/HomeScreen.vue'
import SettingsScreen from './components/settings/SettingsScreen.vue'

type Screen = 'lock' | 'home' | 'settings'
const screenStack = ref<Screen[]>(['lock'])
const currentScreen = computed(() => screenStack.value[screenStack.value.length - 1])

const goToLock = () => {
  screenStack.value = ['lock']
}

const goBack = () => {
  if (screenStack.value.length > 1) {
    screenStack.value = screenStack.value.slice(0, -1)
  }
}

const handleUnlock = () => {
  screenStack.value = [...screenStack.value, 'home']
}

const handleOpenSettings = () => {
  screenStack.value = [...screenStack.value, 'settings']
}
</script>

<template>
  <div class="app-container">
    <Transition name="slide-up">
      <LockScreen
        v-if="currentScreen === 'lock'"
        @unlock="handleUnlock"
      />
      <SettingsScreen
        v-else-if="currentScreen === 'settings'"
        @go-lock="goToLock"
        @go-back="goBack"
      />
      <HomeScreen
        v-else
        @go-lock="goToLock"
        @go-back="goBack"
        @open-settings="handleOpenSettings"
      />
    </Transition>
  </div>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.slide-up-enter-active {
  transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
  position: absolute;
  inset: 0;
}

.slide-up-leave-active {
  transition: all 0.3s ease-in;
  position: absolute;
  inset: 0;
}

.slide-up-enter-from {
  transform: translateY(100%);
  opacity: 0;
}

.slide-up-leave-to {
  transform: translateY(-30%);
  opacity: 0;
}
</style>
