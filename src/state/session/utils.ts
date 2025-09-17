import { loginUser, registerUser, refreshUser } from '#/services';
import { LoginResponse } from '#/services/types';

import { type AccountSchema, type SessionApiContext } from './types';

// TODO: Move all this into session/index
export async function createAccount({
  email,
  password,
}: Parameters<SessionApiContext['createAccount']>[0]): Promise<
  AccountSchema | undefined
> {
  try {
    const success = await registerUser({ email, password });
    if (typeof success === 'undefined') {
      throw Error('createAccount: server error occured');
    }
    if (!success) {
      console.error('createAccount: invalid user request');

      return undefined;
    }

    return await loginAccount({ email, password });
  } catch (e) {
    throw Error('createAccount: API request error', { cause: e });
  }
}

export async function loginAccount({
  email,
  password,
}: Parameters<SessionApiContext['login']>[0]): Promise<AccountSchema> {
  try {
    const response = await loginUser({ email, password });
    if (typeof response === 'number') {
      throw Error('loginAccount: server error occured');
    }

    return await responseToSessionAccountOrThrow(response);
  } catch (e) {
    throw Error('loginAccount: API request error', { cause: e });
  }
}

export async function resumeAccount(
  account: AccountSchema,
): Promise<AccountSchema> {
  try {
    if (!account.refreshJwt) {
      throw Error(
        'resumeAccount: no account was provided; this should have been caught earlier!!',
      );
    }

    const response = await refreshUser({ refreshJwt: account.refreshJwt });
    if (typeof response === 'undefined') {
      throw Error('resumeAccount: server error occured');
    }

    const data = response.data;
    if (!data) {
      throw Error('resumeAccount: no data given from server');
    }

    return {
      ...account,
      accessJwt: data.accessJwt,
      refreshJwt: data.refreshJwt,
    };
  } catch (e) {
    throw Error('resumeAccount: API request error', { cause: e });
  }
}

async function responseToSessionAccountOrThrow(
  response: LoginResponse,
): Promise<AccountSchema> {
  const account = await responseToSessionAccount(response);
  if (!account) {
    throw Error(
      'responseToSessionAccountOrThrow: response did not contain account information',
    );
  }

  return account;
}

async function responseToSessionAccount(
  response: LoginResponse,
): Promise<AccountSchema | undefined> {
  if (['error'].includes(response.status)) {
    console.error('responseToSessionAccount: error in request');

    return undefined;
  }

  const data = response.data;
  if (!data) {
    console.error('responseToSessionAccount: response contained no data');

    return undefined;
  }

  if (!data.userId || !data.email || !data.accessJwt || !data.refreshJwt) {
    console.error(
      'responseToSessionAccount: response contained no account info',
    );

    return undefined;
  }

  return {
    userId: data.userId,
    email: data.email,
    accessJwt: data.accessJwt,
    refreshJwt: data.refreshJwt,
  };
}
