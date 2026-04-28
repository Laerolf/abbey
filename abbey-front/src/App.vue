<script setup lang="ts">
import { onBeforeMount } from 'vue';
import { loadLocaleMessages } from './plugins/i18n';

onBeforeMount(async () => {
  await loadLocaleMessages("en");
});
</script>

<template>
  <main>
    <router-view />
  </main>
</template>
