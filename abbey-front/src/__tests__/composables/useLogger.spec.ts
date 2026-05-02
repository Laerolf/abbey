import { afterAll, beforeEach, describe, expect, it, vi } from "vitest";

import useLogger from "@/composables/useLogger";

const { consoleSpy } = vi.hoisted(() => ({
  consoleSpy: {
    log: vi.spyOn(console, 'log').mockImplementation(() => { }),
    debug: vi.spyOn(console, 'debug').mockImplementation(() => { }),
    info: vi.spyOn(console, 'info').mockImplementation(() => { }),
    warn: vi.spyOn(console, 'warn').mockImplementation(() => { }),
    error: vi.spyOn(console, 'error').mockImplementation(() => { }),
  }
}))

describe("@/composables/useLogger", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  afterAll(() => {
    vi.restoreAllMocks()
  })

  it("should log a message with an unknown scope and no log level", () => {
    // Given
    const expectedMessage = "It's golden week!"
    const expectedOutput = `?? • ${expectedMessage}`

    // When
    useLogger().log(expectedMessage)

    // Then
    expect(consoleSpy.log).toHaveBeenCalledWith(expectedOutput)
  })

  it("should log a message with the provided scope and no log level", () => {
    // Given
    const scope = "Heaven"

    const expectedMessage = "It's golden week!"
    const expectedOutput = `${scope} • ${expectedMessage}`

    // When
    useLogger(scope).log(expectedMessage)

    // Then
    expect(consoleSpy.log).toHaveBeenCalledWith(expectedOutput)
  })

  it("should log a message with the provided scope and a log level", () => {
    // Given
    const scope = "Hell"

    const expectedMessage = "It's golden week!"
    const expectedOutput = `[error] ${scope} • ${expectedMessage}`

    // When
    useLogger(scope).error(expectedMessage)

    // Then
    expect(consoleSpy.error).toHaveBeenCalledWith(expectedOutput)
  })

  it("should log a message with an unknown scope and with the info log level", () => {
    // Given
    const expectedMessage = "It's golden week!"
    const expectedOutput = `[info] ?? • ${expectedMessage}`

    // When
    useLogger().info(expectedMessage)

    // Then
    expect(consoleSpy.info).toHaveBeenCalledWith(expectedOutput)
  })

  it("should log a message with an unknown scope and with the debug log level", () => {
    // Given
    const expectedMessage = "It's golden week!"
    const expectedOutput = `[debug] ?? • ${expectedMessage}`

    // When
    useLogger().debug(expectedMessage)

    // Then
    expect(consoleSpy.debug).toHaveBeenCalledWith(expectedOutput)
  })

  it("should log a message with an unknown scope and with the warn log level", () => {
    // Given
    const expectedMessage = "It's golden week!"
    const expectedOutput = `[warn] ?? • ${expectedMessage}`

    // When
    useLogger().warn(expectedMessage)

    // Then
    expect(consoleSpy.warn).toHaveBeenCalledWith(expectedOutput)
  })

  it("should log a message with an unknown scope and with the error log level", () => {
    // Given
    const expectedMessage = "It's golden week!"
    const expectedOutput = `[error] ?? • ${expectedMessage}`

    // When
    useLogger().error(expectedMessage)

    // Then
    expect(consoleSpy.error).toHaveBeenCalledWith(expectedOutput)
  })
})
