import gsap from 'gsap'

const TARGETS = [
  '.VPHero .name',
  '.VPHero .text',
  '.VPHero .tagline',
  '.VPFeature .box'
] as const

let timeline: gsap.core.Timeline | null = null

function isHomePage(): boolean {
  return Boolean(document.querySelector('.VPHero'))
}

function prefersReducedMotion(): boolean {
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches
}

export function revertHomeHero(): void {
  timeline?.kill()
  timeline = null
  gsap.killTweensOf(TARGETS.join(', '))
  gsap.set(TARGETS.join(', '), { clearProps: 'all' })
}

export function initHomeHero(): void {
  if (typeof window === 'undefined' || !isHomePage()) return

  revertHomeHero()

  if (prefersReducedMotion()) {
    gsap.set(TARGETS.join(', '), { autoAlpha: 1, clearProps: 'transform,filter' })
    return
  }

  timeline = gsap.timeline()

  timeline
    .fromTo(
      '.VPHero .name',
      { y: 90, scale: 0.35, autoAlpha: 0 },
      { y: 0, scale: 1, autoAlpha: 1, duration: 1.1, ease: 'back.out(1.8)' }
    )
    .fromTo(
      '.VPHero .text',
      { x: -120, autoAlpha: 0 },
      { x: 0, autoAlpha: 1, duration: 0.95, ease: 'power4.out' },
      '-=0.65'
    )
    .fromTo(
      '.VPHero .tagline',
      { y: 24, autoAlpha: 0, filter: 'blur(10px)' },
      {
        y: 0,
        autoAlpha: 1,
        filter: 'blur(0px)',
        duration: 0.85,
        ease: 'power2.out',
        clearProps: 'filter'
      },
      '-=0.5'
    )
    .fromTo(
      '.VPFeature .box',
      { y: 70, scale: 0.75, rotation: -4, autoAlpha: 0 },
      {
        y: 0,
        scale: 1,
        rotation: 0,
        autoAlpha: 1,
        duration: 0.75,
        ease: 'power3.out',
        stagger: 0.12
      },
      '-=0.35'
    )
}
