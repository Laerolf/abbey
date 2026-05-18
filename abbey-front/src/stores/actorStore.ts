import { computed } from 'vue'
import { defineStore } from 'pinia'

import { useGameStore } from '@/stores/gameStore'

import useLogger from '@/composables/useLogger'

import { assign, type MonkDto } from '@/api'

const LOGGER_SCOPE = "@/stores/actorStore.ts"

export const useActorStore = defineStore('actors', () => {
  const monks = computed<MonkDto[]>(() => {
    return useGameStore().monastery?.monks ?? []
  })

  /**
   * Assigns a Process to Actors.
   * @param actorIds - The IDs of the Actors to assign the Process to.
   * @param assignPlayer - Assign the Process to the Player too?
   * @param processId - The ID of the Process to assign.
   */
  async function assignActors(actorIds: number[], assignPlayer: boolean, processId: number): Promise<void> {
    const logger = useLogger(LOGGER_SCOPE)
    const gameStore = useGameStore()

    try {
      const response = await assign({ body: { actor_ids: actorIds, assign_player: assignPlayer, process_id: processId } })

      if (response.error) {
        throw response.error
      }

      const { actors, process } = response.data

      actors.forEach(actor => {
        gameStore.setActor(actor)
      })

      gameStore.setProcess(process)
    } catch (error) {
      logger.error("Failed to assign Monks to a Process", error)
      throw error
    }
  }

  return {
    monks,
    assignActors
  }
})
