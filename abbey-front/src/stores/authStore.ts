import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

import { login, register, refresh } from '@/api/sdk.gen'

import useLogger from '@/composables/useLogger'

import { isJwtExpired } from '@/utils/jwt'

import type { LoginUserRequest, RegisterUserRequest } from '@/api'

const LOG_SCOPE = '@/stores/authStore.ts'

/**
 * Represents the authentication store, responsible for managing user authentication state and actions.
 */
export const useAuthStore = defineStore('auth', () => {
  /**
   * The token to authenticate the user with.
   */
  const sessionToken = ref<string | null>(null)

  /**
   * Is the user authenticated?
   */
  const authenticated = computed<boolean>(
    () => !!sessionToken.value && !isJwtExpired(sessionToken.value),
  )

  /**
   * Registers a new user.
   * @param request - The request object containing user registration details.
   */
  async function registerUser(request: RegisterUserRequest): Promise<void> {
    const logger = useLogger(LOG_SCOPE)

    try {
      const response = await register({ body: request })

      if (response.error) {
        throw response.error
      }
    } catch (error) {
      logger.error('Failed to register the user:', error)
      throw error
    }
  }

  /**
   * Authenticates a user.
   * @param request - The request object containing user login details.
   */
  async function loginUser(request: LoginUserRequest): Promise<void> {
    const logger = useLogger(LOG_SCOPE)

    try {
      const response = await login({ body: request })

      if (response.error) {
        throw response.error
      }

      sessionToken.value = response.data.session_token
    } catch (error) {
      logger.error('Failed to login the user:', error)
      throw error
    }
  }

  /**
   * Refreshes the user's credentials.
   */
  async function refreshUser(): Promise<void> {
    const logger = useLogger(LOG_SCOPE)

    try {
      const response = await refresh({})

      if (response.error) {
        throw response.error
      }

      sessionToken.value = response.data.session_token
    } catch (error) {
      logger.error("Failed to refresh the user's credentials:", error)
      throw error
    }
  }

  return {
    sessionToken: computed<string | null>(() => sessionToken.value),
    authenticated,
    registerUser,
    loginUser,
    refreshUser,
  }
})
