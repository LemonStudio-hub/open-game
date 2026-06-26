import { Hono } from 'hono'
import { cors } from 'hono/cors'
import authRoutes from './routes/auth'
import type { Env } from './types'

const app = new Hono<{ Bindings: Env }>()

// CORS
app.use('/api/*', cors({
  origin: (origin, c) => c.env.CORS_ORIGIN || origin,
  allowMethods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
  allowHeaders: ['Content-Type', 'Authorization'],
  maxAge: 86400,
}))

// Health check
app.get('/api/health', (c) => {
  return c.json({ status: 'ok', service: 'scp-latom-node-api', timestamp: new Date().toISOString() })
})

// Auth routes
app.route('/api/auth', authRoutes)

// 404 fallback
app.notFound((c) => {
  return c.json({ success: false, error: 'Not found' }, 404)
})

// Global error handler
app.onError((err, c) => {
  console.error('Unhandled error:', err)
  return c.json({ success: false, error: 'Internal server error' }, 500)
})

export default app
