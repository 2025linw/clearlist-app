-- Drop Schemas
DROP SCHEMA IF EXISTS todo_auth, todo_data CASCADE;
DROP EXTENSION IF EXISTS citext;


-- Add CITEXT module
CREATE EXTENSION citext;


-- Create Schemas with permission changes
CREATE SCHEMA todo_auth;
GRANT USAGE ON SCHEMA todo_auth TO todo_app;

CREATE SCHEMA todo_data;
GRANT USAGE ON SCHEMA todo_data TO todo_app;

ALTER DEFAULT PRIVILEGES
IN SCHEMA todo_auth, todo_data
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO todo_app;


-- Tables in todo_auth schema
CREATE TABLE IF NOT EXISTS todo_auth.users
(
	user_id uuid DEFAULT gen_random_uuid(),
	username varchar(50) NOT NULL,
	password_hash text,

	email varchar(320),

	created_on timestamptz(0) DEFAULT now() NOT NULL,
    updated_on timestamptz(0) DEFAULT now() NOT NULL,

	PRIMARY KEY (user_id)
);


-- Tables in todo_data schema
CREATE TABLE IF NOT EXISTS todo_data.areas
(
	area_id	uuid DEFAULT gen_random_uuid(),
	area_name citext,

	icon_url text,

	user_id uuid NOT NULL,
    created_on timestamptz(0) DEFAULT now() NOT NULL,
    updated_on timestamptz(0) DEFAULT now() NOT NULL,

	PRIMARY	KEY(area_id),
	FOREIGN	KEY(user_id) REFERENCES todo_auth.users(user_id)
);

CREATE TABLE IF NOT EXISTS todo_data.projects
(
	project_id uuid DEFAULT gen_random_uuid(),
    project_title citext,
    project_notes text,

    start_date date,
	start_time time(0),
	deadline date,

    area_id uuid,

	completed_on timestamptz(0),
	logged_on timestamptz(0),
	trashed_on timestamptz(0),

    user_id	uuid NOT NULL,
	created_on timestamptz(0) DEFAULT now() NOT NULL,
    updated_on timestamptz(0) DEFAULT now() NOT NULL,

	PRIMARY KEY(project_id),
    FOREIGN KEY(area_id) REFERENCES todo_data.areas(area_id),
	FOREIGN	KEY(user_id) REFERENCES todo_auth.users(user_id)
);

CREATE TABLE IF NOT EXISTS todo_data.tasks
(
	task_id uuid DEFAULT gen_random_uuid(),
    task_title citext,
    task_notes text,

    start_date date,
	start_time time(0),
	deadline date,

	project_id uuid,
    area_id uuid,

	completed_on timestamptz(0),
	logged_on timestamptz(0),
	trashed_on timestamptz(0),

    user_id	uuid NOT NULL,
	created_on timestamptz(0) DEFAULT now() NOT NULL,
    updated_on timestamptz(0) DEFAULT now() NOT NULL,

	PRIMARY KEY(task_id),
	FOREIGN KEY(project_id) REFERENCES todo_data.projects(project_id),
	FOREIGN KEY(area_id) REFERENCES todo_data.areas(area_id),
	FOREIGN	KEY(user_id) REFERENCES todo_auth.users(user_id)
);

CREATE TABLE IF NOT EXISTS todo_data.tags
(
	tag_id uuid DEFAULT gen_random_uuid(),
	tag_label citext NOT NULL,
	tag_category varchar(255),

	color varchar(7) CHECK (color IS NULL OR color ~* '^#[a-f0-9]{6}$'),

	user_id uuid NOT NULL,
    created_on timestamptz(0) DEFAULT now() NOT NULL,
    updated_on timestamptz(0) DEFAULT now() NOT NULL,

	PRIMARY KEY(tag_id),
	FOREIGN KEY(user_id) REFERENCES todo_auth.users(user_id)
);

CREATE TABLE IF NOT EXISTS todo_data.project_tags
(
	project_id uuid,
	tag_id uuid,

	PRIMARY KEY(project_id, tag_id),
	FOREIGN KEY(project_id) REFERENCES todo_data.projects(project_id),
	FOREIGN KEY(tag_id) REFERENCES todo_data.tags(tag_id)
);

CREATE TABLE IF NOT EXISTS todo_data.task_tags
(
	task_id uuid,
	tag_id uuid,

	PRIMARY KEY(task_id, tag_id),
	FOREIGN KEY(task_id) REFERENCES todo_data.tasks(task_id),
	FOREIGN KEY(tag_id) REFERENCES todo_data.tags(tag_id)
);
