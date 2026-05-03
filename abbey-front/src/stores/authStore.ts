import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

import { login, register } from '@/api/sdk.gen'

import useLogger from '@/composables/useLogger'

import type { LoginUserRequest, RegisterUserRequest } from '@/api'

const LOG_SCOPE = '@/stores/authStore.ts'

/**
 * Represents the authentication store, responsible for managing user authentication state and actions.
 */
export const useAuthStore = defineStore('auth', () => {
  /**
   * The token to authenticate the user with.
   */
  const sessionToken = ref<string>()

  /**
   * Is the user authenticated?
   */
  const authenticated = computed<boolean>(() => !!sessionToken.value)

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

  return { sessionToken, authenticated, registerUser, loginUser }
})
