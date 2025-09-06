import { type LoginResponseSchema } from '#/types/response';

import { loginUser, registerUser, refreshUser } from '#/services/api';

import { type AccountSchema, type SessionApiContext } from './types';

export async function createAccount({
  email,
  password,
}: Parameters<SessionApiContext['createAccount']>[0]): Promise<AccountSchema> {
  try {
    const response = await registerUser({ email, password });
    if (!response) {
      throw Error('no response from server');
    }

    return await responseToSessionAccountOrThrow(response);
  } catch (e) {
    console.error('createAccount - error:', e);

    throw Error('error fetching request', { cause: e });
  }
}

export async function loginAccount({
  email,
  password,
}: Parameters<SessionApiContext['login']>[0]): Promise<AccountSchema> {
  try {
    const response = await loginUser({ email, password });
    if (!response) {
      throw Error('no response from server');
    }

    return await responseToSessionAccountOrThrow(response);
  } catch (e) {
    console.error('loginAccount - error:', e);

    throw Error('error fetching request', { cause: e });
  }
}

export async function resumeAccount(
  account: AccountSchema,
): Promise<AccountSchema> {
  try {
    if (!account.refreshJwt) {
      throw Error('no session saved');
    }

    const response = await refreshUser({ refreshJwt: account.refreshJwt });
    if (!response) {
      throw Error('no response from server');
    }

    const data = response.data;
    if (!data) {
      throw Error('no data from server response');
    }

    return {
      ...account,
      accessJwt: data.accessJwt,
      refreshJwt: data.refreshJwt,
    };
  } catch (e) {
    console.error('resumeAccount - error:', e);

    throw Error('error fetching request', { cause: e });
  }
}

async function responseToSessionAccountOrThrow(
  response: LoginResponseSchema,
): Promise<AccountSchema> {
  const account = await responseToSessionAccount(response);
  if (!account) {
    throw Error('response did not contain account information');
  }

  return account;
}

async function responseToSessionAccount(
  response: LoginResponseSchema,
): Promise<AccountSchema | undefined> {
  try {
    if (!['ok', 'success'].includes(response.status)) {
      console.warn('unexpected status:', response.status);

      return undefined;
    }

    const data = response.data;
    if (!data) {
      console.error('response did not contain data');

      return undefined;
    }

    if (!data.userId || !data.email || !data.accessJwt || !data.refreshJwt) {
      console.error('response data did not contain authentication info');

      return undefined;
    }

    return {
      email: data.email,
      userId: data.userId,
      accessJwt: data.accessJwt,
      refreshJwt: data.refreshJwt,
    };
  } catch (e) {
    console.error('responseToSessionAccount error', e);

    return undefined;
  }
}
