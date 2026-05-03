import type { AppError } from '@/api'

/**
 * Tests whether the provided object is an AppError.
 * @param object - The object to test.
 */
export function isAppError(object: unknown): object is AppError {
  return (
    typeof object === 'object' &&
    object !== null &&
    'message' in object &&
    'code' in object &&
    typeof (object as AppError).message === 'string' &&
    typeof (object as AppError).code === 'string'
  )
}
