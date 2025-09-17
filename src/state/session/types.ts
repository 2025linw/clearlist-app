import { AccountSchema } from '#/storage/schemas';

export { AccountSchema } from '#/storage/schemas';

export type SessionStateContext = {
  account?: AccountSchema;

  hasSession: boolean;
};

export type SessionApiContext = {
  createAccount(props: { email: string; password: string }): Promise<void>;
  login(props: { email: string; password: string }): Promise<void>;
  logout(): void;
  resumeSession(account: AccountSchema): Promise<void>;
};
