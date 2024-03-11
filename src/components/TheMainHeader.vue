<script setup lang="ts">
const { translate } = useTranslations("globals")
</script>

<template>
  <a-grid>
    <h1 id="main-header-title">
      {{ translate('applicationName') }}
    </h1>
  </a-grid>
</template>

<style scoped>
#main-header-title {
  font-size: xx-large;
  margin: 0;
}
</style>
