import { defineStore } from 'pinia'

import { login, register } from '@/api/sdk.gen'

import type { LoginUserRequest, RegisterUserRequest } from '@/api'
import { ref } from 'vue'

/**
 * Represents the authentication store, responsible for managing user authentication state and actions.
 */
export const useAuthStore = defineStore('auth', () => {
  /**
   * The token to authenticate the user with.
   */
  const sessionToken = ref<string>()

  /**
   * Registers a new user.
   * @param request - The request object containing user registration details.
   */
  async function registerUser(request: RegisterUserRequest): Promise<void> {
    try {
      await register({ body: request })
    } catch (error) {
      console.error('Failed to register the user:', error)
      throw error
    }
  }

  /**
   * Authenticates a user.
   * @param request - The request object containing user login details.
   */
  async function loginUser(request: LoginUserRequest): Promise<void> {
    try {
      const response = await login({ body: request })

      if (response.error) {
        throw response.error
      }

      sessionToken.value = response.data.session_token
    } catch (error) {
      console.error('Failed to login the user:', error)
      throw error
    }
  }

  return { sessionToken, registerUser, loginUser }
})
