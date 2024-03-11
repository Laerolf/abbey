<script setup lang="ts">
const { translate } = useTranslations("globals")
</script>

<template>
  <a-grid id="main-header">
    <h1 id="main-header-title">
      {{ translate('applicationName') }}
    </h1>
  </a-grid>
</template>

<style scoped>
#main-header {
  padding: 0.25rem 0.5rem;
  border-bottom: 0.25rem solid black;
}

#main-header-title {
  font-size: xx-large;
  margin: 0;
}
</style>
