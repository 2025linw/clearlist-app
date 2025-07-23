use deadpool_postgres::Object;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    models::{area, auth, project, tag, task, user},
    util::{Join, PostgresCmp, SQLQueryBuilder, ToSQLQueryBuilder},
};

// Task
pub async fn create_task(
    conn: &mut Object,
    schema: task::CreateRequest,
    user_id: Uuid,
) -> Result<Uuid> {
    let transaction = conn.transaction().await?;

    let mut builder = schema.to_sql_builder();
    builder.add_column(task::DatabaseModel::USER_ID, &user_id);

    // Insert task
    let (statement, params) = builder.build_insert();

    let task_id: Uuid = transaction
        .query_one(&statement, &params)
        .await?
        .get(task::DatabaseModel::ID);

    // Insert task-tags
    for tag_id in schema.tag_ids() {
        let mut builder = SQLQueryBuilder::new(task::tag::DatabaseModel::TABLE);
        builder.add_column(task::tag::DatabaseModel::TASK_ID, &task_id);
        builder.add_column(task::tag::DatabaseModel::TAG_ID, &tag_id);

        let (statement, params) = builder.build_insert();

        if transaction.execute(&statement, &params).await? != 1 {
            return Err(Error::Internal(String::from(
                "expected only one element to be added",
            )));
        }
    }

    transaction.commit().await?;

    Ok(task_id)
}

pub async fn retrieve_task(
    conn: &Object,
    task_id: Uuid,
    user_id: Uuid,
) -> Result<Option<task::DatabaseModel>> {
    let mut builder = SQLQueryBuilder::new(task::DatabaseModel::TABLE);
    builder.add_condition(task::DatabaseModel::ID, PostgresCmp::Equal, &task_id);
    builder.add_condition(task::DatabaseModel::USER_ID, PostgresCmp::Equal, &user_id);

    let (statement, params) = builder.build_select();

    // Get task
    let mut task: task::DatabaseModel = match conn.query_opt(&statement, &params).await? {
        Some(r) => r.into(),
        None => return Ok(None),
    };

    let mut builder = SQLQueryBuilder::new(task::tag::DatabaseModel::TABLE);
    builder.add_condition(
        task::tag::DatabaseModel::TASK_ID,
        PostgresCmp::Equal,
        task.id_as_ref(),
    );
    builder.add_join(
        Join::Right,
        tag::DatabaseModel::TABLE,
        tag::DatabaseModel::ID,
    );

    let (statement, params) = builder.build_select();

    // Get task-tags
    let tags: Vec<tag::DatabaseModel> = conn
        .query(&statement, &params)
        .await?
        .into_iter()
        .map(|r| r.into())
        .collect();

    task.set_tags(tags);

    Ok(Some(task))
}

pub async fn update_task(
    conn: &mut Object,
    task_id: Uuid,
    user_id: Uuid,
    schema: task::UpdateRequest,
) -> Result<Option<Uuid>> {
    let transaction = conn.transaction().await?;

    let mut builder = schema.to_sql_builder();
    builder.add_condition(task::DatabaseModel::ID, PostgresCmp::Equal, &task_id);
    builder.add_condition(task::DatabaseModel::USER_ID, PostgresCmp::Equal, &user_id);

    let (statement, params) = builder.build_update();

    // Update task
    let task_id: Uuid = match transaction.query_opt(&statement, &params).await? {
        Some(r) => r.get(task::DatabaseModel::ID),
        None => return Ok(None),
    };

    // Update task-tags
    if let Some(tag_ids) = schema.tag_ids() {
        let statement = format!(
            "DELETE FROM {} WHERE {} = $1",
            task::tag::DatabaseModel::TABLE,
            task::tag::DatabaseModel::TASK_ID
        );
        transaction.execute(&statement, &[&task_id]).await?;

        for tag_id in tag_ids {
            let mut builder = SQLQueryBuilder::new(task::tag::DatabaseModel::TABLE);
            builder.add_column(task::tag::DatabaseModel::TASK_ID, &task_id);
            builder.add_column(task::tag::DatabaseModel::TAG_ID, &tag_id);

            let (statement, params) = builder.build_insert();

            if transaction.execute(&statement, &params).await? != 1 {
                return Err(Error::Internal(String::from(
                    "expected only one element to be added",
                )));
            }
        }
    }

    transaction.commit().await?;

    Ok(Some(task_id))
}

