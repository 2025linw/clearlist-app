import { jwtDecode } from 'jwt-decode';

export function isJwtExpired(jwt: string) {
  try {
    if (jwt) {
      const decoded = jwtDecode(jwt);
      if (decoded.exp) {
        return Date.now() >= decoded.exp * 1000;
      }
    }
  } catch (e) {
    console.error('error:', e);
  }

  return true;
}
