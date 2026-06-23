<script setup lang="ts">
import { ref } from 'vue';
import { storeToRefs } from 'pinia';

import { useActorStore } from '@/stores/actorStore';

import useLogger from '@/composables/useLogger';
import useCyclicProcess from '@/composables/useCyclicProcess';

import AMonk from '@/components/AMonk.vue';

const props = defineProps<{ cyclicProcessId: number }>()

const logger = useLogger()
const { assignedMonks } = useCyclicProcess(props.cyclicProcessId)

const actorStore = useActorStore()

const { monks } = storeToRefs(actorStore)

const isModelOpen = ref<boolean>()
const selectedMonkIds = ref<number[]>(assignedMonks.value.map(({ id }) => id))

function handleSelectedMonk(id: number): void {
  if (selectedMonkIds.value.includes(id)) {
    selectedMonkIds.value = selectedMonkIds.value.filter(selectedMonkId => selectedMonkId != id)
  } else {
    selectedMonkIds.value.push(id)
  }
}

async function assign(): Promise<void> {
  try {
    await actorStore.assignActors(selectedMonkIds.value, false, props.cyclicProcessId)
  } catch (error) {
    logger.error("Failed to assign Monks to a Process", error)
    throw error
  } finally {
    isModelOpen.value = false
  }
}

function reset(): void {
  selectedMonkIds.value = []
}

console.log({ assigned: assignedMonks.value })
</script>

<template>
  <a-modal @toggled="reset" v-model="isModelOpen" class="a-monk-roster">
    <template #trigger="{ toggle }">
      <a-button @click="toggle">Assign</a-button>
    </template>

    <template #header>
      <h4>Assign</h4>
    </template>

    <a-grid rows class="monks">
      <a-monk :selected="selectedMonkIds.includes(monk.id)" @selected="handleSelectedMonk" option v-for="monk in monks"
        :key="`monk-${monk.id}`" :monk="monk" />
    </a-grid>

    <template #actions>
      <a-button @click="assign">Assign</a-button>
    </template>
  </a-modal>
</template>

<style scoped>
.monks {
  grid-auto-flow: initial;
  grid-template-columns: repeat(4, 1fr);
}
</style>
