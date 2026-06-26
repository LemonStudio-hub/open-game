import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import i18n from '@/i18n'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('@/views/HomeView.vue'),
    meta: { titleKey: 'nav.home' },
  },
  {
    path: '/catalog',
    name: 'catalog',
    component: () => import('@/views/CatalogView.vue'),
    meta: { titleKey: 'nav.catalog' },
  },
  {
    path: '/entry/:id',
    name: 'entry',
    component: () => import('@/views/EntryView.vue'),
    meta: { titleKey: 'nav.catalog' },
  },
  {
    path: '/documents',
    name: 'documents',
    component: () => import('@/views/DocumentsView.vue'),
    meta: { titleKey: 'nav.documents' },
  },
  {
    path: '/about',
    name: 'about',
    component: () => import('@/views/AboutView.vue'),
    meta: { titleKey: 'nav.about' },
  },
  {
    path: '/login',
    name: 'login',
    component: () => import('@/views/LoginView.vue'),
    meta: { titleKey: 'auth.loginTitle', requiresGuest: true },
  },
  {
    path: '/register',
    name: 'register',
    component: () => import('@/views/RegisterView.vue'),
    meta: { titleKey: 'auth.registerTitle', requiresGuest: true },
  },
  {
    path: '/profile',
    name: 'profile',
    component: () => import('@/views/ProfileView.vue'),
    meta: { titleKey: 'auth.profile', requiresAuth: true },
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'not-found',
    component: () => import('@/views/NotFoundView.vue'),
    meta: { titleKey: 'notFound.accessDenied' },
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior() {
    return { top: 0, behavior: 'smooth' }
  },
})

router.beforeEach((to) => {
  const key = to.meta.titleKey as string
  const t = i18n.global.t
  const title = key ? t(key) : ''
  document.title = title ? `${title} — ${t('site.title')}` : t('site.title')
})

export default router
