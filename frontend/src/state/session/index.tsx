import {
  createContext,
  useReducer,
  useCallback,
  useEffect,
  useMemo,
  useContext,
} from 'react';

import * as persisted from '#/storage/async-storage';

import { getInitialState, reducer } from './reducer';
import { type SessionStateContext, type SessionApiContext } from './types';
import * as accountF from './utils';

const StateContext = createContext<SessionStateContext>({
  account: undefined,
  hasSession: false,
});

const ApiContext = createContext<SessionApiContext>({
  createAccount: async () => {},
  login: async () => {},
  logout: async () => {},
  resumeSession: async () => {},
});

export function Provider({ children }: React.PropsWithChildren<unknown>) {
  const [state, dispatch] = useReducer(reducer, null, () => {
    const initialState = getInitialState(persisted.get('account'));

    return initialState;
  });

  const createAccount = useCallback<SessionApiContext['createAccount']>(
    async params => {
      const account = await accountF.createAccount(params);

      dispatch({ type: 'logged-in', newAccount: account });
    },
    [],
  );
  const login = useCallback<SessionApiContext['login']>(async params => {
    const account = await accountF.loginAccount(params);

    dispatch({ type: 'logged-in', newAccount: account });
  }, []);
  const logout = useCallback<SessionApiContext['logout']>(async () => {
    dispatch({ type: 'logged-out' });
  }, []);
  const resumeSession = useCallback<SessionApiContext['resumeSession']>(
    async params => {
      const account = await accountF.resumeAccount(params);

      dispatch({ type: 'logged-in', newAccount: account });
    },
    [],
  );

  // Persist data if needed
  useEffect(() => {
    if (state.needsPersist) {
      state.needsPersist = false;

      persisted.write('account', state.account);
    }
  }, [state]);

  // Keep the account state context
  const stateContext = useMemo(
    () => ({ account: state.account, hasSession: !!state.account?.refreshJwt }),
    [state],
  );

  // Keep the account api context
  const api = useMemo(
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
  return useContext(StateContext);
}

export function useSessionApi() {
  return useContext(ApiContext);
}
