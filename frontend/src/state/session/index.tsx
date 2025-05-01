import React from 'react';

import type { SessionStateContext, SessionApiContext } from "#/state/session/types";

const StateContext = React.createContext<SessionStateContext>({
  accounts: [],
  currentAccount: undefined,
  hasSession: false,
});

const ApiContext = React.createContext<SessionApiContext>({
  createAccount: async () => {},
  login: async () => {},
  logoutCurrentAccount: async () => {},
  logoutEveryAccount: async () => {},
  resumeSession: async () => {},
  removeAccount: () => {},
});

export function Provider({ children }: React.PropsWithChildren<{}>) {
  // TODO
}

export function useSession() {
  return React.useContext(StateContext);
}

export function useSessionApi() {
  return React.useContext(ApiContext);
}

export function useRequireAuth() {
  const { hasSession } = useSession();

  // TODO
}
