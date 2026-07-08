<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import * as d3 from 'd3'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import gsap from 'gsap'
import { useScrollReveal } from '../composables/useScrollReveal'

const { t } = useI18n()
const sectionRef = ref<HTMLElement | null>(null)
const mapContainer2d = ref<HTMLElement | null>(null)
const mapContainer3d = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

type Faction = 'havoc' | 'asara' | 'contested' | 'unknown' | 'gti'

interface Location {
  id: string
  x: number
  y: number
  faction: Faction
  labelKey: string
  labelEn: string
  descKey: string
  type: 'city' | 'military' | 'landmark' | 'poi'
  importance: number
}

const locations: Location[] = [
  { id: 'markajesh', x: 50, y: 38, faction: 'contested', labelKey: 'markajesh', labelEn: 'Markajesh', descKey: 'markajesh', type: 'city', importance: 3 },
  { id: 'baksh', x: 28, y: 32, faction: 'havoc', labelKey: 'baksh', labelEn: 'Baksh', descKey: 'baksh', type: 'military', importance: 3 },
  { id: 'zalvatt', x: 65, y: 28, faction: 'unknown', labelKey: 'zalvatt', labelEn: 'Zalvatt', descKey: 'zalvatt', type: 'city', importance: 2 },
  { id: 'zero-dam', x: 42, y: 45, faction: 'asara', labelKey: 'zero-dam', labelEn: 'Zero Dam', descKey: 'zero-dam', type: 'landmark', importance: 3 },
  { id: 'longbow', x: 52, y: 52, faction: 'contested', labelKey: 'longbow', labelEn: 'Longbow Valley', descKey: 'longbow', type: 'landmark', importance: 2 },
  { id: 'space-base', x: 72, y: 62, faction: 'havoc', labelKey: 'space-base', labelEn: 'Space Base', descKey: 'space-base', type: 'military', importance: 3 },
  { id: 'radar', x: 58, y: 56, faction: 'havoc', labelKey: 'radar', labelEn: 'Havoc Radar', descKey: 'radar', type: 'military', importance: 2 },
  { id: 'chaoxing', x: 38, y: 58, faction: 'havoc', labelKey: 'chaoxing', labelEn: 'Chaoxing Station', descKey: 'chaoxing', type: 'military', importance: 2 },
  { id: 'storage', x: 22, y: 48, faction: 'havoc', labelKey: 'storage', labelEn: 'Storage Station', descKey: 'storage', type: 'military', importance: 1 },
  { id: 'tide-prison', x: 84, y: 46, faction: 'havoc', labelKey: 'tide-prison', labelEn: 'Tide Prison', descKey: 'tide-prison', type: 'poi', importance: 2 },
  { id: 'canal', x: 45, y: 65, faction: 'contested', labelKey: 'canal', labelEn: "Umm's Canal", descKey: 'canal', type: 'landmark', importance: 2 },
  { id: 'gti-outpost', x: 35, y: 25, faction: 'gti', labelKey: 'gti-outpost', labelEn: 'GTI Outpost', descKey: 'gti-outpost', type: 'military', importance: 3 },
]

const factionColors: Record<Faction, string> = {
  havoc: '#e8534a',
  asara: '#4a9ee8',
  contested: '#e8a84a',
  unknown: '#5a5650',
  gti: '#c9a84c',
}

const getFactionLabel = (faction: Faction) => t(`situation.factions.${faction}`)
const getLocationLabel = (loc: Location) => t(`situation.locations.${loc.labelKey}`)
const getLocationDesc = (loc: Location) => t(`situation.locationDescs.${loc.descKey}`)
const getLocationTypeLabel = (type: string) => t(`situation.locationTypes.${type}`)

const factionOrder: Faction[] = ['havoc', 'asara', 'contested', 'gti', 'unknown']

const stats = computed(() => {
  const counts: Record<Faction, number> = { havoc: 0, asara: 0, contested: 0, unknown: 0, gti: 0 }
  locations.forEach((l) => counts[l.faction]++)
  return counts
})

const viewMode = ref<'2d' | '3d'>('2d')
const hoveredLocation = ref<Location | null>(null)
const tooltipPos = ref({ x: 0, y: 0 })

let svg: d3.Selection<SVGSVGElement, unknown, null, undefined>
let resizeObserver2d: ResizeObserver | null = null

const generateTerrain = (w: number, h: number): number[][] => {
  const data: number[][] = []
  for (let y = 0; y < h; y++) {
    data[y] = []
    for (let x = 0; x < w; x++) {
      const nx = x / w - 0.5
      const ny = y / h - 0.5
      const dist = Math.sqrt(nx * nx + ny * ny)
      let elevation = Math.max(0, 1 - dist * 1.8)
      elevation += Math.sin(nx * 12) * 0.05 + Math.cos(ny * 8) * 0.04
      elevation += Math.sin(nx * 20 + ny * 15) * 0.03
      const riverY = 0.3 + Math.sin(nx * 6) * 0.15
      const riverDist = Math.abs(ny - riverY)
      if (riverDist < 0.06) elevation *= 0.3
      if (ny > 0.2) elevation += (ny - 0.2) * 0.3
      data[y][x] = Math.max(0, Math.min(1, elevation))
    }
  }
  return data
}

