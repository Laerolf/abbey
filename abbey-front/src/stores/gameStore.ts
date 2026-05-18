import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

import { getSessionGame } from '@/api'

import useLogger from '@/composables/useLogger'

import type {
  ActorDto,
  CyclicProcessDto,
  GameDto,
  MonasteryDto,
  MonkDto,
  PlayerDto,
  ProcessDto,
  SurroundingsDto,
  TaskDto,
} from '@/api'

const LOGGER_SCOPE = '@/stores/gameStore.ts'

export const useGameStore = defineStore('games', () => {
  /**
   * The current loaded Game.
   */
  const game = ref<GameDto | null>(null)

  /**
   * The Player of the current Game.
   */
  const player = computed<PlayerDto | null>(() => game.value?.player ?? null)
  /**
   * The Monastery of the current Game.
   */
  const monastery = computed<MonasteryDto | null>(() => game.value?.monastery ?? null)
  /**
   * The Surroundings of the current Game.
   */
  const surroundings = computed<SurroundingsDto | null>(() => game.value?.surroundings ?? null)

  /**
   * Loads the current session's Game.
   */
  async function loadSessionGame(): Promise<void> {
    const logger = useLogger(LOGGER_SCOPE)

    try {
      const response = await getSessionGame()

      if (response.error) {
        throw response.error
      }

      game.value = response.data
    } catch (error) {
      logger.error('Failed to load the session game', error)
      throw error
    }
  }

  /**
   * Sets a Process in the Game state.
   * @param process - The Process to set in the state
   */
  function setProcess(process: ProcessDto): void {
    switch (process.type) {
      case 'CyclicProcess':
        setCyclicProcess(process)
        break
      case 'Task':
        setTask(process)
        break
    }
  }

  /**
   * Sets a Task in the Game state.
   * @param task - The Task to set in the state
   */
  function setTask(_task: TaskDto): void {
    throw new Error('setTask is not implemented yet.')
  }

  /**
   * Sets a Cyclic Process in the Game state.
   * @param process - The Cyclic Process to set in the state
   */
  function setCyclicProcess(process: CyclicProcessDto): void {
    const logger = useLogger(LOGGER_SCOPE)

    try {
      if (!game.value) {
        throw new Error('The game has not been loaded yet.')
      }

      const selectedSource = game.value.surroundings.sources.find(
        source => source.process.id === process.id,
      )

      if (!selectedSource) {
        throw new Error('The Source was not found.')
      }

      selectedSource.process = process
    } catch (error) {
      logger.error('Failed to set a Cyclic Process', error)
      throw error
    }
  }

  /**
   * Sets an Actor in the Game state.
   * @param actor - The Actor to set in the state
   */
  function setActor(actor: ActorDto): void {
    switch (actor.type) {
      case 'Player':
        setPlayer(actor)
        break
      case 'Monk':
        setMonk(actor)
        break
    }
  }

  /**
   * Sets the Monk in the Game state.
   * @param monk - The Monk to set in the state
   */
  function setMonk(monk: MonkDto): void {
    const logger = useLogger(LOGGER_SCOPE)

    try {
      if (!game.value) {
        throw new Error('The game has not been loaded yet.')
      }

      const monkIndex = game.value.monastery.monks.findIndex(({ id }) => id === monk.id)

      if (monkIndex === -1) {
        throw new Error('The Monk was not found.')
      }

      game.value.monastery.monks[monkIndex] = monk
    } catch (error) {
      logger.error('Failed to set a Monk of the Game', error)
      throw error
    }
  }

  /**
   * Sets the Player in the Game state.
   * @param player - The Player to set in the state
   */
  function setPlayer(player: PlayerDto): void {
    const logger = useLogger(LOGGER_SCOPE)

    try {
      if (!game.value) {
        throw new Error('The game has not been loaded yet.')
      }

      game.value.player = player
    } catch (error) {
      logger.error('Failed to set the Player of the Game', error)
      throw error
    }
  }

  return {
    player,
    monastery,
    surroundings,
    loadSessionGame,
    setProcess,
    setActor,
  }
})
