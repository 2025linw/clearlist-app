export type UserSessionContext = {
  currentSession: string | undefined;
  hasSession: boolean;
};

export type SessionApiContext = {
  createAccount: (props: {
    email: string;
    password: string;
    name: string;
  }) => Promise<void>;
  login: (props: { email: string; password: string }) => Promise<void>;
  logout: () => Promise<void>;
};
