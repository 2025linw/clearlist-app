export type LoginSchema = {
  userId: string;
  email: string;

  accessJwt: string;
  refreshJwt: string;
};

export type RefreshSchema = { accessJwt: string; refreshJwt: string };

export type TaskSchema = {
  id: string;

  title: string;
  notes: string;
  startDate?: string;
  startTime?: string;
  deadline?: string;

  completedOn?: string;
  loggedOn?: string;

  areaId?: string;
  projectId?: string;
  tags: TagSchema[];

  userId: string;

  createdOn: string;
  updatedOn: string;
  deletedOn?: string;
};
export type TaskQuery = TaskSchema[];

export type ProjectSchema = {
  id: string;

  title: string;
  notes: string;
  startDate?: string;
  startTime?: string;
  deadline?: string;

  completedOn?: string;
  loggedOn?: string;

  areaId?: string;
  tags: TagSchema[];

  userId: string;

  createdOn: string;
  updatedOn: string;
  deletedOn?: string;
};
export type ProjectQuery = ProjectSchema[];

export type AreaSchema = {
  id: string;

  name: string;
  iconUrl?: string;

  userId: string;

  createdOn: string;
  updatedOn: string;
  deletedOn?: string;
};
export type AreaQuery = AreaSchema[];

export type TagSchema = {
  id: string;

  label: string;
  color?: string;

  category?: string;

  userId: string;

  createdOn: string;
  updatedOn: string;
  deletedOn?: string;
};
export type TagQuery = TagSchema[];
