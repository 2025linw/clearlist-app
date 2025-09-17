import { TaskSchema as RequestSchema } from '#/types/request';

import apiClient from '../apiClient';
import { TaskSchema as ResponseSchema } from '../schemas';
import { TaskQueryResponse, TaskResponse } from '../types';
import { isServerError, isSuccess } from '../utils';

const placeholder: ResponseSchema = {
  id: '00000000-0000-0000-0000-000000000000',
  title: 'placeholder title',
  notes: 'placeholder notes',
  tags: [],
  userId: '00000000-0000-40ff-9000-000000000001',
  createdOn: 'placeholder creation date',
  updatedOn: 'placeholder modified date',
};
const placeholderQuery = [
  { ...placeholder, id: '00000000-0000-0000-0000-000000000000' },
  { ...placeholder, id: '00000000-0000-0000-0000-000000000001' },
  { ...placeholder, id: '00000000-0000-0000-0000-000000000002' },
  { ...placeholder, id: '00000000-0000-0000-0000-000000000003' },
  { ...placeholder, id: '00000000-0000-0000-0000-000000000004' },
];

export async function createTask(task: RequestSchema): Promise<TaskResponse | undefined> {
  try {
    const { status, statusText, data } = await apiClient.post(
      '/tasks/create',
      task,
    );
    if (isServerError(status)) {
      console.error(`createTask: server error - ${status} ${statusText}`);

      return undefined;
    }

    return isSuccess(status) ? data : undefined;
  } catch (e) {
    console.log(e);
  }

  return { status: 'error', data: placeholder };
}

export async function retrieveTask(
  taskId: string,
): Promise<TaskResponse | undefined> {
  return { status: 'error', data: placeholder };
}

export async function updateTask(
  taskId: string,
  task: RequestSchema,
): Promise<TaskResponse | undefined> {
  return undefined;
}

export async function deleteTask(taskId: string): Promise<boolean> {
  return false;
}

export async function queryTask({}): Promise<TaskQueryResponse> {
  return { status: 'error', data: placeholderQuery };
}
