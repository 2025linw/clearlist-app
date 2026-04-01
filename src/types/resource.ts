export type Task = {
  id: string;

  title: string;
  notes: string;
  start_date: Date;
  start_time: Date;
  deadline: Date;

  tags: Tag[];
};

export type Tag = {
  id: string;

  label: string;
  category: string;
};
