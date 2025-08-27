import { z } from 'zod';

import { schema as accountSchema } from '#/state/session/types';

const schema = z.object({
  colorMode: z.enum(['system', 'light', 'dark']),
  darkTheme: z.enum(['dim', 'dark']).optional(),

  session: z.object({
    account: accountSchema.optional(),
  }),
});
export type PersistedSchema = z.infer<typeof schema>;

export const defaults: PersistedSchema = {
  colorMode: 'system',
  darkTheme: 'dim',

  session: {
    account: undefined,
  },
};

export function tryParse(rawData: string): PersistedSchema | undefined {
  let objData;
  try {
    objData = JSON.parse(rawData);
  } catch (e) {
    console.error('JSON parse error:', e);
  }
  if (!objData) {
    return undefined;
  }

  const parsed = schema.safeParse(objData);
  if (parsed.success) {
    return objData;
  } else {
    const errors =
      parsed.error?.errors?.map((e) => ({
        code: e.code,
        // expected: e.expected,
        path: e.path?.join('.'),
      })) || [];

    console.error('zod parse error:', errors);

    return undefined;
  }
}

export function tryStringify(value: PersistedSchema): string | undefined {
  try {
    schema.parse(value);

    return JSON.stringify(value);
  } catch (e) {
    console.error('JSON stringify error:', e);

    return undefined;
  }
}