pub async fn delete_task(conn: &mut Object, task_id: Uuid, user_id: Uuid) -> Result<Option<()>> {
    let transaction = conn.transaction().await?;

    let mut builder = SQLQueryBuilder::new(task::DatabaseModel::TABLE);
    builder.add_condition(task::DatabaseModel::ID, PostgresCmp::Equal, &task_id);
    builder.add_condition(task::DatabaseModel::USER_ID, PostgresCmp::Equal, &user_id);

    let (statement, params) = builder.build_delete();

    // Delete task
    match transaction.execute(&statement, &params).await? {
        0 => return Ok(None),
        1 => (),
        n => {
            return Err(Error::Internal(format!(
                "unexpected number of tasks removed (expected: 1, actual: {n}), no changes commited"
            )));
        }
    }

    transaction.commit().await?;

    Ok(Some(()))
}

pub async fn query_task(
    conn: &Object,
    user_id: Uuid,
    schema: task::QueryRequest,
    limit: usize,
    offset: usize,
) -> Result<Vec<task::DatabaseModel>> {
    let mut builder = schema.to_sql_builder();
    builder.add_column(task::DatabaseModel::USER_ID, &user_id);
    builder.set_limit(limit);
    builder.set_offset(offset);

    let (statement, params) = schema.to_sql_builder().build_select();

    // Query tasks
    let tasks: Vec<task::DatabaseModel> = conn
        .query(&statement, &params)
        .await?
        .into_iter()
        .map(|r| r.into())
        .collect();

    // Get all task ids with filtered tags
    let temp = schema.tag_ids();
    let num = temp.len() as i64;
    let mut builder = SQLQueryBuilder::new(task::tag::DatabaseModel::TABLE);
    builder.add_condition(task::tag::DatabaseModel::TAG_ID, PostgresCmp::In, &temp);
    builder.set_group_by(&[task::tag::DatabaseModel::TASK_ID]);
    builder.set_having("COUNT(tag_id)", PostgresCmp::Equal, &num);
    builder.set_return(&[task::tag::DatabaseModel::TASK_ID]);

    let (statement, params) = builder.build_select();

    let task_ids: Vec<Uuid> = conn
        .query(&statement, &params)
        .await?
        .into_iter()
        .map(|r| r.get(task::tag::DatabaseModel::TASK_ID))
        .collect();

    // Filter tasks
    let mut tasks: Vec<task::DatabaseModel> = tasks
        .into_iter()
        .filter(|t| task_ids.contains(t.id_as_ref()))
        .collect();

    // Get tags for each task
    for task in tasks.iter_mut() {
        let mut builder = SQLQueryBuilder::new(task::tag::DatabaseModel::TABLE);
        builder.add_condition(
            task::tag::DatabaseModel::TASK_ID,
            PostgresCmp::Equal,
            task.id_as_ref(),
        );
        builder.add_join(
            Join::Right,
            tag::DatabaseModel::TABLE,
            task::tag::DatabaseModel::TAG_ID,
        );

        let (statement, params) = builder.build_select();

        let tags: Vec<tag::DatabaseModel> = conn
            .query(&statement, &params)
            .await?
            .into_iter()
            .map(|r| r.into())
            .collect();

        task.set_tags(tags);
    }

    Ok(tasks)
}

// Project
pub async fn create_project(conn: &mut Object, schema: project::CreateRequest) -> Result<Uuid> {
    let transaction = conn.transaction().await?;

    // Insert project
    let (statement, params) = schema.to_sql_builder().build_insert();

    let project_id: Uuid = transaction
        .query_one(&statement, &params)
        .await?
        .get(project::DatabaseModel::ID);

    // Insert project-tags
    for tag_id in schema.tag_ids() {
        let mut builder = SQLQueryBuilder::new(project::tag::DatabaseModel::TABLE);
        builder.add_column(project::tag::DatabaseModel::PROJECT_ID, &project_id);
        builder.add_column(project::tag::DatabaseModel::TAG_ID, tag_id);

        let (statement, params) = builder.build_insert();

        if transaction.execute(&statement, &params).await? != 1 {
            return Err(Error::Internal(String::from(
                "expected only one element to be added",
            )));
        }
    }

    transaction.commit().await?;

    Ok(project_id)
}

