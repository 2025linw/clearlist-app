import {
  LoginSchema,
  RefreshSchema,
  TaskSchema,
  TaskQuery,
  ProjectSchema,
  ProjectQuery,
  AreaSchema,
  AreaQuery,
  TagSchema,
  TagQuery,
} from './schemas';

type BaseResponse = { status: 'ok' | 'success' | 'error' };
type DataResponse<T> = BaseResponse & { data: T };

export type HealthcheckResponse = BaseResponse & {
  version?: string;

  message?: string;
};

export type LoginResponse = DataResponse<LoginSchema>;
export type RefreshResponse = DataResponse<RefreshSchema>;

export type TaskResponse = DataResponse<TaskSchema>;
export type TaskQueryResponse = DataResponse<TaskQuery>;

export type ProjectResponse = DataResponse<ProjectSchema>;
export type ProjectQueryResponse = DataResponse<ProjectQuery>;

export type AreaResponse = DataResponse<AreaSchema>;
export type AreaQueryResponse = DataResponse<AreaQuery>;

export type TagResponse = DataResponse<TagSchema>;
export type TagQueryResponse = DataResponse<TagQuery>;
