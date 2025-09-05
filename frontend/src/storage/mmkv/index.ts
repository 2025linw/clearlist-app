import { MMKV } from 'react-native-mmkv';

import { StorageInterface } from '#/storage';

export class MMKVStorage<Schema> implements StorageInterface<Schema> {
  protected store: MMKV;

  constructor({ id }: { id?: string }) {
    if (id) {
      this.store = new MMKV({ id });
    } else {
      this.store = new MMKV();
    }
  }

  set<Key extends keyof Schema>(key: Key, data: Schema[Key]) {
    this.store.set(key.toString(), JSON.stringify({ data }));
  }

  get<Key extends keyof Schema>(key: Key): Schema[Key] | undefined {
    const res = this.store.getString(key.toString());
    if (!res) return undefined;

    return JSON.parse(res).data;
  }

  getMany<Key extends keyof Schema>(
    keys: Key[],
  ): Array<Schema[Key]> | undefined {
    const values: Array<Schema[Key]> = [];
    for (const key of keys) {
      const value = this.get(key);
      if (!value) return undefined;

      values.push(value);
    }

    return values;
  }

  remove<Key extends keyof Schema>(key: Key) {
    this.store.delete(key.toString());
  }

  removeMany<Key extends keyof Schema>(keys: Key[]) {
    keys.map(key => this.remove(key));
  }

  removeAll() {
    this.store.clearAll();
  }
}
