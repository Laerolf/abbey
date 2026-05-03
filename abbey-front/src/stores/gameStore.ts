import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

import { getSessionGame } from '@/api'

import { useAuthStore } from './authStore'

import useLogger from '@/composables/useLogger'

import type { GameDto, MonasteryDto, PlayerDto } from '@/api'

const LOGGER_SCOPE = '@/stores/gameStore.ts'

export const useGameStore = defineStore('games', () => {
  const { sessionToken } = useAuthStore()

  const logger = useLogger(LOGGER_SCOPE)

  const game = ref<GameDto | undefined>()

  const player = computed<PlayerDto | undefined>(() => game.value?.player)
  const monastery = computed<MonasteryDto | undefined>(() => game.value?.monastery)

  async function loadSessionGame() {
    try {
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
    game,
    player,
    monastery,
    loadSessionGame,
  }
})
