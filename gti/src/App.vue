<script setup lang="ts">
import Navbar from './components/Navbar.vue'
import HeroSection from './components/HeroSection.vue'
import AboutSection from './components/AboutSection.vue'
import HistorySection from './components/HistorySection.vue'
import StructureSection from './components/StructureSection.vue'
import OperatorsSection from './components/OperatorsSection.vue'
import OperationsSection from './components/OperationsSection.vue'
import RelationsSection from './components/RelationsSection.vue'
import FooterSection from './components/FooterSection.vue'
</script>

<template>
  <Navbar />
  <main>
    <HeroSection />
    <AboutSection />
    <HistorySection />
    <StructureSection />
    <OperatorsSection />
    <OperationsSection />
    <RelationsSection />
  </main>
  <FooterSection />
</template>

<style>
/* Remove default style.css imports - using global.css instead */
</style>
