-- Drop Schemas
DROP SCHEMA IF EXISTS auth, data CASCADE;
DROP EXTENSION IF EXISTS citext;


-- Add citext module
CREATE EXTENSION citext;


-- Create schemas and add permissions
CREATE SCHEMA auth;
GRANT USAGE ON SCHEMA auth TO todo_app;
ALTER DEFAULT PRIVILEGES
IN SCHEMA auth
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO todo_app;

CREATE SCHEMA data;
GRANT USAGE ON SCHEMA data TO todo_app;
ALTER DEFAULT PRIVILEGES
IN SCHEMA data
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO todo_app;


-- Tables in auth schema
CREATE TABLE IF NOT EXISTS auth.users
(
	user_id uuid DEFAULT gen_random_uuid(),

	username varchar(50) NOT NULL,
	password_hash text,
	email varchar(320),

	created_on timestamptz(0) DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_on timestamptz(0) DEFAULT CURRENT_TIMESTAMP NOT NULL,

	PRIMARY KEY (user_id)
);


-- Tables in data schema
CREATE TABLE IF NOT EXISTS data.areas
(
	area_id	uuid DEFAULT gen_random_uuid(),

	area_name citext,
	icon_url text,

	user_id uuid NOT NULL,
    created_on timestamptz(0) DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_on timestamptz(0) DEFAULT CURRENT_TIMESTAMP NOT NULL,

	PRIMARY	KEY(area_id),
	FOREIGN	KEY(user_id) REFERENCES auth.users(user_id)
);

CREATE TABLE IF NOT EXISTS data.projects
(
	project_id uuid DEFAULT gen_random_uuid(),

    project_title citext,
    project_notes text,
    start_date date,
	start_time time(0),
	deadline date,
	completed_on timestamptz(0),
	logged_on timestamptz(0),
	trashed_on timestamptz(0),

    area_id uuid,

    user_id	uuid NOT NULL,
	created_on timestamptz(0) DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_on timestamptz(0) DEFAULT CURRENT_TIMESTAMP NOT NULL,

	PRIMARY KEY(project_id),
    FOREIGN KEY(area_id) REFERENCES data.areas(area_id),
	FOREIGN	KEY(user_id) REFERENCES auth.users(user_id)
);

CREATE TABLE IF NOT EXISTS data.tasks
(
	task_id uuid DEFAULT gen_random_uuid(),

    task_title citext,
    task_notes text,
    start_date date,
	start_time time(0),
	deadline date,
	completed_on timestamptz(0),
	logged_on timestamptz(0),
	trashed_on timestamptz(0),

    area_id uuid,
	project_id uuid,

    user_id	uuid NOT NULL,
	created_on timestamptz(0) DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_on timestamptz(0) DEFAULT CURRENT_TIMESTAMP NOT NULL,

	PRIMARY KEY(task_id),
	FOREIGN KEY(area_id) REFERENCES data.areas(area_id),
	FOREIGN KEY(project_id) REFERENCES data.projects(project_id),
	FOREIGN	KEY(user_id) REFERENCES auth.users(user_id)
);

CREATE TABLE IF NOT EXISTS data.tags
(
	tag_id uuid DEFAULT gen_random_uuid(),

	tag_label citext NOT NULL,
	tag_category varchar(255),
	tag_color varchar(7) CHECK (tag_color IS NULL OR tag_color ~* '^#[a-f0-9]{6}$'),

	user_id uuid NOT NULL,
    created_on timestamptz(0) DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_on timestamptz(0) DEFAULT CURRENT_TIMESTAMP NOT NULL,

	PRIMARY KEY(tag_id),
	FOREIGN KEY(user_id) REFERENCES auth.users(user_id)
);

CREATE TABLE IF NOT EXISTS data.project_tags
(
	project_id uuid,
	tag_id uuid,

	PRIMARY KEY(project_id, tag_id),
	FOREIGN KEY(project_id) REFERENCES data.projects(project_id) ON DELETE CASCADE,
	FOREIGN KEY(tag_id) REFERENCES data.tags(tag_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS data.task_tags
(
	task_id uuid,
	tag_id uuid,

	PRIMARY KEY(task_id, tag_id),
	FOREIGN KEY(task_id) REFERENCES data.tasks(task_id) ON DELETE CASCADE,
	FOREIGN KEY(tag_id) REFERENCES data.tags(tag_id) ON DELETE CASCADE
);
