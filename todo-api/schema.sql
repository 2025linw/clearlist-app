CREATE SCHEMA todo_auth AUTHORIZATION CURRENT_USER;
CREATE SCHEMA todo_data AUTHORIZATION CURRENT_USER;


-- Tables in user schema
CREATE TABLE todo_auth.users
(
	user_id uuid,
	username varchar(50),
	password_hash text,

	email varchar(320),

	PRIMARY KEY (user_id)
);


-- Tables in todo_data schema
CREATE TABLE todo_data.areas	(
	area_id	uuid,
	area_name varchar(255),

	icon_url text,

	user_id uuid NOT NULL,

	PRIMARY	KEY(area_id),
	FOREIGN	KEY(user_id) REFERENCES todo_auth.users(user_id)
);

CREATE TABLE todo_data.todos	(
	todo_id uuid,
	todo_title varchar(255),
	todo_notes text,

	area_id uuid,

	start_date date,
	start_time time(0),
	deadline date,

	user_id	uuid NOT NULL,
	created_on timestamp(0) DEFAULT CURRENT_TIMESTAMP,
	completed_on timestamp(0),
	logged_on timestamp(0),
	trashed_on timestamp(0),

	PRIMARY KEY(todo_id),
	FOREIGN KEY(user_id) REFERENCES todo_auth.users(user_id)
);

CREATE TABLE todo_data.projects (
	project_id uuid,

	PRIMARY KEY(project_id),
	FOREIGN KEY(project_id) REFERENCES todo_data.todos(todo_id)
);

CREATE TABLE todo_data.tasks (
	task_id uuid,

	project_id uuid,

	PRIMARY KEY(task_id),
	FOREIGN KEY(task_id) REFERENCES todo_data.todos(todo_id),
	FOREIGN KEY(project_id) REFERENCES todo_data.projects(project_id)
);

CREATE TABLE todo_data.tags (
	tag_id uuid,
	tag_label varchar(255),
	tag_category varchar(255),

	color varchar(255),

	user_id uuid NOT NULL,

	PRIMARY KEY(tag_id),
	FOREIGN KEY(user_id) REFERENCES todo_auth.users(user_id)
);

CREATE TABLE todo_data.tagged_with (
	todo_id uuid,
	tag_id uuid,

	PRIMARY KEY(todo_id, tag_id),
	FOREIGN KEY(todo_id) REFERENCES todo_data.todos(todo_id),
	FOREIGN KEY(tag_id) REFERENCES todo_data.tags(tag_id)
)
