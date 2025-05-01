
import type { PersistedAccount } from '#/state/persisted';

export type SessionAccount = PersistedAccount;

export type SessionStateContext = {
  accounts: SessionAccount[];
  currentAccount: SessionAccount | undefined;
  hasSession: boolean;
}

export type SessionApiContext = {
  createAccount: (
    props: {
      email: string,
      password: string,
    }
  ) => Promise<void>;
  login: (
    props: {
      email: string,
      password: string,
    }
  ) => Promise<void>;
  logoutCurrentAccount: () => Promise<void>;
  logoutEveryAccount: () => Promise<void>;
  resumeSession: (account: SessionAccount) => Promise<void>;
  removeAccount: (account: SessionAccount) => void;
}
