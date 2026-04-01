import { useSession } from '@/context/auth-context';

import LogInScreen from '#/log-in-screen';
import ListScreen from '#/lists-screen';

export default function Index() {
  const { hasSession } = useSession();

  return hasSession ? <ListScreen /> : <LogInScreen />;
}
