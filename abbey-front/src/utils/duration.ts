import type { DurationDto } from '@/api'

/**
 * Returns the duration in milliseconds.
 */
export function durationToMilliseconds(duration: DurationDto): number {
  return duration.seconds * 1000 + duration.nanoseconds / 1_000_000
}
