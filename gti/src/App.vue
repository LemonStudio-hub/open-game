<script setup lang="ts">
import Navbar from './components/Navbar.vue'
import HeroSection from './components/HeroSection.vue'
import AboutSection from './components/AboutSection.vue'
import HistorySection from './components/HistorySection.vue'
import ResolutionSection from './components/ResolutionSection.vue'
import SituationSection from './components/SituationSection.vue'
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
    <ResolutionSection />
    <SituationSection />
    <StructureSection />
    <OperatorsSection />
    <OperationsSection />
    <RelationsSection />
  </main>
  <FooterSection />
</template>

<style>
</style>