const draw2dMap = () => {
  if (!mapContainer2d.value) return
  const container = mapContainer2d.value
  const rect = container.getBoundingClientRect()
  const width = rect.width || 800
  if (width < 10) return
  const height = Math.max(400, width * 0.65)
  try { d3.select(container).select('svg').remove() } catch (_) { /* ignore */ }

  svg = d3.select(container).append('svg').attr('width', width).attr('height', height).attr('viewBox', `0 0 ${width} ${height}`).style('display', 'block')
  const defs = svg.append('defs')

  const gridSize = width / 20
  const gridPattern = defs.append('pattern').attr('id', 'tacGrid2d').attr('width', gridSize).attr('height', gridSize).attr('patternUnits', 'userSpaceOnUse')
  gridPattern.append('path').attr('d', `M ${gridSize} 0 L 0 0 0 ${gridSize}`).attr('fill', 'none').attr('stroke', 'rgba(201,168,76,0.04)').attr('stroke-width', 0.5)

  const glowFilter = defs.append('filter').attr('id', 'markerGlow2d')
  glowFilter.append('feGaussianBlur').attr('stdDeviation', 3).attr('result', 'blur')
  const glowMerge = glowFilter.append('feMerge')
  glowMerge.append('feMergeNode').attr('in', 'blur')
  glowMerge.append('feMergeNode').attr('in', 'SourceGraphic')

  svg.append('rect').attr('width', width).attr('height', height).attr('fill', '#0a0c10')
  svg.append('rect').attr('width', width).attr('height', height).attr('fill', 'url(#tacGrid2d)')

  const sx = (v: number) => (v / 100) * width
  const sy = (v: number) => (v / 80) * height

  const terrainW = 100, terrainH = 80
  const terrain = generateTerrain(terrainW, terrainH)
  const thresholds = [0.1, 0.2, 0.35, 0.5, 0.65, 0.8]
  const contourColors = ['rgba(74,158,232,0.03)', 'rgba(201,168,76,0.02)', 'rgba(201,168,76,0.035)', 'rgba(201,168,76,0.05)', 'rgba(201,168,76,0.07)', 'rgba(201,168,76,0.09)']
  const contours = d3.contours().size([terrainW, terrainH]).thresholds(thresholds)
  const contourData = contours(terrain.flat())
  const proj = d3.geoIdentity().scale(width / terrainW)
  const pathGen = d3.geoPath().projection(proj)
  svg.append('g').attr('class', 'terrain').selectAll('path').data(contourData).join('path').attr('d', pathGen as any).attr('fill', (_d, i) => contourColors[i] || 'transparent').attr('stroke', 'rgba(201,168,76,0.04)').attr('stroke-width', 0.3)

  const peninsulaPath = 'M10,15 L18,10 L30,8 L42,10 L55,8 L68,10 L78,12 L88,18 L92,28 L90,38 L85,42 L90,45 L92,50 L88,55 L82,52 L78,55 L80,62 L78,68 L72,72 L60,75 L48,73 L38,70 L28,72 L18,68 L12,60 L8,50 L10,42 L8,35 L10,28 L8,22 Z'
  svg.append('path').attr('d', peninsulaPath).attr('fill', 'none').attr('stroke', 'rgba(201,168,76,0.2)').attr('stroke-width', 1).attr('transform', `scale(${width / 100},${height / 80})`)
  svg.append('path').attr('d', 'M80,44 L84,42 L88,44 L86,48 L82,48 Z').attr('fill', 'rgba(201,168,76,0.04)').attr('stroke', 'rgba(201,168,76,0.15)').attr('stroke-width', 0.8).attr('transform', `scale(${width / 100},${height / 80})`)
  svg.append('path').attr('d', 'M38,70 L42,74 L48,76 L55,74 L60,75 L58,78 L50,80 L42,78 L38,74 Z').attr('fill', 'rgba(201,168,76,0.03)').attr('stroke', 'rgba(201,168,76,0.12)').attr('stroke-width', 0.8).attr('transform', `scale(${width / 100},${height / 80})`)

  const zones = [
    { cx: 28, cy: 32, rx: 10, ry: 7, faction: 'havoc' as Faction },
    { cx: 50, cy: 38, rx: 8, ry: 6, faction: 'contested' as Faction },
    { cx: 52, cy: 53, rx: 10, ry: 6, faction: 'contested' as Faction },
  ]
  zones.forEach((zone) => {
    const g = svg.append('g').attr('class', 'zone-overlay')
    g.append('ellipse').attr('cx', sx(zone.cx)).attr('cy', sy(zone.cy)).attr('rx', sx(zone.rx)).attr('ry', sy(zone.ry)).attr('fill', `${factionColors[zone.faction]}08`).attr('stroke', `${factionColors[zone.faction]}18`).attr('stroke-width', 0.8).attr('stroke-dasharray', '4,3')
    g.append('ellipse').attr('cx', sx(zone.cx)).attr('cy', sy(zone.cy)).attr('rx', sx(zone.rx)).attr('ry', sy(zone.ry)).attr('fill', 'none').attr('stroke', `${factionColors[zone.faction]}20`).attr('stroke-width', 1).attr('stroke-dasharray', '2,8').attr('class', 'zone-scan')
  })

  const riverPath = 'M15,30 Q25,32 35,35 Q42,38 42,45 Q42,50 48,54 Q55,58 62,60 Q68,62 75,65'
  const riverGroup = svg.append('g').attr('class', 'river')
  riverGroup.append('path').attr('d', riverPath).attr('fill', 'none').attr('stroke', 'rgba(74,158,232,0.08)').attr('stroke-width', 6).attr('stroke-linecap', 'round').attr('transform', `scale(${width / 100},${height / 80})`)
  riverGroup.append('path').attr('d', riverPath).attr('fill', 'none').attr('stroke', 'rgba(74,158,232,0.2)').attr('stroke-width', 1.5).attr('stroke-linecap', 'round').attr('transform', `scale(${width / 100},${height / 80})`)
  const riverDash = riverGroup.append('path').attr('d', riverPath).attr('fill', 'none').attr('stroke', 'rgba(74,158,232,0.35)').attr('stroke-width', 0.6).attr('stroke-linecap', 'round').attr('stroke-dasharray', '3,12').attr('transform', `scale(${width / 100},${height / 80})`)
  let riverOffset = 0
  const animateRiver = () => { riverOffset -= 0.3; riverDash.attr('stroke-dashoffset', riverOffset); requestAnimationFrame(animateRiver) }
  animateRiver()

  svg.append('path').attr('d', 'M38,62 Q42,65 48,66 Q52,67 58,68').attr('fill', 'none').attr('stroke', 'rgba(74,158,232,0.12)').attr('stroke-width', 0.8).attr('stroke-dasharray', '3,4').attr('stroke-linecap', 'round').attr('transform', `scale(${width / 100},${height / 80})`)

  const damG = svg.append('g').attr('transform', `translate(${sx(39)},${sy(43)}) scale(${width / 100})`)
  damG.append('rect').attr('width', 6).attr('height', 1.5).attr('rx', 0.3).attr('fill', 'rgba(201,168,76,0.2)').attr('stroke', 'rgba(201,168,76,0.4)').attr('stroke-width', 0.15)
  for (let i = 1; i <= 3; i++) damG.append('line').attr('x1', i * 1.5).attr('y1', 0).attr('x2', i * 1.5).attr('y2', 1.5).attr('stroke', 'rgba(201,168,76,0.15)').attr('stroke-width', 0.08)

  svg.append('line').attr('x1', 0).attr('y1', sy(70)).attr('x2', width).attr('y2', sy(70)).attr('stroke', 'rgba(201,168,76,0.08)').attr('stroke-width', 0.8).attr('stroke-dasharray', '6,4')
  svg.append('text').attr('x', 8).attr('y', sy(69.3)).attr('fill', 'rgba(201,168,76,0.15)').attr('font-size', 9).attr('font-family', 'monospace').text('EQUATOR 0°')

  for (let gx = 10; gx < 100; gx += 10) svg.append('text').attr('x', sx(gx)).attr('y', 12).attr('fill', 'rgba(201,168,76,0.1)').attr('font-size', 7).attr('font-family', 'monospace').attr('text-anchor', 'middle').text(String.fromCharCode(64 + gx / 10))
  for (let gy = 10; gy < 80; gy += 10) svg.append('text').attr('x', 4).attr('y', sy(gy) + 3).attr('fill', 'rgba(201,168,76,0.1)').attr('font-size', 7).attr('font-family', 'monospace').text(gy / 10)

  const compass = svg.append('g').attr('transform', `translate(${width - 35}, 35)`).attr('opacity', 0.4)
  compass.append('circle').attr('r', 14).attr('fill', 'none').attr('stroke', 'rgba(201,168,76,0.2)').attr('stroke-width', 0.5)
  compass.append('line').attr('y1', -12).attr('y2', 12).attr('stroke', 'rgba(201,168,76,0.3)').attr('stroke-width', 0.5)
  compass.append('line').attr('x1', -12).attr('x2', 12).attr('stroke', 'rgba(201,168,76,0.3)').attr('stroke-width', 0.5)
  compass.append('polygon').attr('points', '0,-10 -3,-2 3,-2').attr('fill', 'rgba(201,168,76,0.5)')
  compass.append('text').attr('y', -14).attr('text-anchor', 'middle').attr('fill', 'rgba(201,168,76,0.6)').attr('font-size', 7).attr('font-family', 'monospace').text('N')

  const scaleBar = svg.append('g').attr('transform', `translate(20, ${height - 20})`).attr('opacity', 0.4)
  scaleBar.append('line').attr('x2', 50).attr('stroke', 'rgba(201,168,76,0.4)').attr('stroke-width', 1)
  scaleBar.append('line').attr('y1', -3).attr('y2', 3).attr('stroke', 'rgba(201,168,76,0.4)').attr('stroke-width', 0.8)
  scaleBar.append('line').attr('x1', 50).attr('x2', 50).attr('y1', -3).attr('y2', 3).attr('stroke', 'rgba(201,168,76,0.4)').attr('stroke-width', 0.8)
  scaleBar.append('text').attr('x', 25).attr('y', 12).attr('text-anchor', 'middle').attr('fill', 'rgba(201,168,76,0.4)').attr('font-size', 7).attr('font-family', 'monospace').text('50 km')

  svg.append('text').attr('x', width / 2).attr('y', height * 0.06).attr('text-anchor', 'middle').attr('fill', 'rgba(201,168,76,0.08)').attr('font-size', Math.max(16, width * 0.025)).attr('font-family', 'monospace').attr('letter-spacing', 4).text('AHSARAH PENINSULA')
  svg.append('text').attr('transform', `translate(${sx(28)},${sy(35)}) rotate(8)`).attr('fill', 'rgba(74,158,232,0.18)').attr('font-size', 8).attr('font-family', 'monospace').text('UMM RIVER')

  const markersGroup = svg.append('g').attr('class', 'markers')
  locations.forEach((loc) => {
    const cx = sx(loc.x), cy = sy(loc.y)
    const color = factionColors[loc.faction]
    const baseR = loc.importance * 1.5 + 2
    const g = markersGroup.append('g').attr('class', 'marker-group').attr('data-id', loc.id).style('cursor', 'pointer')

    if (loc.faction === 'contested') g.append('circle').attr('cx', cx).attr('cy', cy).attr('r', baseR).attr('fill', 'none').attr('stroke', color).attr('stroke-width', 0.8).attr('opacity', 0).attr('class', 'pulse-ring')
    g.append('circle').attr('cx', cx).attr('cy', cy).attr('r', baseR * 2.5).attr('fill', color).attr('opacity', 0.04).attr('class', 'glow-outer')
    g.append('circle').attr('cx', cx).attr('cy', cy).attr('r', baseR * 1.5).attr('fill', color).attr('opacity', 0.1).attr('class', 'glow-mid')
    g.append('circle').attr('cx', cx).attr('cy', cy).attr('r', baseR * 0.5).attr('fill', color).attr('filter', 'url(#markerGlow2d)').attr('class', 'marker-core')

    if (loc.type === 'military') { const ds = baseR * 0.35; g.append('path').attr('d', `M${cx},${cy - ds} L${cx + ds},${cy} L${cx},${cy + ds} L${cx - ds},${cy} Z`).attr('fill', 'none').attr('stroke', color).attr('stroke-width', 0.6).attr('opacity', 0.6) }
    if (loc.type === 'city') { const ss = baseR * 0.3; g.append('rect').attr('x', cx - ss).attr('y', cy - ss).attr('width', ss * 2).attr('height', ss * 2).attr('fill', 'none').attr('stroke', color).attr('stroke-width', 0.6).attr('opacity', 0.5).attr('transform', `rotate(45, ${cx}, ${cy})`) }
    if (loc.type === 'poi') { const cs = baseR * 0.4; g.append('line').attr('x1', cx - cs).attr('y1', cy).attr('x2', cx + cs).attr('y2', cy).attr('stroke', color).attr('stroke-width', 0.6).attr('opacity', 0.6); g.append('line').attr('x1', cx).attr('y1', cy - cs).attr('x2', cx).attr('y2', cy + cs).attr('stroke', color).attr('stroke-width', 0.6).attr('opacity', 0.6) }

    const labelY = cy - baseR * 2 - 4
    g.append('text').attr('x', cx).attr('y', labelY).attr('text-anchor', 'middle').attr('fill', color).attr('font-size', Math.max(9, width * 0.012)).attr('font-family', 'var(--font-body)').attr('font-weight', 500).attr('opacity', 0.8).text(getLocationLabel(loc))
    g.append('text').attr('x', cx).attr('y', labelY + Math.max(10, width * 0.013)).attr('text-anchor', 'middle').attr('fill', 'rgba(201,168,76,0.3)').attr('font-size', Math.max(6, width * 0.008)).attr('font-family', 'monospace').text(loc.labelEn)

    g.on('mouseenter', function (event: MouseEvent) {
      hoveredLocation.value = loc
      const cr = mapContainer2d.value?.getBoundingClientRect()
      if (cr) tooltipPos.value = { x: event.clientX - cr.left, y: event.clientY - cr.top }
      d3.select(this).select('.glow-outer').transition().duration(300).attr('opacity', 0.12).attr('r', baseR * 3.5)
      d3.select(this).select('.glow-mid').transition().duration(300).attr('opacity', 0.2).attr('r', baseR * 2)
      d3.select(this).select('.marker-core').transition().duration(300).attr('r', baseR * 0.8)
    })
    g.on('mousemove', function (event: MouseEvent) { const cr = mapContainer2d.value?.getBoundingClientRect(); if (cr) tooltipPos.value = { x: event.clientX - cr.left, y: event.clientY - cr.top } })
    g.on('mouseleave', function () {
      hoveredLocation.value = null
      d3.select(this).select('.glow-outer').transition().duration(300).attr('opacity', 0.04).attr('r', baseR * 2.5)
      d3.select(this).select('.glow-mid').transition().duration(300).attr('opacity', 0.1).attr('r', baseR * 1.5)
      d3.select(this).select('.marker-core').transition().duration(300).attr('r', baseR * 0.5)
    })
  })

  svg.selectAll('.pulse-ring').each(function () { const ring = d3.select(this); const sr = parseFloat(ring.attr('r')) || 8; const anim = () => { ring.attr('r', sr).attr('opacity', 0.5).transition().duration(2000).ease(d3.easeLinear).attr('r', sr + 15).attr('opacity', 0).on('end', anim) }; anim() })
  svg.selectAll('.zone-scan').each(function () { const el = d3.select(this); const node = el.node() as SVGEllipseElement | null; const tl = node?.getTotalLength?.() ?? 100; el.attr('stroke-dasharray', `2,${tl - 2}`).attr('stroke-dashoffset', 0); const scan = () => { el.transition().duration(4000).ease(d3.easeLinear).attr('stroke-dashoffset', -tl).on('end', scan) }; scan() })
  const markerEls = container.querySelectorAll('.marker-group')
  if (markerEls.length) gsap.from(markerEls, { opacity: 0, scale: 0, transformOrigin: 'center center', duration: 0.8, stagger: 0.08, ease: 'back.out(1.7)', delay: 0.3 })
}

