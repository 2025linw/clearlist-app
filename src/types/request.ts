type Compare = '=' | '!=' | '<' | '<=' | '>' | '>=';
type QueryMethod<T> = boolean | T | [T, Compare];

export type AuthSchema = { email: string; password: string };

export type RefreshSchema = { refreshJwt: string };

export type TaskSchema = {
  title?: string;
  notes?: string;
  startDate?: string;
  startTime?: string;
  deadline?: string;

  completedOn?: string;
  loggedOn?: string;

  areaId?: string;
  projectId?: string;
  tagIds?: string[];
};
export type TaskQuery = {
  title?: QueryMethod<string>;
  startDate?: QueryMethod<string>;
  startTime?: QueryMethod<string>;
  deadline?: QueryMethod<string>;

  completed?: boolean;
  logged?: boolean;

  areaId?: string;
  projectId?: string;
  tagIds?: string[];

  deleted?: boolean;
};

export type ProjectSchema = {
  title?: string;
  notes?: string;
  startDate?: string;
  startTime?: string;
  deadline?: string;

  completedOn?: string;
  loggedOn?: string;

  areaId?: string;
  tags?: string[];
};
export type ProjectQuery = {
  title?: QueryMethod<string>;
  startDate?: QueryMethod<string>;
  startTime?: QueryMethod<string>;
  deadline?: QueryMethod<string>;

  completed?: boolean;
  logged?: boolean;

  areaId?: string;
  tagIds?: string[];

  deleted?: boolean;
};

export type AreaSchema = { name?: string; iconUrl?: string };
export type AreaQuery = { name?: QueryMethod<string> };

export type TagSchema = {
  label?: string;
  color?: string;

  category?: string;
};
export type TagQuery = { label?: string; category?: string };
