import { computed, ref } from "vue"

/**
 * Represents the variant of a notification.
 */
type ANotificationVariant = "success" | "warning" | "error"

/**
 * Represents an application notification.
 */
export type ANotification = {
  /**
   * The ID of the notification.
   */
  id: string
  /**
   * The content of the notification.
   */
  content: string
  /**
   * The variant of the notification.
   */
  variant: ANotificationVariant
  /**
   * The lifespan of the notification in milliseconds.
   */
  lifespan: number,
}

/**
 * Represents the options of a notification to create.
 */
type ANotificationOptions = {
  /**
   * The content of the notification to create.
   */
  content: string
  /**
   * The variant of the notification to create.
   */
  variant: ANotificationVariant
  /**
   * The lifespan of the notification to create in milliseconds.
   */
  lifespan?: number
}

/**
 * All the notifications in the application.
 */
const allNotifications = ref<ANotification[]>([])

/**
 * A composable handling application notifications.
 */
export default function useNotifications() {

  /**
   * Calculates a comfortable duration of a notification.
   * @param content - The content of a notification.
   * @returns {number} - A duration of a notification in milliseconds.
   */
  function getNotificationDuration(content: string): number {
    const WORDS_PER_MINUTE = 200  // average reading speed
    const MINIMUM_MS = 3000       // never shorter than 3s
    const MS_PER_WORD = (60 / WORDS_PER_MINUTE) * 1000

    const wordCount = content.trim().split(/\s+/).length
    const duration = wordCount * MS_PER_WORD

    return Math.max(duration, MINIMUM_MS)
  }

  /**
   * Adds a new notification.
   * @param options - The options for the new notification.
   */
  function add(options: ANotificationOptions): void {
    allNotifications.value.push({
      id: crypto.randomUUID(),
      lifespan: options.lifespan ?? getNotificationDuration(options.content),
      ...options
    })
  }

  /**
   * Removes a notification.
   * @param id - The ID of the notification to remove.
   */
  function remove(id: ANotification["id"]): void {
    allNotifications.value = allNotifications.value.filter(notification => notification.id !== id)
  }

  return {
    allNotifications: computed(() => allNotifications.value),
    add,
    remove
  }
}