let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let controls: OrbitControls
let animationId = 0
let markerMeshes: THREE.Mesh[] = []
let labelSprites: THREE.Sprite[] = []
let raycaster: THREE.Raycaster
let mouse: THREE.Vector2
let hoveredMarker: THREE.Object3D | null = null
let threeInitialized = false

const factionColorsThree: Record<Faction, THREE.Color> = {
  havoc: new THREE.Color(0xe8534a), asara: new THREE.Color(0x4a9ee8), contested: new THREE.Color(0xe8a84a), unknown: new THREE.Color(0x5a5650), gti: new THREE.Color(0xc9a84c),
}

const generateTerrainHeight = (ix: number, iz: number, segW: number, segH: number): number => {
  const nx = ix / segW - 0.5, nz = iz / segH - 0.5
  const dist = Math.sqrt(nx * nx + nz * nz)
  let h = Math.max(0, 1 - dist * 1.6)
  h += Math.sin(nx * 10) * 0.06 + Math.cos(nz * 8) * 0.05 + Math.sin(nx * 18 + nz * 14) * 0.04
  const riverZ = 0.05 + Math.sin(nx * 5) * 0.18
  if (Math.abs(nz - riverZ) < 0.08) h *= 0.25
  if (nz > 0.15) h += (nz - 0.15) * 0.5
  return Math.max(0, Math.min(1.2, h))
}

