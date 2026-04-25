import { Platform } from 'react-native';

import { BASE_URL } from '@/constants';
import { Tag, Task } from '@/types';

import { authClient } from '@/lib/auth-client';

import { TagResponse, TaskResponse } from './types';

function toYYYYMMDD(date: Date) {
  let dateStr = [date.getFullYear(), date.getMonth() + 1, date.getDay()].join('-');

  return dateStr;
}

export enum Category {
  Inbox,
  Today,
  Upcoming,
  Deadline,
  Logged,
  Trash,
}

const categoryQueryMap: Record<Category, string> = {
  [Category.Inbox]: '/api/tasks?start=false&deadline=false',
  [Category.Today]: `/api/tasks?start=${toYYYYMMDD(new Date())}`,
  [Category.Upcoming]: `/api/tasks?start[>]=${toYYYYMMDD(new Date())}`,
  [Category.Deadline]: '/api/tasks?deadline=true',
  [Category.Logged]: '/api/tasks?logged=true',
  [Category.Trash]: '/api/tasks?deleted=true',
};

export async function getTasks(category?: Category): Promise<Task[]> {
  let query: string = '/api/tasks';
  if (category) {
    query = categoryQueryMap[category];
  }

  const res =
    Platform.OS === 'web'
      ? await fetch(BASE_URL + query, {
          credentials: 'include',
        })
      : await fetch(BASE_URL + query, {
          headers: {
            Cookie: authClient.getCookie(),
          },
          credentials: 'omit',
        });

  if (res.status !== 200) {
    const body: TaskResponse = await res.json();

    console.error(body.message);

    return [];
  }

  const body: TaskResponse = await res.json();

  return body.data.tasks;
}

export async function getTags(): Promise<Tag[]> {
  const query = '/api/tags';

  const res =
    Platform.OS === 'web'
      ? await fetch(BASE_URL + query, {
          credentials: 'include',
        })
      : await fetch(BASE_URL + query, {
          headers: {
            Cookie: authClient.getCookie(),
          },
          credentials: 'omit',
        });

  if (res.status !== 200) {
    const body: TagResponse = await res.json();

    console.error(body.message);

    return [];
  }

  const body: TagResponse = await res.json();

  return body.data.tags;
}
