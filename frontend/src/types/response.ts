import { z } from 'zod';

// TODO: don't use zod for now
const baseResponseSchema = z.strictObject({
  status: z.enum(['ok', 'success', 'error']),

  version: z.string().optional(),

  message: z.string().optional(),
});

function responseWithDataSchema<T extends z.ZodTypeAny>(dataSchema: T) {
  return baseResponseSchema.extend({
    data: dataSchema,
  });
}

const loginDataSchema = z.strictObject({
  userId: z.string().uuid(),

  email: z.string().email(), // NOTE: this may be deprecated in future responses

  accessJwt: z.string().jwt({ alg: 'EdDSA' }),
  refreshJwt: z.string().jwt({ alg: 'EdDSA' }),
});

const refreshDataSchema = z.strictObject({
  accessJwt: z.string().jwt({ alg: 'EdDSA' }),
  refreshJwt: z.string().jwt({ alg: 'EdDSA' }),
});

export const loginResponseSchema = responseWithDataSchema(loginDataSchema);
export const refreshResponseSchema = responseWithDataSchema(refreshDataSchema);

export type LoginResponse = z.infer<typeof loginResponseSchema>;
export type RefreshResponse = z.infer<typeof refreshResponseSchema>;
