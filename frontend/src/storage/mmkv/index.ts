import { MMKV } from 'react-native-mmkv';

import { StorageInterface } from '#/storage';

const CLIST_SESSION = 'clist_session';

class Storage<Schema> implements StorageInterface<Schema> {
  protected store: MMKV;

  constructor({ id }: { id?: string }) {
    if (id) {
      this.store = new MMKV({ id });
    } else {
      this.store = new MMKV();
    }
  }

  set<Key extends keyof Schema>(key: Key, data: Schema[Key]) {
    this.store.set(key, JSON.stringify({ data }));
  }

  get<Key extends keyof Schema>(key: Key): Schema[Key] | undefined {
    const res = this.store.getString(key);
    if (!res) return undefined;

    return JSON.parse(res).data;
  }

  getMany<Key extends keyof Schema>(keys: Key[]) {
    keys.map(key => this.get(key));
  }

  remove<Key extends keyof Schema>(key: Key) {
    this.store.delete(key);
  }

  removeMany<Key extends keyof Schema>(keys: Key[]) {
    keys.map(key => this.remove(key));
  }

  removeAll() {
    this.store.clearAll();
  }
}

export const device = new Storage<Device>();
export const session = new Storage<Session>({ id: CLIST_SESSION });
