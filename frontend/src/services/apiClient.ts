import axios from 'axios';

const apiClient = axios.create({
  baseURL: 'https://todo.saphynet.io/api',
  headers: {
    'Content-Type': 'application/json',
  },
});

export default apiClient;
