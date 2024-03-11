<script setup lang="ts">
import TheMainHeader from "@/components/TheMainHeader.vue"

import {capitalize} from "@/utils/strings"

const computedNavigationLinks = computed<{key: string, label: string, to: string}[]>(() => (
    useRouter().getRoutes()
        .filter(route => route.meta.$secured)
        .map(route => ({key: route.name, label: route.name, to: route.path}))) as {key: string, label: string, to: string}[]
)

const computedPageTitle = computed<string>(() => useRoute().name as string)
</script>

<template>
  <a-grid
    row-gap="5vh"
    rows
    justify-center
  >
    <the-main-header id="main-header" />

    <a-grid
      id="content"
      rows
      row-gap="2vh"
      align-start
    >
      <a-list :items="computedNavigationLinks">
        <template #default="{item: {to, label}}">
          <nuxt-link :to="to">
            {{ capitalize(label) }}
          </nuxt-link>
        </template>
      </a-list>

      <a-card>
        <template #header>
          <h2 class="no-margin">
            {{ capitalize(computedPageTitle) }}
          </h2>
        </template>

        <nuxt-page />
      </a-card>
    </a-grid>
  </a-grid>
</template>

<style scoped>
#main-header {
  padding: 0.25rem 0.5rem;
  border-bottom: 0.25rem solid black;
}

#content {
  width: 90vw;
  grid-auto-columns: 1fr 4fr;
  grid-template-rows: max-content;
}
</style>
