import { ref, onMounted, onUnmounted } from 'vue'

export function useCursorFollower() {
  const cursorX = ref(0)
  const cursorY = ref(0)
  const isVisible = ref(false)
  const isHovering = ref(false)

  let targetX = 0
  let targetY = 0
  let animationId: number | null = null

  const lerp = (start: number, end: number, factor: number) => {
    return start + (end - start) * factor
  }

  const animate = () => {
    cursorX.value = lerp(cursorX.value, targetX, 0.12)
    cursorY.value = lerp(cursorY.value, targetY, 0.12)

    if (Math.abs(cursorX.value - targetX) > 0.1 || Math.abs(cursorY.value - targetY) > 0.1) {
      animationId = requestAnimationFrame(animate)
    }
  }

  const handleMouseMove = (e: MouseEvent) => {
    targetX = e.clientX
    targetY = e.clientY
    isVisible.value = true

    if (!animationId) {
      animationId = requestAnimationFrame(animate)
    }

    const target = e.target as HTMLElement
    const isInteractive = target.closest('a, button, [role="button"], .clickable, .operator-card')
    isHovering.value = !!isInteractive
  }

  const handleMouseLeave = () => {
    isVisible.value = false
  }

  onMounted(() => {
    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseleave', handleMouseLeave)
  })

  onUnmounted(() => {
    document.removeEventListener('mousemove', handleMouseMove)
    document.removeEventListener('mouseleave', handleMouseLeave)
    if (animationId) {
      cancelAnimationFrame(animationId)
    }
  })

  return {
    cursorX,
    cursorY,
    isVisible,
    isHovering
  }
}
