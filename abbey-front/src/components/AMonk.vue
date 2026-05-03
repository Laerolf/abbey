<script setup lang="ts">
import { computed } from 'vue'

import useLocale from '@/composables/useLocale'

import type { MonkDto } from '@/api'

const props = defineProps<{ monk: MonkDto }>()

const { translate } = useLocale('components.aMonk')

const skillIds = computed(() => props.monk.skill_ids)
</script>

<template>
  <a-card class="a-monk">
    <template #header>
      <h4>{{ monk.name }}</h4>
    </template>

    <a-grid class="skills" rows>
      <h5>{{ translate('skills') }}</h5>

      <ul>
        <li v-for="skillId in skillIds" :key="`monk-${monk.id}-skill-${skillId}`">{{ skillId }}</li>
      </ul>
    </a-grid>
  </a-card>
</template>

<style scoped>
.skills {
  row-gap: 0;
}
</style>
