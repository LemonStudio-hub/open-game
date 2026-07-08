import { onMounted, onUnmounted, type Ref } from 'vue'

export function useMagnetic(
  elementRef: Ref<HTMLElement | null>,
  options: { strength?: number; radius?: number; ease?: string } = {}
) {
  const { strength = 0.3, radius = 100, ease = 'cubic-bezier(0.16, 1, 0.3, 1)' } = options
  let animationId: number | null = null
  let currentX = 0
  let currentY = 0
  let targetX = 0
  let targetY = 0

  const lerp = (start: number, end: number, factor: number) => {
    return start + (end - start) * factor
  }

  const animate = () => {
    currentX = lerp(currentX, targetX, 0.15)
    currentY = lerp(currentY, targetY, 0.15)

    if (elementRef.value) {
      elementRef.value.style.transform = `translate(${currentX}px, ${currentY}px)`
    }

    if (Math.abs(currentX - targetX) > 0.1 || Math.abs(currentY - targetY) > 0.1) {
      animationId = requestAnimationFrame(animate)
    }
  }

  const handleMouseMove = (e: MouseEvent) => {
    if (!elementRef.value) return

    const rect = elementRef.value.getBoundingClientRect()
    const centerX = rect.left + rect.width / 2
    const centerY = rect.top + rect.height / 2
    const distX = e.clientX - centerX
    const distY = e.clientY - centerY
    const dist = Math.sqrt(distX * distX + distY * distY)

    if (dist < radius) {
      targetX = distX * strength
      targetY = distY * strength

      if (!animationId) {
        animationId = requestAnimationFrame(animate)
      }
    } else {
      targetX = 0
      targetY = 0
    }
  }

  const handleMouseLeave = () => {
    targetX = 0
    targetY = 0
    if (!animationId) {
      animationId = requestAnimationFrame(animate)
    }
  }

  onMounted(() => {
    const el = elementRef.value
    if (!el) return

    el.style.transition = `transform 0.3s ${ease}`
    el.addEventListener('mousemove', handleMouseMove)
    el.addEventListener('mouseleave', handleMouseLeave)
  })

  onUnmounted(() => {
    const el = elementRef.value
    if (el) {
      el.removeEventListener('mousemove', handleMouseMove)
      el.removeEventListener('mouseleave', handleMouseLeave)
    }
    if (animationId) {
      cancelAnimationFrame(animationId)
    }
  })
}
