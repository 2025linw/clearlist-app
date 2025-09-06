import { type RefreshSchema, type LoginSchema } from '#/types/request';
import {
  type LoginResponseSchema,
  type RefreshResponseSchema,
} from '#/types/response';

import { isSuccess, isUserErr } from '#/services/utils';

import apiClient from './apiClient';

function isInternalError(status: number): boolean {
  return !isSuccess(status) && !isUserErr(status);
}

export async function registerUser({
  email,
  password,
}: LoginSchema): Promise<LoginResponseSchema | undefined> {
  const { status, statusText, data } = await apiClient.post('/auth/register', {
    email: email,
    password: password,
  });
  if (isInternalError(status)) {
    console.error(`registerUser - internal error: ${status} - ${statusText}`);

    return undefined;
  }

  // TODO: eventually use zod or some type verification to parse data
  return data;
}

export async function loginUser({
  email,
  password,
}: LoginSchema): Promise<LoginResponseSchema | undefined> {
  const { status, statusText, data } = await apiClient.post('/auth/login', {
    email: email,
    password: password,
  });
  if (isInternalError(status)) {
    console.error(`loginUser - internal error: ${status} - ${statusText}`);

    return undefined;
  }

  // TODO: eventually use zod or some type verification to parse data
  return data;
}

export async function refreshUser({
  refreshJwt,
}: RefreshSchema): Promise<RefreshResponseSchema | undefined> {
  const { status, statusText, data } = await apiClient.post('/auth/refresh', {
    refreshJwt: refreshJwt,
  });
  if (isInternalError(status)) {
    console.error(`refreshUser - internal error: ${status} - ${statusText}`);

    return undefined;
  }

  // TODO: eventually use zod or some type verification to parse data
  return data;
}