const getTerrainColor = (h: number, ix: number, iz: number, segW: number, segH: number): THREE.Color => {
  const nx = ix / segW - 0.5, nz = iz / segH - 0.5
  const riverZ = 0.05 + Math.sin(nx * 5) * 0.18
  if (Math.abs(nz - riverZ) < 0.06) return new THREE.Color(0x1a3a5c).lerp(new THREE.Color(0x0d2240), Math.random() * 0.3)
  if (h < 0.1) return new THREE.Color(0x1a2a1a)
  if (h < 0.25) return new THREE.Color(0x1e2e1e).lerp(new THREE.Color(0x2a3a2a), (h - 0.1) / 0.15)
  if (h < 0.5) return new THREE.Color(0x2a3a2a).lerp(new THREE.Color(0x3a4a3a), (h - 0.25) / 0.25)
  if (h < 0.8) return new THREE.Color(0x3a4a3a).lerp(new THREE.Color(0x4a5a4a), (h - 0.5) / 0.3)
  return new THREE.Color(0x5a6a5a)
}

const createTextSprite = (text: string, color: string, fontSize = 48): THREE.Sprite => {
  const canvas = document.createElement('canvas')
  const ctx = canvas.getContext('2d')!
  canvas.width = 512; canvas.height = 128
  ctx.clearRect(0, 0, 512, 128)
  ctx.font = `600 ${fontSize}px Inter, sans-serif`
  ctx.textAlign = 'center'; ctx.textBaseline = 'middle'
  ctx.shadowColor = 'rgba(0,0,0,0.8)'; ctx.shadowBlur = 8
  ctx.fillStyle = color; ctx.fillText(text, 256, 64)
  const texture = new THREE.CanvasTexture(canvas)
  texture.minFilter = THREE.LinearFilter
  const material = new THREE.SpriteMaterial({ map: texture, transparent: true, depthTest: false })
  const sprite = new THREE.Sprite(material)
  sprite.scale.set(1.2, 0.3, 1)
  return sprite
}

