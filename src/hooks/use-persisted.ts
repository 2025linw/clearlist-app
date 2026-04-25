import * as SecureStore from 'expo-secure-store';
import { useEffect, useState } from 'react';

type Key = 'theme';

export default function usePersisted<T>(key: Key, initialValue?: T) {
  const [value, setValue] = useState<typeof initialValue | null>(initialValue || null);

  useEffect(() => {
    SecureStore.getItemAsync(key).then((stored) => {
      if (stored !== null) {
        setValue(JSON.parse(stored));
      } else {
        setValue(undefined);
      }
    });
  }, [key]);

  const setPersisted = (val: T) => {
    setValue(val);

    SecureStore.setItemAsync(key, JSON.stringify(val));
  };

  return [value, setPersisted];
}
