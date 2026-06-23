<script setup lang="ts">
import useCyclicProcess from '@/composables/useCyclicProcess';

import AProgressBar from '@/components/AProgressBar.vue'

const props = defineProps<{ cyclicProcessId: number }>()

const { progress } = useCyclicProcess(props.cyclicProcessId)
</script>

<template>
  <a-grid v-if="progress" class="a-cyclic-process" rows>
    <a-progress-bar :progress="progress" />
  </a-grid>
</template>
