import { computed, onMounted, onUnmounted, ref } from "vue";
import { storeToRefs } from "pinia";

import { useProcessStore } from "@/stores/processStore";

import { durationToMilliseconds } from "@/utils/duration";
import type { ActorDto, CyclicProcessDto, MonkDto } from "@/api";
import { isMonkDto } from "@/utils/mapper";

/**
 * A composable handling all CyclicProcess related topics.
 * @param cyclicProcessId - The ID of the CyclicProcess to deal with.
 */
export default function useCyclicProcess(cyclicProcessId: number) {
  const processStore = useProcessStore()

  const now = ref(Date.now())
  let interval: ReturnType<typeof setInterval>

  const { cyclicProcesses } = storeToRefs(processStore)

  const selectedCyclicProcess = computed<CyclicProcessDto | null>(() => cyclicProcesses.value.find(({ id }) => id === cyclicProcessId) ?? null)

  const isInProgress = computed<boolean | null>(() => selectedCyclicProcess.value ? selectedCyclicProcess.value.status === 'InProgress' : null)

  const intervalDurationInMilliseconds = computed<number | null>(() => {
    if (!selectedCyclicProcess.value) {
      return null
    }

    return durationToMilliseconds(selectedCyclicProcess.value.cycle_interval)
  })

  const timeElapsedInMilliseconds = computed<number | null>(() => {
    if (!selectedCyclicProcess.value || !intervalDurationInMilliseconds.value) {
      return null
    }

    const baseElapsed = durationToMilliseconds(selectedCyclicProcess.value.elapsed)

    if (isInProgress.value && selectedCyclicProcess.value.started_at) {
      const startedAt = new Date(selectedCyclicProcess.value.started_at).getTime()
      const totalElapsed = baseElapsed + (now.value - startedAt)

      return totalElapsed % intervalDurationInMilliseconds.value
    }

    return baseElapsed % intervalDurationInMilliseconds.value
  })

  const progress = computed<number | null>(() => {
    if (!timeElapsedInMilliseconds.value || !intervalDurationInMilliseconds.value) {
      return null
    }

    return (timeElapsedInMilliseconds.value / intervalDurationInMilliseconds.value) * 100
  })

  const assignedActors = computed<ActorDto[]>(() => selectedCyclicProcess.value ? selectedCyclicProcess.value.assigned_actors : [])

  const assignedMonks = computed<MonkDto[]>(() => assignedActors.value.filter(isMonkDto) as MonkDto[])

  onMounted(() => {
    interval = setInterval(() => (now.value = Date.now()), 1000)
  })

  onUnmounted(() => clearInterval(interval))

  return {
    isInProgress,
    progress,
    assignedActors,
    assignedMonks
  }
}