pub async fn retrieve_project(
    conn: &Object,
    schema: project::RetrieveRequest,
) -> Result<Option<project::DatabaseModel>> {
    // Get project
    let (statement, params) = schema.to_sql_builder().build_select();

    let mut project: project::DatabaseModel = match conn.query_opt(&statement, &params).await? {
        Some(r) => r.into(),
        None => return Ok(None),
    };

    // Get project-tags
    let mut builder = SQLQueryBuilder::new(project::tag::DatabaseModel::TABLE);
    builder.add_condition(
        project::tag::DatabaseModel::PROJECT_ID,
        PostgresCmp::Equal,
        project.id_as_ref(),
    );
    builder.add_join(
        Join::Right,
        tag::DatabaseModel::TABLE,
        tag::DatabaseModel::ID,
    );

    let (statement, params) = builder.build_select();

    let tags: Vec<tag::DatabaseModel> = conn
        .query(&statement, &params)
        .await?
        .into_iter()
        .map(|r| r.into())
        .collect();

    project.set_tags(tags);

    Ok(Some(project))
}

pub async fn update_project(
    conn: &mut Object,
    schema: project::UpdateRequest,
) -> Result<Option<Uuid>> {
    let transaction = conn.transaction().await?;

    // Update project
    let (statement, params) = schema.to_sql_builder().build_update();

    let project_id: Uuid = match transaction.query_opt(&statement, &params).await? {
        Some(r) => r.get(project::DatabaseModel::ID),
        None => return Ok(None),
    };

    // Update project-tags
    if let Some(tag_ids) = schema.tag_ids() {
        let statement = format!(
            "DELETE FROM {} WHERE {} = $1",
            project::tag::DatabaseModel::TABLE,
            project::tag::DatabaseModel::PROJECT_ID,
        );
        transaction.execute(&statement, &[&project_id]).await?;

        for tag_id in tag_ids {
            let mut builder = SQLQueryBuilder::new(project::tag::DatabaseModel::TABLE);
            builder.add_column(project::tag::DatabaseModel::PROJECT_ID, &project_id);
            builder.add_column(project::tag::DatabaseModel::TAG_ID, &tag_id);

            let (statement, params) = builder.build_insert();

            if transaction.execute(&statement, &params).await? != 1 {
                return Err(Error::Internal(String::from(
                    "expected only one element to be added",
                )));
            }
        }
    }

    transaction.commit().await?;

    Ok(Some(project_id))
}

pub async fn delete_project(
    conn: &mut Object,
    schema: project::DeleteRequest,
) -> Result<Option<()>> {
    let transaction = conn.transaction().await?;

    // Delete project
    let (statement, params) = schema.to_sql_builder().build_delete();

    match transaction.execute(&statement, &params).await? {
        0 => return Ok(None),
        1 => (),
        n => {
            return Err(Error::Internal(format!(
                "unexpected number of projects removed (expected: 1, actual: {n}), no changes commited"
            )));
        }
    }

    transaction.commit().await?;

    Ok(Some(()))
}

pub async fn query_project(
    conn: &Object,
    schema: project::QueryRequest,
) -> Result<Vec<project::DatabaseModel>> {
    // Query projects
    let (statement, params) = schema.to_sql_builder().build_select();

    let projects: Vec<project::DatabaseModel> = conn
        .query(&statement, &params)
        .await?
        .into_iter()
        .map(|r| r.into())
        .collect();

    // Get all project ids with filtered tags
    let temp = schema.tag_ids();
    let num = temp.len() as i64;
    let mut builder = SQLQueryBuilder::new(project::tag::DatabaseModel::TABLE);
    builder.add_condition(project::tag::DatabaseModel::TAG_ID, PostgresCmp::In, &temp);
    builder.set_group_by(&[project::tag::DatabaseModel::PROJECT_ID]);
    builder.set_having("COUNT(tag_id)", PostgresCmp::Equal, &num);
    builder.set_return(&[project::tag::DatabaseModel::PROJECT_ID]);

    let (statement, params) = builder.build_select();

    let project_ids: Vec<Uuid> = conn
        .query(&statement, &params)
        .await?
        .into_iter()
        .map(|r| r.get(project::tag::DatabaseModel::PROJECT_ID))
        .collect();

    // Filter projects
    let mut projects: Vec<project::DatabaseModel> = projects
        .into_iter()
        .filter(|p| project_ids.contains(p.id_as_ref()))
        .collect();

    // Get tags for each project
    for project in projects.iter_mut() {
        let mut builder = SQLQueryBuilder::new(project::tag::DatabaseModel::TABLE);
        builder.add_condition(
            project::tag::DatabaseModel::PROJECT_ID,
            PostgresCmp::Equal,
            project.id_as_ref(),
        );
        builder.add_join(
            Join::Right,
            tag::DatabaseModel::TABLE,
            project::tag::DatabaseModel::TAG_ID,
        );

        let (statement, params) = builder.build_select();

        let tags: Vec<tag::DatabaseModel> = conn
            .query(&statement, &params)
            .await?
            .into_iter()
            .map(|r| r.into())
            .collect();

        project.set_tags(tags);
    }

    Ok(projects)
}

