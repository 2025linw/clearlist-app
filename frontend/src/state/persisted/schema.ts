import { z } from 'zod';

const accountSchema = z.object({
  uid: z.string(),
  email: z.string(),
  emailConfirmed: z.boolean().optional(),

  accessJwt: z.string().optional(),
  refreshJwt: z.string().optional(),
});
export type PersistedAccount = z.infer<typeof accountSchema>;
