import { AreaSchema as RequestSchema } from '#/types/request';

import { AreaSchema as ResponseSchema } from '#/services/schemas';

import { AreaQueryResponse, AreaResponse } from '../types';

const placeholder: ResponseSchema = {
  id: '00000000-0000-0000-0000-000000000000',
  name: 'placeholder title',
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

export async function createArea({}: ResponseSchema): Promise<AreaResponse> {
  return { status: 'error', data: placeholder };
}

export async function retrieveArea({
  Area_id,
}: {
  Area_id: string;
}): Promise<AreaResponse | undefined> {
  return { status: 'error', data: placeholder };
}

export async function updateArea({}: {
  AreaId: string;
  updatedArea: object;
}): Promise<AreaResponse | undefined> {
  return undefined;
}

export async function deleteArea({}): Promise<boolean> {
  return false;
}

export async function queryArea({}): Promise<AreaQueryResponse> {
  return { status: 'error', data: placeholderQuery };
}
