import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

import { getSessionGame } from '@/api'

import { useAuthStore } from './authStore'

import useLogger from '@/composables/useLogger'

import type { GameDto, MonasteryDto, PlayerDto } from '@/api'

const LOGGER_SCOPE = '@/stores/gameStore.ts'

export const useGameStore = defineStore('games', () => {


  const logger = useLogger(LOGGER_SCOPE)

  /**
   * The current loaded Game.
   */
  const game = ref<GameDto | null>(null)

  /**
   * The Player of the current Game.
   */
  const player = computed<PlayerDto | undefined>(() => game.value?.player)
  /**
   * The Monastery of the current Game.
   */
  const monastery = computed<MonasteryDto | undefined>(() => game.value?.monastery)

  /**
   * Loads the current session's Game.
   */
  async function loadSessionGame() {
    const { sessionToken } = useAuthStore()

    try {
      if (!sessionToken) {
        throw new Error("You are not authenticated.")
      }

      const response = await getSessionGame({ auth: sessionToken })

      if (response.error) {
        throw response.error
      }

      game.value = response.data
    } catch (error) {
      logger.error('Failed to load the session game', error)
      throw error
    }
  }

  return {
    player,
    monastery,
    loadSessionGame,
  }
})
