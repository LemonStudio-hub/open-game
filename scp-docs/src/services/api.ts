const API_BASE = 'https://api.scp.lat'

interface ApiResult<T = unknown> {
  success: boolean
  data?: T
  error?: string
  message?: string
}

async function request<T = unknown>(
  method: string,
  path: string,
  body?: unknown,
  token?: string
): Promise<ApiResult<T>> {
  const headers: Record<string, string> = { 'Content-Type': 'application/json' }
  if (token) headers['Authorization'] = `Bearer ${token}`

  try {
    const res = await fetch(`${API_BASE}${path}`, {
      method,
      headers,
      body: body ? JSON.stringify(body) : undefined,
    })
    return await res.json()
  } catch (err) {
    return { success: false, error: 'Network error. Please try again.' }
  }
}

export function apiGet<T = unknown>(path: string, token?: string) {
  return request<T>('GET', path, undefined, token)
}

export function apiPost<T = unknown>(path: string, body?: unknown, token?: string) {
  return request<T>('POST', path, body, token)
}

export function apiPut<T = unknown>(path: string, body?: unknown, token?: string) {
  return request<T>('PUT', path, body, token)
}
