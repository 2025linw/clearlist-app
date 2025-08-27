import AsyncStorage from '@react-native-async-storage/async-storage';

import {
  type PersistedSchema,
  defaults,
  tryParse,
  tryStringify,
} from './schema';

const CLIST_STORAGE = 'CLIST_STORAGE';

let _state: PersistedSchema = defaults;

// Initializes any persisted data
export async function init() {
  const stored = await readFromStorage();
  if (stored) {
    _state = stored;
  } else {
    console.warn('No data was persisted');
  }
}

// Gets value of persisted data by key
export function get<K extends keyof PersistedSchema>(
  key: K,
): PersistedSchema[K] {
  return _state[key];
}

// Adds key-value to persisted data
export async function write<K extends keyof PersistedSchema>(
  key: K,
  value: PersistedSchema[K],
): Promise<void> {
  _state = { ..._state, [key]: value };

  await writeToStorage(_state);
}

// Clear all persisted data from storage
export async function clearStorage() {
  try {
    await AsyncStorage.removeItem(CLIST_STORAGE);
  } catch (e) {
    console.error('error: %e', e);
  }
}

// Write persisted data to storage
async function writeToStorage(value: PersistedSchema) {
  const rawData = tryStringify(value);
  if (rawData) {
    try {
      await AsyncStorage.setItem(CLIST_STORAGE, rawData);
    } catch (e) {
      console.error('error: %s', e);
    }
  }
}

// Read persisted data from storage
async function readFromStorage(): Promise<PersistedSchema | undefined> {
  let rawData: string | null = null;
  try {
    rawData = await AsyncStorage.getItem(CLIST_STORAGE);
  } catch (e) {
    console.error('error: %s', e);
  }

  if (rawData) {
    const parsed = tryParse(rawData);
    if (parsed) {
      return parsed;
    }
  }
}
