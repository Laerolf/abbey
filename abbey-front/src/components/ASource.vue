<script setup lang="ts">
import { computed } from 'vue'

import { useProcessStore } from '@/stores/processStore';

import useLocale from '@/composables/useLocale'
import useCyclicProcess from '@/composables/useCyclicProcess';

import ACyclicProcess from '@/components/ACyclicProcess.vue'
import AMonkRoster from '@/components/AMonkRoster.vue'

import type { CyclicProcessDto, SourceDto } from '@/api'

const props = defineProps<{ source: SourceDto }>()

const processStore = useProcessStore()

const { translate, translateInScope } = useLocale("components.aSource")
const { isInProgress } = useCyclicProcess(props.source.process.id)

const cyclicProcess = computed<CyclicProcessDto>(() => props.source.process)

/**
 * Starts the Cyclic Process of this Source.
 */
async function startCyclicProcess(): Promise<void> {
  await processStore.startCyclicProcess(cyclicProcess.value.id)
}
</script>

<template>
  <a-card class="a-source">
    <template #header>
      <h3>{{ translate(`catalog.sources.${source.name}`) }}</h3>
    </template>

    <a-cyclic-process :cyclic-process-id="source.process.id" />

    <template #footer>
      <a-monk-roster :cyclic-process-id="cyclicProcess.id" />

      <a-button @click="startCyclicProcess" v-if="!isInProgress">{{
        translateInScope('actions.start')
        }}</a-button>
      <a-button v-else>{{ translateInScope('actions.pause') }}</a-button>
    </template>
  </a-card>
</template>

<style scoped>
.a-source {
  border-width: var(--border-width-1);
}
</style>
