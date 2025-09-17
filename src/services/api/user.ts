import { type RefreshSchema, type AuthSchema } from '#/types/request';

import { INTERNAL_SERVER_ERR } from '#/services/consts';

import apiClient from '../apiClient';
import { type LoginResponse, type RefreshResponse } from '../types';
import { isServerError, isSuccess } from '../utils';

/**
 * Registers a new user via API
 *
 * @param creds - user registration credentials
 * @param creds.email - user registration email
 * @param creds.password - user registration password
 *
 * @returns {number} Status code
 *
 * @throws {Error} if server responds with status code 500 (Internal Server Error)
 * @throws {AxiosError} if Axios error, or unexpected status code is recieved
 */
export async function registerUser({
  email,
  password,
}: AuthSchema): Promise<number> {
  // TODO: catch any input not formatted as an email: backend or here? or both?

  const { status, statusText } = await apiClient.post('/auth/register', {
    email: email,
    password: password,
  });
  if (status === 500) {
    throw new Error(
      `registerUser: ${INTERNAL_SERVER_ERR} - ${status} ${statusText}`,
    );
  }

  return status;
}

/**
 * Login a user via API
 *
 * @param creds - user login credentials
 * @param creds.email - user login email
 * @param creds.password - user login password
 *
 * @returns {Promise<LoginResponse | number>}
 * - `LoginResponse` on success
 * - status code on failure and server error
 *
 * @throws {Error} if server responds with a 1xx or 3xx status code
 * @throws {AxiosError} if an Axios error occurs
 */
export async function loginUser({
  email,
  password,
}: AuthSchema): Promise<[number, LoginResponse | null]> {
  const { status, statusText, data } = await apiClient.post('/auth/login', {
    email: email,
    password: password,
  });
  if (status === 500) {
    throw new Error(
      `loginUser: ${INTERNAL_SERVER_ERR} - ${status} ${statusText}`,
    );
  }

  // TODO: eventually use zod or some type verification to parse data
  return isSuccess(status) ? data : status;
}

/**
 * Refresh a users session via API
 *
 * @param creds - user refresh credentials
 * @param creds.refreshJwt - user refresh token
 *
 * @returns {Promise<RefreshResponse | undefined>}
 * - `RefreshResponse` on success
 * - `undefined` on server error
 *
 * @throws {Error} if server response with 1xx or 3xx status code
 * @throws {AxiosError} if an axios error occurs
 */
export async function refreshUser({
  refreshJwt,
}: RefreshSchema): Promise<RefreshResponse | undefined> {
  const { status, statusText, data } = await apiClient.post('/auth/refresh', {
    refreshJwt: refreshJwt,
  });
  if (isServerError(status)) {
    console.error(
      `refreshUser: ${INTERNAL_SERVER_ERR} - ${status} ${statusText}`,
    );

    return undefined;
  }

  // TODO: eventually use zod or some type verification to parse data
  return data;
}
