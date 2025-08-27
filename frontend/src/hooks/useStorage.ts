import { useCallback } from 'react';

import { StorageInterface } from '#/storage';

type StorageSchema<T extends StorageInterface<Schema>> =
  T extends StorageInterface<infer S> ? S : never;

export function useStorage<Key extends keyof StorageSchema>(
  storage: StorageInterface<StorageSchema>,
  key: Key,
): [
  StorageSchema<StorageInterface>[Key] | undefined,
  (data: StorageSchema<StorageInterface>[Key]) => void,
] {
  type Schema = StorageSchema<StorageInterface>;
  const [value, setValue] = useState<Schema[Key] | undefined>(() =>
    storage.get(key),
  );

  const setter = useCallback(
    (data: Schema[Key]) => {
      setValue(data);

      storage.set(key, data);
    },
    [storage, scopes],
  );

  return [value, setter] as const;
}
