import { defineStore } from 'pinia'

import { useGameStore } from '@/stores/gameStore'

import useLogger from '@/composables/useLogger'

import { start, type CyclicProcessDto } from '@/api'
import { computed } from 'vue'

const LOG_SCOPE = '@/stores/processStore.ts'

export const useProcessStore = defineStore('processes', () => {
  const logger = useLogger(LOG_SCOPE)

  const cyclicProcesses = computed<CyclicProcessDto[]>(() => {
    const surroundings = useGameStore().surroundings

    if (!surroundings) {
      return []
    }

    return surroundings.sources.map(({ process }) => process)
  })

  /**
   * Starts a Cyclic Process with the provided ID.
   * @param processId - The ID of the Cyclic Process to start.
   */
  async function startCyclicProcess(processId: number): Promise<void> {
    const gameStore = useGameStore()

    try {
      const response = await start({ path: { process_id: processId } })

      if (response.error) {
        throw response.error
      }

      gameStore.setProcess(response.data)
    } catch (error) {
      logger.error('Failed to start a Cyclic Process', error)
      throw error
    }
  }

  return {
    cyclicProcesses,
    startCyclicProcess,
  }
})
