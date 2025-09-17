import { createContext, useContext } from 'react';

type ErrorObj = {
  code: number;

  errorMsg: string;
}

type LayoutContextType = {
  hasHeader: boolean,
  error?: ErrorObj,
}

export const LayoutContext = createContext<LayoutContextType>({ hasHeader: false });

export function useLayoutContext() {
  return useContext(LayoutContext);
}