// Area
pub async fn create_area(conn: &mut Object, schema: area::CreateRequest) -> Result<Uuid> {
    let transaction = conn.transaction().await?;

    // Insert area
    let (statement, params) = schema.to_sql_builder().build_insert();

    let area_id: Uuid = transaction
        .query_one(&statement, &params)
        .await?
        .get(area::DatabaseModel::ID);

    transaction.commit().await?;

    Ok(area_id)
}

pub async fn retrieve_area(
    conn: &Object,
    schema: area::RetrieveRequest,
) -> Result<Option<area::DatabaseModel>> {
    // Get area
    let (statement, params) = schema.to_sql_builder().build_select();

    let area: area::DatabaseModel = match conn.query_opt(&statement, &params).await? {
        Some(r) => r.into(),
        None => return Ok(None),
    };

    Ok(Some(area))
}

pub async fn update_area(conn: &mut Object, schema: area::UpdateRequest) -> Result<Option<Uuid>> {
    let transaction = conn.transaction().await?;

    // Update area
    let (statement, params) = schema.to_sql_builder().build_update();

    let area_id: Uuid = match transaction.query_opt(&statement, &params).await? {
        Some(r) => r.get(area::DatabaseModel::ID),
        None => return Ok(None),
    };

    transaction.commit().await?;

    Ok(Some(area_id))
}

pub async fn delete_area(conn: &mut Object, schema: area::DeleteRequest) -> Result<Option<()>> {
    let transaction = conn.transaction().await?;

    // Delete area
    let (statement, params) = schema.to_sql_builder().build_delete();

    match transaction.execute(&statement, &params).await? {
        0 => return Ok(None),
        1 => (),
        n => {
            return Err(Error::Internal(format!(
                "unexpected number of areas removed (expected: 1, actual: {n}), no changes commited"
            )));
        }
    }

    transaction.commit().await?;

    Ok(Some(()))
}

pub async fn query_area(
    conn: &Object,
    schema: area::QueryRequest,
) -> Result<Vec<area::DatabaseModel>> {
    // Query areas
    let (statement, params) = schema.to_sql_builder().build_select();

    let areas: Vec<area::DatabaseModel> = conn
        .query(&statement, &params)
        .await?
        .into_iter()
        .map(|r| r.into())
        .collect();

    Ok(areas)
}

// Tag
pub async fn create_tag(conn: &mut Object, schema: tag::CreateRequest) -> Result<Uuid> {
    let transaction = conn.transaction().await?;

    // Insert tag
    let (statement, params) = schema.to_sql_builder().build_insert();

    let tag_id: Uuid = transaction
        .query_one(&statement, &params)
        .await?
        .get(tag::DatabaseModel::ID);

    transaction.commit().await?;

    Ok(tag_id)
}

pub async fn retrieve_tag(
    conn: &Object,
    schema: tag::RetrieveRequest,
) -> Result<Option<tag::DatabaseModel>> {
    // Get tag
    let (statement, params) = schema.to_sql_builder().build_select();

    let tag: tag::DatabaseModel = match conn.query_opt(&statement, &params).await? {
        Some(r) => r.into(),
        None => return Ok(None),
    };

    Ok(Some(tag))
}

