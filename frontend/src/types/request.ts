import { z } from 'zod';

const loginSchema = z.strictObject({
  email: z.string().email(),
  password: z.string(),
});
export interface LoginSchema extends z.infer<typeof loginSchema> {}

const refreshSchema = z.strictObject({
  refreshJwt: z.string().jwt({ alg: 'EdDSA' }),
});
export interface RefreshSchema extends z.infer<typeof refreshSchema> {}