const locTo3d = (loc: Location): { x: number; z: number } => ({
  x: (loc.x - 50) / 12,
  z: (loc.y - 40) / 12,
})

const createMarker3d = (loc: Location, terrain: THREE.Mesh): THREE.Mesh => {
  const color = factionColorsThree[loc.faction]
  const baseSize = 0.06 + loc.importance * 0.02
  let geometry: THREE.BufferGeometry
  if (loc.type === 'military') geometry = new THREE.OctahedronGeometry(baseSize, 0)
  else if (loc.type === 'city') geometry = new THREE.BoxGeometry(baseSize, baseSize, baseSize)
  else if (loc.type === 'poi') geometry = new THREE.TorusGeometry(baseSize * 0.8, baseSize * 0.25, 8, 12)
  else geometry = new THREE.ConeGeometry(baseSize * 0.6, baseSize * 1.5, 4)

  const material = new THREE.MeshStandardMaterial({ color, emissive: color, emissiveIntensity: 0.5, metalness: 0.3, roughness: 0.6 })
  const mesh = new THREE.Mesh(geometry, material)
  const { x, z } = locTo3d(loc)
  const terrainGeo = terrain.geometry as THREE.PlaneGeometry
  const pos = terrainGeo.attributes.position
  let closestY = 0.1, minDist = Infinity
  for (let i = 0; i < pos.count; i++) {
    const px = pos.getX(i), pz = pos.getZ(i)
    const d = Math.sqrt((px - x) ** 2 + (pz - z) ** 2)
    if (d < minDist) { minDist = d; closestY = pos.getY(i) }
  }
  mesh.position.set(x, closestY + 0.15 + loc.importance * 0.03, z)
  mesh.userData = { location: loc }
  mesh.castShadow = true
  return mesh
}

