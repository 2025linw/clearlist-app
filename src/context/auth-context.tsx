import {
  createContext,
  PropsWithChildren,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from 'react';

import { authClient } from '@/lib/auth-client';
import { SessionApiContext, UserSessionContext } from './types';
import { useRouter } from 'expo-router';

export const AuthContext = createContext<UserSessionContext>({
  currentSession: undefined,
  hasSession: false,
});
export const ApiContext = createContext<SessionApiContext>({
  createAccount: async () => {},
  login: async () => {},
  logout: async () => {},
});

export function AuthProvider({ children }: PropsWithChildren) {
  const router = useRouter();

  const [user, setUser] = useState<UserSessionContext>({
    currentSession: undefined,
    hasSession: false,
  });

  useEffect(() => {
    const getSession = async () => {
      const { data, error } = await authClient.getSession();

      if (error) {
        console.error(error);

        return;
      }

      if (!data) {
        // if there isn't an existing session or session expired
        return;
      }

      console.log(data);

      setUser({
        currentSession: data.session.token,
        hasSession: true,
      });
    };

    getSession();
  }, []);

  const createAccount = useCallback<SessionApiContext['createAccount']>(
    async (params) => {
      const { data, error } = await authClient.signUp.email({
        email: params.email,
        password: params.password,
        name: params.name,
      });

      if (error) {
        console.error(error);

        return;
      }

      setUser({
        currentSession: data.token!,
        hasSession: true,
      });
    },
    [],
  );

  const login = useCallback<SessionApiContext['login']>(async (params) => {
    const { data, error } = await authClient.signIn.email({
      email: params.email,
      password: params.password,
    });

    if (error) {
      console.error(error);

      return;
    }

    setUser({
      currentSession: data.token!,
      hasSession: true,
    });
  }, []);

  const logout = useCallback<SessionApiContext['logout']>(async () => {
    const { error } = await authClient.signOut();

    if (error) {
      console.error(error);

      return;
    }

    setUser({
      currentSession: undefined,
      hasSession: false,
    });

    router.navigate('/');
  }, [router]);

  const api = useMemo(
    () => ({
      createAccount,
      login,
      logout,
    }),
    [createAccount, login, logout],
  );

  return (
    <AuthContext value={user}>
      <ApiContext value={api}>{children}</ApiContext>
    </AuthContext>
  );
}

export function useSession() {
  return useContext(AuthContext);
}

export function useSessionApi() {
  return useContext(ApiContext);
}
