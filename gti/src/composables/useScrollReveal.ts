import { onMounted, onUnmounted, type Ref } from 'vue'

interface ScrollRevealOptions extends IntersectionObserverInit {
  stagger?: boolean
  staggerDelay?: number
}

export function useScrollReveal(
  elementRef: Ref<HTMLElement | null>,
  options: ScrollRevealOptions = {}
) {
  let observer: IntersectionObserver | null = null

  const defaultOptions: ScrollRevealOptions = {
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px',
    stagger: false,
    staggerDelay: 80,
    ...options,
  }

  onMounted(() => {
    if (!elementRef.value) return

    observer = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          entry.target.classList.add('visible')

          if (defaultOptions.stagger && entry.target.children.length > 0) {
            Array.from(entry.target.children).forEach((child, index) => {
              const htmlChild = child as HTMLElement
              htmlChild.style.transitionDelay = `${index * (defaultOptions.staggerDelay || 80)}ms`
            })
          }
        }
      })
    }, defaultOptions)

    const revealSelectors = [
      '.reveal',
      '.reveal-left',
      '.reveal-right',
      '.reveal-scale',
      '.reveal-rotate',
      '.reveal-blur',
      '.stagger-children'
    ].join(', ')

    const revealElements = elementRef.value.querySelectorAll(revealSelectors)
    revealElements.forEach((el) => observer!.observe(el))

    if (elementRef.value.matches(revealSelectors)) {
      observer.observe(elementRef.value)
    }
  })

  onUnmounted(() => {
    observer?.disconnect()
  })
}
