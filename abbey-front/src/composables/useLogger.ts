import { computed, getCurrentInstance } from "vue"

/**
 * The possible log levels.
 */
const LOG_LEVELS = ["debug", "info", "warn", "error"] as const

/**
 * The level of the log message.
 */
type LogLevel = typeof LOG_LEVELS[number]

/**
 * The options of a log message.
 */
type LogMessageOptions = {
  /**
   * The log level of the log message.
   */
  level?: LogLevel
  /**
   * The content of the log message.
   */
  message: string,
  /**
   * The context of the log message.
   */
  context: unknown[]
}

/**
 * A reusable composable representing a logger.
 */
export default function useLogger(scope?: string) {
  /**
   * The scope of the log message.
   */
  const computedScope = computed<string>(() => {
    if (scope) {
      return scope
    }

    return getCurrentInstance()?.type.__file?.replace(/.*\/src\//, '@/') ?? "??"
  })

  /**
   * Logs a message based on the provided options.
   * @param options - The options to base the log message on.
   */
  function logMessage(options: LogMessageOptions): void {
    const computedMessage = [options.level ? `[${options.level}]` : '', computedScope.value, "•", options.message].filter((option) => option.length).join(" ")

    switch (options.level) {
      case "info":
        console.info(computedMessage, ...options.context)
        break

      case "debug":
        console.debug(computedMessage, ...options.context)
        break

      case "warn":
        console.warn(computedMessage, ...options.context)
        break

      case "error":
        console.error(computedMessage, ...options.context)
        break

      default:
        console.log(computedMessage, ...options.context)
    }
  }

  /**
   * Logs a message without a log level.
   */
  function log(message: string, ...context: unknown[]): void {
    logMessage({ message, context })
  }

  /**
  * Logs a message with the info log level.
  */
  function info(message: string, ...context: unknown[]): void {
    logMessage({ message, level: "info", context })
  }

  /**
  * Logs a message with the debug log level.
  */
  function debug(message: string, ...context: unknown[]): void {
    logMessage({ message, level: "debug", context })
  }

  /**
  * Logs a message with the warn log level.
  */
  function warn(message: string, ...context: unknown[]): void {
    logMessage({ message, level: "warn", context })
  }

  /**
  * Logs a message with the error log level.
  */
  function error(message: string, ...context: unknown[]): void {
    logMessage({ message, level: "error", context })
  }

  return {
    log,
    info,
    debug,
    warn,
    error
  }
}
