import { jwtDecode } from 'jwt-decode';

import { type AccountSchema } from './types';

export function isSessionExpired(account: AccountSchema) {
  try {
    if (account.accessJwt) {
      const decoded = jwtDecode(account.accessJwt);
      if (decoded.exp) {
        return Date.now() >= decoded.exp * 1000;
      }
    }
  } catch (e) {
    console.error('error:', e);
  }

  return true;
}
