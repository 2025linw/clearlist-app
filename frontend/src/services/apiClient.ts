import axios from 'axios';

const apiClient = axios.create({
  baseURL: 'https://todo.saphynet.io/api',
  headers: { 'Content-Type': 'application/json' },
});

export function setAuthToken(token?: string) {
  if (token) {
    apiClient.defaults.headers.Authorization = `Bearer ${token}`;
  } else {
    delete apiClient.defaults.headers.Authorization;
  }
}

export default apiClient;
