import { z } from 'zod';

export const schema = z.strictObject({
  userId: z.string(),

  email: z.string(),

  accessJwt: z.string().optional(),
  refreshJwt: z.string().optional(),
});
export type AccountSchema = z.infer<typeof schema>;

export type SessionStateContext = {
  account: AccountSchema | undefined;

  hasSession: boolean;
};

export type SessionApiContext = {
  createAccount: (props: { email: string; password: string }) => Promise<void>;
  login: (props: { email: string; password: string }) => Promise<void>;
  logout: () => void;
  resumeSession: (account: AccountSchema) => Promise<void>;
};