pub async fn update_tag(conn: &mut Object, schema: tag::UpdateRequest) -> Result<Option<Uuid>> {
    let transaction = conn.transaction().await?;

    // Update tag
    let (statement, params) = schema.to_sql_builder().build_update();

    let tag_id: Uuid = match transaction.query_opt(&statement, &params).await? {
        Some(r) => r.get(tag::DatabaseModel::ID),
        None => return Ok(None),
    };

    transaction.commit().await?;

    Ok(Some(tag_id))
}

pub async fn delete_tag(conn: &mut Object, schema: tag::DeleteRequest) -> Result<Option<()>> {
    let transaction = conn.transaction().await?;

    // Delete area
    let (statement, params) = schema.to_sql_builder().build_delete();

    match transaction.execute(&statement, &params).await? {
        0 => return Ok(None),
        1 => (),
        n => {
            return Err(Error::Internal(format!(
                "unexpected number of tags removed (expected: 1, actual: {n}), no changes commited"
            )));
        }
    }

    transaction.commit().await?;

    Ok(Some(()))
}

pub async fn query_tag(
    conn: &Object,
    schema: tag::QueryRequest,
) -> Result<Vec<tag::DatabaseModel>> {
    // Query tags
    let (statement, params) = schema.to_sql_builder().build_select();

    let tags: Vec<tag::DatabaseModel> = conn
        .query(&statement, &params)
        .await?
        .into_iter()
        .map(|r| r.into())
        .collect();

    Ok(tags)
}

// Auth
pub async fn check_for_email(conn: &Object, email: &str) -> Result<bool> {
    let mut builder = SQLQueryBuilder::new(user::DatabaseModel::TABLE);
    builder.add_condition(user::DatabaseModel::EMAIL, PostgresCmp::Equal, &email);
    builder.set_return(&[user::DatabaseModel::EMAIL]);

    let (statement, params) = builder.build_select();

    match conn.query_opt(&statement, &params).await? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub async fn check_for_user(conn: &Object, user_id: Uuid) -> Result<bool> {
    let mut builder = SQLQueryBuilder::new(user::DatabaseModel::TABLE);
    builder.add_condition(user::DatabaseModel::ID, PostgresCmp::Equal, &user_id);
    builder.set_return(&[user::DatabaseModel::EMAIL]);

    let (statement, params) = builder.build_select();

    match conn.query_opt(&statement, &params).await? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub async fn register_user(conn: &mut Object, schema: auth::LoginInfo) -> Result<()> {
    let transaction = conn.transaction().await?;

    let (statement, params) = schema.to_sql_builder().build_insert();

    transaction.query_one(&statement, &params).await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn get_login_info(conn: &Object, email: &str) -> Result<auth::LoginInfo> {
    let mut builder = SQLQueryBuilder::new(user::DatabaseModel::TABLE);
    builder.add_condition(user::DatabaseModel::EMAIL, PostgresCmp::Equal, &email);
    builder.set_return(&[
        user::DatabaseModel::ID,
        user::DatabaseModel::EMAIL,
        user::DatabaseModel::PASS_HASH,
    ]);

    let (statement, params) = builder.build_select();

    // let login: auth::LoginInfo = match conn.query_opt(&statement, &params).await? {
    //     Some(r) => r.into(),
    //     None => return Ok(None),
    // };

    Ok(conn.query_one(&statement, &params).await?.into())
}

// User
// TODO: decide whether this is important

// pub async fn retrieve_user(conn: &Object, user_id: Uuid) -> Result<Option<user::DatabaseModel>> {
//     let mut builder = SQLQueryBuilder::new(user::DatabaseModel::TABLE);
//     builder.add_condition(user::DatabaseModel::ID, PostgresCmp::Equal, &user_id);

//     let (statement, params) = builder.build_select();

//     let user: user::DatabaseModel = match conn.query_opt(&statement, &params).await? {
//         Some(r) => r.into(),
//         None => return Ok(None),
//     };

//     Ok(Some(user))
// }

// pub async fn update_user(conn: &mut Object, user_id: Uuid) -> Result<Option<Uuid>> {
//     todo!()
// }

// pub async fn delete_user(conn: &mut Object, user_id: Uuid) -> Result<Option<()>> {
//     todo!()
// }
