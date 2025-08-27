import { useCallback, useEffect, useState } from 'react';
import { MMKV } from 'react-native-mmkv';

import { type Device, type Session } from './schema';

export * from './schema';

export interface StorageInterface<Schema> {
  store: unknown;

  set<Key extends keyof Schema>(key: Key, data: Schema[Key]);
  get<Key extends keyof Schema>(key: Key): Schema[Key];
  getMany<Key extends keyof Schema>(keys: Key[]): Array<Schema[Key]>;
  remove<Key extends keyof Schema>(key: Key);
  removeMany<Key extends keyof Schema>(keys: Key[]);
  removeAll();
}

// This is copied from bluesky-socal/social-app
class Storage<Scopes extends unknown[], Schema> {
  protected sep = ':';
  protected store: MMKV;

  constructor({ id }: { id: string }) {
    this.store = new MMKV({ id });
  }

  set<Key extends keyof Schema>(
    scopes: [...Scopes, Key],
    data: Schema[Key],
  ): void {
    this.store.set(scopes.join(this.sep), JSON.stringify({ data }));
  }

  get<Key extends keyof Schema>(
    scopes: [...Scopes, Key],
  ): Schema[Key] | undefined {
    const res = this.store.getString(scopes.join(this.sep));
    if (!res) return undefined;

    return JSON.parse(res).data;
  }

  remove<Key extends keyof Schema>(scopes: [...Scopes, Key]) {
    this.store.delete(scopes.join(this.sep));
  }

  removeMany<Key extends keyof Schema>(scopes: [...Scopes], keys: Key[]) {
    keys.forEach((key) => this.remove([...scopes, key]));
  }

  removeAll() {
    this.store.clearAll();
  }

  addOnValueChangedListener<Key extends keyof Schema>(
    scopes: [...Scopes, Key],
    callback: () => void,
  ) {
    return this.store.addOnValueChangedListener((key) => {
      if (key === scopes.join(this.sep)) {
        callback();
      }
    });
  }
}

// Type of storage scopes
type StorageScopes<T extends Storage<unknown, unknown>> =
  T extends Storage<infer S, unknown> ? S : never;
// Type of
type StorageSchema<T extends Storage<unknown, unknown>> =
  T extends Storage<unknown, infer U> ? U : never;

export function useStorage<
  Store extends Storage<unknown, unknown>,
  Key extends keyof StorageSchema<Store>,
>(
  storage: Store,
  scopes: [...StorageScopes<Store>, Key],
): [
  StorageSchema<Store>[Key] | undefined,
  (data: StorageSchema<Store>[Key]) => void,
] {
  type Schema = StorageSchema<Store>;
  const [value, setValue] = useState<Schema[Key] | undefined>(() =>
    storage.get(scopes),
  );

  useEffect(() => {
    const sub = storage.addOnValueChangedListener(scopes, () => {
      setValue(storage.get(scopes));
    });
    return () => sub.remove();
  }, [storage, scopes]);

  const setter = useCallback(
    (data: Schema[Key]) => {
      setValue(data);

      storage.set(scopes, data);
    },
    [storage, scopes],
  );

  return [value, setter] as const;
}

export const device = new Storage<[], Device>({ id: 'clist_device' });
export const session = new Storage<[string], Session>({ id: 'clist_session'})
