import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { apiPost, apiGet, apiPut } from '@/services/api'

export interface User {
  id: number
  codename: string
  role: string
  clearance: number
  created_at?: string
}

const TOKEN_KEY = 'scp-auth-token'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const token = ref<string>(localStorage.getItem(TOKEN_KEY) || '')
  const loading = ref(false)
  const error = ref('')

  const isAuthenticated = computed(() => !!token.value && !!user.value)

  function setToken(t: string) {
    token.value = t
    localStorage.setItem(TOKEN_KEY, t)
  }

  function clearAuth() {
    user.value = null
    token.value = ''
    localStorage.removeItem(TOKEN_KEY)
  }

  async function register(codename: string, password: string): Promise<boolean> {
    loading.value = true
    error.value = ''
    const res = await apiPost<{ user: User; token: string }>('/api/auth/register', { codename, password })
    loading.value = false
    if (res.success && res.data) {
      user.value = res.data.user
      setToken(res.data.token)
      return true
    }
    error.value = res.error || 'Registration failed'
    return false
  }

  async function login(codename: string, password: string): Promise<boolean> {
    loading.value = true
    error.value = ''
    const res = await apiPost<{ user: User; token: string }>('/api/auth/login', { codename, password })
    loading.value = false
    if (res.success && res.data) {
      user.value = res.data.user
      setToken(res.data.token)
      return true
    }
    error.value = res.error || 'Login failed'
    return false
  }

  async function fetchProfile(): Promise<boolean> {
    if (!token.value) return false
    const res = await apiGet<{ user: User }>('/api/auth/me', token.value)
    if (res.success && res.data) {
      user.value = res.data.user
      return true
    }
    clearAuth()
    return false
  }

  async function updateProfile(data: {
    codename?: string
    password?: string
    newPassword?: string
  }): Promise<boolean> {
    loading.value = true
    error.value = ''
    const res = await apiPut<{ user: User }>('/api/auth/profile', data, token.value)
    loading.value = false
    if (res.success && res.data) {
      user.value = res.data.user
      return true
    }
    error.value = res.error || 'Update failed'
    return false
  }

  function logout() {
    clearAuth()
  }

  async function init() {
    if (token.value) {
      await fetchProfile()
    }
  }

  return {
    user,
    token,
    loading,
    error,
    isAuthenticated,
    register,
    login,
    logout,
    fetchProfile,
    updateProfile,
    init,
  }
})