const init3dScene = () => {
  if (!mapContainer3d.value || threeInitialized) return
  threeInitialized = true
  const containerEl = mapContainer3d.value
  const w = containerEl.clientWidth
  if (w < 10) { threeInitialized = false; return }
  const h = Math.max(500, w * 0.55)

  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x060810)
  scene.fog = new THREE.FogExp2(0x060810, 0.08)
  camera = new THREE.PerspectiveCamera(45, w / h, 0.1, 100)
  camera.position.set(3, 4, 5)
  camera.lookAt(0, 0, 1)

  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setSize(w, h)
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  renderer.shadowMap.enabled = true
  renderer.shadowMap.type = THREE.PCFSoftShadowMap
  renderer.toneMapping = THREE.ACESFilmicToneMapping
  renderer.toneMappingExposure = 0.8
  containerEl.appendChild(renderer.domElement)

  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true; controls.dampingFactor = 0.05
  controls.minDistance = 2; controls.maxDistance = 12
  controls.maxPolarAngle = Math.PI / 2.2; controls.minPolarAngle = 0.2
  controls.target.set(0, 0, 1); controls.update()

  scene.add(new THREE.AmbientLight(0x334455, 0.6))
  const dirLight = new THREE.DirectionalLight(0xffeedd, 0.8)
  dirLight.position.set(5, 8, 3); dirLight.castShadow = true
  dirLight.shadow.mapSize.set(2048, 2048)
  dirLight.shadow.camera.near = 0.1; dirLight.shadow.camera.far = 30
  dirLight.shadow.camera.left = -8; dirLight.shadow.camera.right = 8
  dirLight.shadow.camera.top = 8; dirLight.shadow.camera.bottom = -8
  scene.add(dirLight)
  scene.add(new THREE.PointLight(0xc9a84c, 0.4, 20).translateY(3))
  scene.add(new THREE.HemisphereLight(0x223344, 0x111122, 0.4))

  const segW = 80, segH = 80
  const terrainGeo = new THREE.PlaneGeometry(8, 8, segW, segH)
  terrainGeo.rotateX(-Math.PI / 2)
  const pos = terrainGeo.attributes.position
  const colors = new Float32Array(pos.count * 3)
  for (let i = 0; i < pos.count; i++) {
    const ix = Math.floor(i / (segW + 1)), iz = i % (segW + 1)
    const ht = generateTerrainHeight(ix, iz, segW, segH)
    pos.setY(i, ht * 1.2)
    const c = getTerrainColor(ht, ix, iz, segW, segH)
    colors[i * 3] = c.r; colors[i * 3 + 1] = c.g; colors[i * 3 + 2] = c.b
  }
  terrainGeo.setAttribute('color', new THREE.BufferAttribute(colors, 3))
  terrainGeo.computeVertexNormals()
  const terrain = new THREE.Mesh(terrainGeo, new THREE.MeshStandardMaterial({ vertexColors: true, roughness: 0.85, metalness: 0.05, flatShading: true }))
  terrain.receiveShadow = true; scene.add(terrain)

  const waterGeo = new THREE.PlaneGeometry(10, 10).rotateX(-Math.PI / 2)
  const water = new THREE.Mesh(waterGeo, new THREE.MeshStandardMaterial({ color: 0x0d2240, transparent: true, opacity: 0.7, roughness: 0.2, metalness: 0.4 }))
  water.position.y = -0.02; scene.add(water)

  const grid = new THREE.GridHelper(8, 16, 0x1a2a3a, 0x0d1520)
  grid.position.y = -0.01; (grid.material as THREE.Material).transparent = true; (grid.material as THREE.Material).opacity = 0.15; scene.add(grid)

  const zoneData = [
    { cx: -2.2, cz: 0, rx: 1.2, rz: 0.9, faction: 'havoc' as Faction },
    { cx: 0, cz: 0.4, rx: 1, rz: 0.8, faction: 'contested' as Faction },
    { cx: 0.4, cz: 2, rx: 1.2, rz: 0.8, faction: 'contested' as Faction },
  ]
  zoneData.forEach((zone) => {
    const pts: THREE.Vector3[] = []
    for (let i = 0; i <= 48; i++) { const a = (i / 48) * Math.PI * 2; pts.push(new THREE.Vector3(zone.cx + Math.cos(a) * zone.rx, 0.03, zone.cz + Math.sin(a) * zone.rz)) }
    const zoneGeo = new THREE.BufferGeometry().setFromPoints(pts)
    scene.add(new THREE.LineLoop(zoneGeo, new THREE.LineBasicMaterial({ color: factionColorsThree[zone.faction], transparent: true, opacity: 0.2 })))
  })

  locations.forEach((loc) => {
    const marker = createMarker3d(loc, terrain)
    scene.add(marker); markerMeshes.push(marker)
    const sprite = createTextSprite(getLocationLabel(loc), factionColors[loc.faction])
    sprite.position.copy(marker.position); sprite.position.y += 0.25 + loc.importance * 0.05
    scene.add(sprite); labelSprites.push(sprite)
    if (loc.faction === 'contested') {
      const { x, z } = locTo3d(loc)
      const rGeo = new THREE.RingGeometry(0.15, 0.18, 32)
      const rMat = new THREE.MeshBasicMaterial({ color: factionColorsThree[loc.faction], transparent: true, opacity: 0.3, side: THREE.DoubleSide })
      const ring = new THREE.Mesh(rGeo, rMat); ring.rotation.x = -Math.PI / 2
      const pGeo = terrain.geometry as THREE.PlaneGeometry; const p = pGeo.attributes.position
      let cy = 0, md = Infinity
      for (let i = 0; i < p.count; i++) { const px = p.getX(i), pz = p.getZ(i); const d = Math.sqrt((px - x) ** 2 + (pz - z) ** 2); if (d < md) { md = d; cy = p.getY(i) } }
      ring.position.set(x, cy + 0.02, z); ring.userData = { isPulse: true }; scene.add(ring)
    }
  })

  raycaster = new THREE.Raycaster(); mouse = new THREE.Vector2()
  const handleMouseMove = (event: MouseEvent) => {
    const rect = containerEl.getBoundingClientRect()
    mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1
    mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1
    raycaster.setFromCamera(mouse, camera)
    const intersects = raycaster.intersectObjects(markerMeshes)
    if (intersects.length > 0) {
      const obj = intersects[0].object
      if (hoveredMarker !== obj) {
        if (hoveredMarker) { gsap.to(hoveredMarker.scale, { x: 1, y: 1, z: 1, duration: 0.3 }); gsap.to((hoveredMarker as THREE.Mesh).material as THREE.MeshStandardMaterial, { emissiveIntensity: 0.5, duration: 0.3 }) }
        hoveredMarker = obj
        gsap.to(obj.scale, { x: 1.5, y: 1.5, z: 1.5, duration: 0.3, ease: 'back.out(2)' })
        gsap.to((obj as THREE.Mesh).material as THREE.MeshStandardMaterial, { emissiveIntensity: 1, duration: 0.3 })
      }
      hoveredLocation.value = obj.userData.location
      tooltipPos.value = { x: event.clientX - rect.left, y: event.clientY - rect.top }
    } else {
      if (hoveredMarker) { gsap.to(hoveredMarker.scale, { x: 1, y: 1, z: 1, duration: 0.3 }); gsap.to((hoveredMarker as THREE.Mesh).material as THREE.MeshStandardMaterial, { emissiveIntensity: 0.5, duration: 0.3 }); hoveredMarker = null }
      hoveredLocation.value = null
    }
  }
  containerEl.addEventListener('mousemove', handleMouseMove)

  gsap.from(camera.position, { x: 6, y: 8, z: 10, duration: 2, ease: 'power2.out' })
  markerMeshes.forEach((m, i) => { gsap.from(m.position, { y: m.position.y + 2, duration: 1, delay: 0.5 + i * 0.06, ease: 'back.out(1.4)' }); gsap.from(m.scale, { x: 0, y: 0, z: 0, duration: 0.8, delay: 0.5 + i * 0.06, ease: 'back.out(2)' }) })

  const clock = new THREE.Clock()
  const animate = () => {
    animationId = requestAnimationFrame(animate)
    const t = clock.getElapsedTime()
    markerMeshes.forEach((m, i) => { m.rotation.y = t * 0.5 + i * 0.3; if (m.userData.location?.type === 'poi') m.rotation.x = Math.sin(t * 0.8 + i) * 0.15 })
    scene.children.forEach((child) => { if (child.userData?.isPulse) { const s = 1 + Math.sin(t * 2) * 0.15; child.scale.set(s, s, s); ((child as THREE.Mesh).material as THREE.MeshBasicMaterial).opacity = 0.15 + Math.sin(t * 2) * 0.1 } })
    labelSprites.forEach((s) => s.lookAt(camera.position))
    controls.update(); renderer.render(scene, camera)
  }
  animate()

  const handleResize = () => { const nw = containerEl.clientWidth; const nh = Math.max(500, nw * 0.55); camera.aspect = nw / nh; camera.updateProjectionMatrix(); renderer.setSize(nw, nh) }
  window.addEventListener('resize', handleResize)
}

