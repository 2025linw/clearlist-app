import { Tag, Task } from '@/types';

export type TaskResponse = {
  message?: string;
  data: {
    count: number;
    tasks: Task[];
  };
};

export type TagResponse = {
  message?: string;
  data: {
    count: number;
    tags: Tag[];
  };
};
