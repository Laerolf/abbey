import type { ActorDto, AppError, MonkDto } from '@/api'

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

/**
 * Tests whether the provided object is an ActorDto.
 * @param object - The object to test.
 */
export function isActorDto(object: unknown): object is ActorDto {
  return (
    typeof object === 'object' &&
    object !== null &&
    'id' in object &&
    'type' in object &&
    'assigned_process_id' in object &&
    typeof (object as ActorDto).id === 'number' &&
    typeof (object as ActorDto).type === 'string' && ["Player", "Monk"].includes((object as ActorDto).type) &&
    (typeof (object as ActorDto).assigned_process_id === 'string' || typeof (object as ActorDto).assigned_process_id === 'undefined')
  )
}

/**
 * Tests whether the provided object is a MonkDto.
 * @param object - The object to test.
 */
export function isMonkDto(object: unknown): object is MonkDto {
  return (
    isActorDto(object) && (object as ActorDto).type === "Monk"
  )
}