const switchView = (mode: '2d' | '3d') => {
  viewMode.value = mode
  if (mode === '3d') {
    nextTick(() => {
      setTimeout(() => {
        try { init3dScene() } catch (e) { console.error('3D scene init error:', e) }
      }, 50)
    })
  }
}

onMounted(() => {
  nextTick(() => {
    if (mapContainer2d.value) {
      resizeObserver2d = new ResizeObserver(() => { if (viewMode.value === '2d') draw2dMap() })
      resizeObserver2d.observe(mapContainer2d.value)
      try { draw2dMap() } catch (e) { console.error('2D map init error:', e) }
    }
  })
})

onUnmounted(() => {
  resizeObserver2d?.disconnect()
  cancelAnimationFrame(animationId)
  renderer?.dispose()
  controls?.dispose()
})
</script>

<template>
  <section id="situation" ref="sectionRef" class="section situation">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">{{ t('situation.label') }}</span>
        <h2 class="section-title">{{ t('situation.title') }}</h2>
        <div class="divider"></div>
        <p class="section-subtitle">
          {{ t('situation.subtitle') }}
        </p>
      </div>

      <div class="heatmap-stats reveal">
        <div v-for="faction in factionOrder" :key="faction" class="stat-item" :style="{ '--stat-color': factionColors[faction] }">
          <span class="stat-count">{{ stats[faction] }}</span>
          <span class="stat-label">{{ getFactionLabel(faction) }}</span>
        </div>
      </div>

      <div class="map-wrapper reveal-scale">
        <div class="map-toolbar">
          <span class="toolbar-label">{{ t('situation.toolbar') }}</span>
          <div class="toolbar-center">
            <button class="view-btn" :class="{ active: viewMode === '2d' }" @click="switchView('2d')">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18M9 3v18"/></svg>
              2D
            </button>
            <button class="view-btn" :class="{ active: viewMode === '3d' }" @click="switchView('3d')">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
              3D
            </button>
          </div>
          <span class="toolbar-status"><span class="status-dot"></span>LIVE</span>
        </div>

        <div v-show="viewMode === '2d'" ref="mapContainer2d" class="map-container"></div>
        <div v-show="viewMode === '3d'" ref="mapContainer3d" class="map-container map-container-3d">
          <div v-if="viewMode === '3d'" class="canvas-hint">{{ t('situation.canvasHint') }}</div>
        </div>

        <Transition name="tooltip">
          <div v-if="hoveredLocation" class="map-tooltip" :style="{ left: tooltipPos.x + 'px', top: tooltipPos.y + 'px', '--tooltip-color': factionColors[hoveredLocation.faction] }">
            <div class="tooltip-header">
              <span class="tooltip-dot" :style="{ background: factionColors[hoveredLocation.faction] }"></span>
              <span class="tooltip-faction">{{ getFactionLabel(hoveredLocation.faction) }}</span>
              <span class="tooltip-type">{{ getLocationTypeLabel(hoveredLocation.type) }}</span>
            </div>
            <h4 class="tooltip-title">{{ getLocationLabel(hoveredLocation) }}</h4>
            <span class="tooltip-en">{{ hoveredLocation.labelEn }}</span>
            <p class="tooltip-desc">{{ getLocationDesc(hoveredLocation) }}</p>
          </div>
        </Transition>

        <div class="map-legend">
          <span class="legend-label">{{ t('situation.legend.title') }}</span>
          <div class="legend-items">
            <div v-for="faction in factionOrder" :key="faction" class="legend-item">
              <span class="legend-dot" :style="{ background: factionColors[faction] }"></span>
              <span class="legend-text">{{ getFactionLabel(faction) }}</span>
            </div>
          </div>
          <div class="legend-separator"></div>
          <div class="legend-symbols">
            <span class="symbol-item"><span class="sym-diamond"></span> {{ t('situation.legend.symbols.military') }}</span>
            <span class="symbol-item"><span class="sym-square"></span> {{ t('situation.legend.symbols.city') }}</span>
            <span class="symbol-item"><span class="sym-cross"></span> {{ t('situation.legend.symbols.facility') }}</span>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.situation { background: var(--color-bg-primary); }

.heatmap-stats { display: flex; justify-content: center; gap: var(--space-lg); margin-bottom: var(--space-3xl); flex-wrap: wrap; }
.stat-item { display: flex; flex-direction: column; align-items: center; gap: var(--space-xs); padding: var(--space-sm) var(--space-lg); border: 1px solid var(--color-border); border-radius: var(--radius-md); background: var(--color-bg-card); min-width: 90px; transition: all var(--transition-base); }
.stat-item:hover { border-color: var(--stat-color, var(--color-accent)); box-shadow: 0 0 16px rgba(201,168,76,0.08); }
.stat-count { font-family: var(--font-mono); font-size: 1.8rem; font-weight: 700; color: var(--stat-color, var(--color-accent)); line-height: 1; }
.stat-label { font-size: 0.7rem; color: var(--color-text-muted); letter-spacing: 0.05em; white-space: nowrap; }

.map-wrapper { background: var(--color-bg-card); border: 1px solid var(--color-border); border-radius: var(--radius-lg); overflow: hidden; position: relative; }

