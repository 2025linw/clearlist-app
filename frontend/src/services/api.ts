import { type RefreshSchema, type LoginSchema } from '#/types/request';
import {
  type LoginResponse,
  loginResponseSchema,
  type RefreshResponse,
  refreshResponseSchema,
} from '#/types/response';

import apiClient from './apiClient';

function isSuccessOrUserError(status: number): boolean {
  return (200 <= status && status < 300) || (400 <= status && status < 500);
}

export async function registerUser({
  email,
  password,
}: LoginSchema): Promise<LoginResponse | undefined> {
  const { status, statusText, data } = await apiClient.post('/auth/register', {
    email: email,
    password: password,
  });
  if (!isSuccessOrUserError(status)) {
    console.error(`Internal error: ${status} - ${statusText}`);

    return undefined;
  }

  return loginResponseSchema.parse(data);
}

export async function loginUser({
  email,
  password,
}: LoginSchema): Promise<LoginResponse | undefined> {
  const { status, statusText, data } = await apiClient.post('/auth/login', {
    email: email,
    password: password,
  });
  if (!isSuccessOrUserError(status)) {
    console.error(`Internal error: ${status} - ${statusText}`);

    return undefined;
  }

  return loginResponseSchema.parse(data);
}

export async function refreshUser({
  refreshJwt,
}: RefreshSchema): Promise<RefreshResponse | undefined> {
  const { status, statusText, data } = await apiClient.post('/auth/refresh', {
    refreshJwt: refreshJwt,
  });
  if (!isSuccessOrUserError(status)) {
    console.error(`Internal error: ${status} - ${statusText}`);

    return undefined;
  }

  return refreshResponseSchema.parse(data);
}
