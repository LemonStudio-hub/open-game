import { onMounted, onUnmounted, type Ref } from 'vue'

export function useScrollReveal(
  elementRef: Ref<HTMLElement | null>,
  options: IntersectionObserverInit = {}
) {
  let observer: IntersectionObserver | null = null

  const defaultOptions: IntersectionObserverInit = {
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px',
    ...options,
  }

  onMounted(() => {
    if (!elementRef.value) return

    observer = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          entry.target.classList.add('visible')
        }
      })
    }, defaultOptions)

    const revealElements = elementRef.value.querySelectorAll(
      '.reveal, .reveal-left, .reveal-right, .reveal-scale, .stagger-children'
    )

    revealElements.forEach((el) => observer!.observe(el))

    if (
      elementRef.value.classList.contains('reveal') ||
      elementRef.value.classList.contains('reveal-left') ||
      elementRef.value.classList.contains('reveal-right') ||
      elementRef.value.classList.contains('reveal-scale') ||
      elementRef.value.classList.contains('stagger-children')
    ) {
      observer.observe(elementRef.value)
    }
  })

  onUnmounted(() => {
    observer?.disconnect()
  })
}