.map-toolbar { display: flex; align-items: center; justify-content: space-between; padding: var(--space-sm) var(--space-md); border-bottom: 1px solid var(--color-border); background: rgba(201,168,76,0.02); }
.toolbar-label { font-family: var(--font-mono); font-size: 0.65rem; letter-spacing: 0.15em; color: var(--color-text-muted); }
.toolbar-center { display: flex; gap: 2px; background: rgba(201,168,76,0.05); border-radius: var(--radius-sm); padding: 2px; }
.view-btn { display: flex; align-items: center; gap: var(--space-xs); padding: 6px 14px; background: transparent; border: 1px solid transparent; border-radius: var(--radius-sm); font-family: var(--font-mono); font-size: 0.65rem; letter-spacing: 0.08em; color: var(--color-text-muted); cursor: pointer; transition: all var(--transition-fast); }
.view-btn:hover { color: var(--color-text-primary); }
.view-btn.active { background: rgba(201,168,76,0.12); border-color: var(--color-border); color: var(--color-accent); }
.view-btn svg { opacity: 0.7; }
.view-btn.active svg { opacity: 1; }
.toolbar-status { display: flex; align-items: center; gap: var(--space-xs); font-family: var(--font-mono); font-size: 0.6rem; letter-spacing: 0.1em; color: #4ae87a; }
.status-dot { width: 6px; height: 6px; border-radius: 50%; background: #4ae87a; animation: pulse-glow 2s ease-in-out infinite; }

.map-container { position: relative; width: 100%; min-height: 400px; }
.map-container :deep(svg) { display: block; }
.map-container-3d { min-height: 550px; cursor: grab; }
.map-container-3d:active { cursor: grabbing; }
.map-container-3d :deep(canvas) { display: block; }
.canvas-hint { position: absolute; bottom: var(--space-md); left: 50%; transform: translateX(-50%); font-family: var(--font-mono); font-size: 0.6rem; color: var(--color-text-muted); opacity: 0.4; pointer-events: none; z-index: 5; }

.map-tooltip { position: absolute; pointer-events: none; z-index: 10; background: rgba(10,12,16,0.95); -webkit-backdrop-filter: blur(16px); backdrop-filter: blur(16px); border: 1px solid var(--tooltip-color, var(--color-border)); border-radius: var(--radius-md); padding: var(--space-md); width: max-content; max-width: 260px; transform: translate(20px, -50%); box-shadow: 0 4px 24px rgba(0,0,0,0.6); }
.tooltip-enter-active, .tooltip-leave-active { transition: opacity 0.15s ease; }
.tooltip-enter-from, .tooltip-leave-to { opacity: 0; }
.tooltip-header { display: flex; align-items: center; gap: var(--space-xs); margin-bottom: var(--space-xs); }
.tooltip-dot { width: 6px; height: 6px; border-radius: 50%; flex-shrink: 0; }
.tooltip-faction { font-family: var(--font-mono); font-size: 0.65rem; color: var(--color-text-muted); letter-spacing: 0.1em; }
.tooltip-type { font-family: var(--font-mono); font-size: 0.55rem; color: var(--color-text-muted); margin-left: auto; padding: 1px 6px; border: 1px solid var(--color-border); border-radius: 2px; }
.tooltip-title { font-size: 0.95rem; font-weight: 600; color: var(--color-text-primary); margin-bottom: 2px; }
.tooltip-en { display: block; font-family: var(--font-mono); font-size: 0.65rem; color: var(--color-text-muted); letter-spacing: 0.05em; margin-bottom: var(--space-xs); }
.tooltip-desc { font-size: 0.75rem; color: var(--color-text-secondary); line-height: 1.5; }

.map-legend { display: flex; align-items: center; justify-content: center; gap: var(--space-lg); padding: var(--space-md) var(--space-xl); border-top: 1px solid var(--color-border); background: rgba(201,168,76,0.02); flex-wrap: wrap; }
.legend-label { font-family: var(--font-mono); font-size: 0.65rem; letter-spacing: 0.15em; color: var(--color-text-muted); }
.legend-items { display: flex; gap: var(--space-md); flex-wrap: wrap; }
.legend-item { display: flex; align-items: center; gap: var(--space-xs); }
.legend-dot { width: 8px; height: 8px; border-radius: 50%; flex-shrink: 0; }
.legend-text { font-size: 0.7rem; color: var(--color-text-secondary); }
.legend-separator { width: 1px; height: 16px; background: var(--color-border); }
.legend-symbols { display: flex; gap: var(--space-md); }
.symbol-item { display: flex; align-items: center; gap: var(--space-xs); font-size: 0.65rem; color: var(--color-text-muted); }
.sym-diamond { width: 6px; height: 6px; border: 1px solid var(--color-text-muted); transform: rotate(45deg); }
.sym-square { width: 6px; height: 6px; border: 1px solid var(--color-text-muted); transform: rotate(45deg); }
.sym-cross { width: 8px; height: 8px; position: relative; }
.sym-cross::before, .sym-cross::after { content: ''; position: absolute; background: var(--color-text-muted); }
.sym-cross::before { width: 8px; height: 1px; top: 50%; left: 0; }
.sym-cross::after { width: 1px; height: 8px; left: 50%; top: 0; }

@media (max-width: 768px) {
  .heatmap-stats { gap: var(--space-sm); }
  .stat-item { min-width: 70px; padding: var(--space-sm) var(--space-md); }
  .stat-count { font-size: 1.4rem; }
  .map-tooltip { max-width: 200px; padding: var(--space-sm); }
  .map-legend { flex-direction: column; gap: var(--space-sm); padding: var(--space-sm) var(--space-md); }
  .legend-items { justify-content: center; }
  .legend-separator { width: 40px; height: 1px; }
  .legend-symbols { justify-content: center; }
  .map-container-3d { min-height: 400px; }
  .toolbar-label { display: none; }
}

@media (max-width: 480px) {
  .heatmap-stats { display: grid; grid-template-columns: repeat(3, 1fr); gap: var(--space-xs); }
  .stat-item { min-width: 0; padding: var(--space-xs) var(--space-sm); }
  .stat-count { font-size: 1.2rem; }
  .stat-label { font-size: 0.6rem; }
  .map-container { min-height: 280px; }
  .map-container-3d { min-height: 320px; }
}
</style>
