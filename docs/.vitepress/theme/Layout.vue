<script setup lang="ts">
import DefaultTheme from 'vitepress/theme'
import { useRoute } from 'vitepress'
import { onMounted, onUnmounted, watch } from 'vue'
import { initHomeHero, revertHomeHero } from './home-hero'

const Layout = DefaultTheme.Layout
const route = useRoute()

function runHomeHero() {
  revertHomeHero()

  const start = (attempt = 0) => {
    if (document.querySelector('.VPHero') || attempt > 20) {
      initHomeHero()
      return
    }
    requestAnimationFrame(() => start(attempt + 1))
  }

  requestAnimationFrame(() => start())
}

onMounted(runHomeHero)
onUnmounted(revertHomeHero)
watch(() => route.path, runHomeHero, { flush: 'post' })
</script>

<template>
  <Layout />
</template>
