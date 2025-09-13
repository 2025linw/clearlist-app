import { createContext, useContext } from 'react';

export const LayoutContext = createContext({ hasHeader: false });

export function useLayoutContext() {
  return useContext(LayoutContext);
}
