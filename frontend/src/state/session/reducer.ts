import { type AccountSchema } from './types';

export type State = {
  readonly account: AccountSchema | undefined;

  needsPersist: boolean;
};

export type Action =
  | {
      type: 'logged-in';
      newAccount: AccountSchema;
    }
  | {
      type: 'logged-out';
    };

export function getInitialState(
  persistedAccount: AccountSchema | undefined,
): State {
  return {
    account: persistedAccount,
    needsPersist: false,
  };
}

export function reducer(state: State, action: Action): State {
  switch (action.type) {
    case 'logged-in': {
      const { newAccount } = action;

      return {
        account: newAccount,
        needsPersist: true,
      };
    }
    case 'logged-out': {
      const { account } = state;

      if (!account) {
        return {
          account: undefined,
          needsPersist: true,
        };
      }

      return {
        account: {
          ...account,
          accessJwt: undefined,
          refreshJwt: undefined,
        },
        needsPersist: true,
      };
    }
  }
}
