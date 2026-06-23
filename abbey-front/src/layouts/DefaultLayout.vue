<script setup lang="ts">
import TheHeader from '@/components/TheHeader.vue'
</script>

<template>
  <a-grid rows class="layout">
    <the-header />

    <a-grid class="content">
      <router-view />
    </a-grid>
  </a-grid>
</template>

<style scoped>
.layout {
  row-gap: var(--space-4);

  .content {
    grid-template-columns: 50vw;
    justify-content: center;
  }
}
</style>
