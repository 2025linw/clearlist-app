/*
This SQL script is a down migration script for Todo List Database
*/

/*
Drop auth and data schemas, and all tables associated with them

TODO: Should add more granular control over the removal
*/
DROP SCHEMA IF EXISTS auth, data CASCADE;

DROP FUNCTION IF EXISTS check_task_tag_user_consistency;
DROP FUNCTION IF EXISTS check_project_tag_user_consistency;
