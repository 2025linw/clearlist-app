import React from 'react';

import * as persisted from '#/storage/async-storage';

import * as accountF from './account';
import { getInitialState, reducer } from './reducer';
import { type SessionStateContext, type SessionApiContext } from './types';

const StateContext = React.createContext<SessionStateContext>({
  account: undefined,
  hasSession: false,
});

const ApiContext = React.createContext<SessionApiContext>({
  createAccount: async () => {},
  login: async () => {},
  logout: async () => {},
  resumeSession: async () => {},
});

export function Provider({ children }: React.PropsWithChildren<unknown>) {
  const [state, dispatch] = React.useReducer(reducer, null, () => {
    const initialState = getInitialState(persisted.get('session').account);

    return initialState;
  });

  const createAccount = React.useCallback<SessionApiContext['createAccount']>(
    async params => {
      const account = await accountF.createAccount(params);

      dispatch({ type: 'logged-in', newAccount: account });
    },
    [],
  );
  const login = React.useCallback<SessionApiContext['login']>(async params => {
    const account = await accountF.loginAccount(params);

    dispatch({ type: 'logged-in', newAccount: account });
  }, []);
  const logout = React.useCallback<SessionApiContext['logout']>(async () => {
    dispatch({ type: 'logged-out' });
  }, []);
  const resumeSession = React.useCallback<SessionApiContext['resumeSession']>(
    async params => {
      const account = await accountF.resumeAccount(params);

      dispatch({ type: 'logged-in', newAccount: account });
    },
    [],
  );

  // Track if data needs to be persisted
  React.useEffect(() => {
    if (state.needsPersist) {
      state.needsPersist = false;

      const persistedData = { account: state.account };
      persisted.write('session', persistedData);
    }
  }, [state]);

  // Keep the account state context
  const stateContext = React.useMemo(
    () => ({ account: state.account, hasSession: !!state.account?.refreshJwt }),
    [state],
  );

  // Keep the account api context
  const api = React.useMemo(
    () => ({ createAccount, login, logout, resumeSession }),
    [createAccount, login, logout, resumeSession],
  );

  return (
    <StateContext.Provider value={stateContext}>
      <ApiContext.Provider value={api}>{children}</ApiContext.Provider>
    </StateContext.Provider>
  );
}

export function useSession() {
  return React.useContext(StateContext);
}

export function useSessionApi() {
  return React.useContext(ApiContext);
}
