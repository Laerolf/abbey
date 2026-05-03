/**
 * Gets the expiration date from a raw JWT.
 * @param token - The JWT to decode.
 */
function getJwtExpirationDate(token: string): Date | null {
  const expirationPart = token.split('.')[1]

  if (!expirationPart) {
    return null
  }

  const payload = JSON.parse(atob(expirationPart))
  return new Date(payload.exp * 1000)
}

/**
 * Tests whether a JWT has expired or not.
 * @param token - The JWT to test.
 */
export function isJwtExpired(token: string): boolean {
  const jwtExpirationDate = getJwtExpirationDate(token)

  if (!jwtExpirationDate) {
    return true
  }

  return jwtExpirationDate < new Date()
}
