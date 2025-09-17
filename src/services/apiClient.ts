import axios from 'axios';

import { nonAPIStatusCodes } from '#/services/utils';

const apiClient = axios.create({
  baseURL: 'https://todo.saphynet.io/api',
  headers: { 'Content-Type': 'application/json' },
  validateStatus: status => {
    // Status codes to propagate down to functions

    if (nonAPIStatusCodes(status)) return false;

    if ([200, 201, 204].includes(status)) {
      return true;
    }
    if ([400, 401, 403, 404, 409, 429].includes(status)) {
      return true;
    }
    if ([500].includes(status)) {
      return true;
    }

    return false;
  },
});

export function setAuthToken(token?: string) {
  if (token) {
    apiClient.defaults.headers.Authorization = `Bearer ${token}`;
  } else {
    delete apiClient.defaults.headers.Authorization;
  }
}

export default apiClient;
