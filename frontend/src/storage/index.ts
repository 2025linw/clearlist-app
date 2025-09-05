import { MMKVStorage } from './mmkv';
import { type Device } from './schemas';

const CLIST_DEVICE = 'clist_device';

export interface StorageInterface<Schema> {
  set<Key extends keyof Schema>(key: Key, data: Schema[Key]): void;
  get<Key extends keyof Schema>(key: Key): Schema[Key] | undefined;
  getMany<Key extends keyof Schema>(
    keys: Key[],
  ): Array<Schema[Key]> | undefined;
  remove<Key extends keyof Schema>(key: Key): void;
  removeMany<Key extends keyof Schema>(keys: Key[]): void;
  removeAll(): void;
}

export interface AsyncStorageInterface<Schema> {
  set<Key extends keyof Schema>(key: Key, data: Schema[Key]): Promise<void>;
  get<Key extends keyof Schema>(key: Key): Promise<Schema[Key] | undefined>;
  getMany<Key extends keyof Schema>(
    keys: Key[],
  ): Promise<Array<Schema[Key]> | undefined>;
  remove<Key extends keyof Schema>(key: Key): Promise<void>;
  removeMany<Key extends keyof Schema>(keys: Key[]): Promise<void>;
  removeAll(): Promise<void>;
}

export const device = new MMKVStorage<Device>({ id: CLIST_DEVICE });
