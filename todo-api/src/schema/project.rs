use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    model::project::ProjectModel,
    util::{AddToQuery, NULL, PostgresCmp, SQLQueryBuilder, ToPostgresCmp},
};

use super::{QueryMethod, UpdateMethod};

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectSchema {
    title: Option<String>,
    notes: Option<String>,
    start_date: Option<NaiveDate>,
    start_time: Option<NaiveTime>,
    deadline: Option<NaiveDate>,

    area_id: Option<Uuid>,
}

impl<'a, 'b> AddToQuery<'a, 'b> for CreateProjectSchema {
    fn add_to_query(&'a self, builder: &'b mut SQLQueryBuilder<'a>) {
        builder.set_table(ProjectModel::TABLE);

        if let Some(ref s) = self.title {
            builder.add_column(ProjectModel::TITLE, s);
        }
        if let Some(ref s) = self.notes {
            builder.add_column(ProjectModel::NOTES, s);
        }
        if let Some(ref d) = self.start_date {
            builder.add_column(ProjectModel::START_DATE, d);
        }
        if let Some(ref t) = self.start_time {
            builder.add_column(ProjectModel::START_TIME, t);
        }
        if let Some(ref d) = self.deadline {
            builder.add_column(ProjectModel::DEADLINE, d);
        }

        if let Some(ref i) = self.area_id {
            builder.add_column(ProjectModel::AREA_ID, i);
        }
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectSchema {
    title: Option<UpdateMethod<String>>,
    notes: Option<UpdateMethod<String>>,
    start_date: Option<UpdateMethod<NaiveDate>>,
    start_time: Option<UpdateMethod<NaiveTime>>,
    deadline: Option<UpdateMethod<NaiveDate>>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,

    area_id: Option<UpdateMethod<Uuid>>,
}

impl<'a, 'b> AddToQuery<'a, 'b> for UpdateProjectSchema {
    fn add_to_query(&'a self, builder: &'b mut SQLQueryBuilder<'a>) {
        builder.set_table(ProjectModel::TABLE);

        if let Some(ref u) = self.title {
            if matches!(u, UpdateMethod::Remove(true) | UpdateMethod::Change(..)) {
                builder.add_column(ProjectModel::TITLE, u);
            }
        }
        if let Some(ref u) = self.notes {
            if matches!(u, UpdateMethod::Remove(true) | UpdateMethod::Change(..)) {
                builder.add_column(ProjectModel::NOTES, u);
            }
        }
        if let Some(ref u) = self.start_date {
            if matches!(u, UpdateMethod::Remove(true) | UpdateMethod::Change(..)) {
                builder.add_column(ProjectModel::START_DATE, u);
            }
        }
        if let Some(ref u) = self.start_time {
            if matches!(u, UpdateMethod::Remove(true) | UpdateMethod::Change(..)) {
                builder.add_column(ProjectModel::START_TIME, u);
            }
        }
        if let Some(ref u) = self.deadline {
            if matches!(u, UpdateMethod::Remove(true) | UpdateMethod::Change(..)) {
                builder.add_column(ProjectModel::DEADLINE, u);
            }
        }

        if let Some(b) = self.completed {
            if b {
                builder.add_column(
                    ProjectModel::COMPLETED,
                    builder.get_column(0).unwrap().to_owned(),
                );
            } else {
                builder.add_column(ProjectModel::COMPLETED, &None::<DateTime<Local>>); // TODO: do some ToSql fandangling to fix this
            }
        }
        if let Some(b) = self.logged {
            if b {
                builder.add_column(
                    ProjectModel::LOGGED,
                    builder.get_column(0).unwrap().to_owned(),
                )
            } else {
                builder.add_column(ProjectModel::LOGGED, &None::<DateTime<Local>>);
            }
        }
        if let Some(b) = self.trashed {
            if b {
                builder.add_column(
                    ProjectModel::TRASHED,
                    builder.get_column(0).unwrap().to_owned(),
                )
            } else {
                builder.add_column(ProjectModel::TRASHED, &None::<DateTime<Local>>);
            }
        }

        if let Some(ref u) = self.area_id {
            if matches!(u, UpdateMethod::Remove(true) | UpdateMethod::Change(..)) {
                builder.add_column(ProjectModel::AREA_ID, u);
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct QueryProjectSchema {
    title: Option<QueryMethod<String>>,
    notes: Option<QueryMethod<String>>,
    start_date: Option<QueryMethod<NaiveDate>>,
    start_time: Option<QueryMethod<NaiveTime>>,
    deadline: Option<QueryMethod<NaiveDate>>,

    completed: Option<bool>,
    logged: Option<bool>,
    trashed: Option<bool>,

    area_id: Option<QueryMethod<Uuid>>,
}

impl<'a, 'b> AddToQuery<'a, 'b> for QueryProjectSchema {
    fn add_to_query(&'a self, builder: &'b mut SQLQueryBuilder<'a>) {
        builder.set_table(ProjectModel::TABLE);

        if let Some(ref q) = self.title {
            let cmp;
            match q {
                QueryMethod::NotNull(b) => {
                    if *b {
                        cmp = PostgresCmp::NotNull;
                    } else {
                        cmp = PostgresCmp::IsNull;
                    }
                }
                QueryMethod::Match(_) => cmp = PostgresCmp::Like,
                QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
            }
            builder.add_condition(ProjectModel::TITLE, cmp, q);
        }
        if let Some(ref q) = self.notes {
            let cmp;
            match q {
                QueryMethod::NotNull(b) => {
                    if *b {
                        cmp = PostgresCmp::NotNull;
                    } else {
                        cmp = PostgresCmp::IsNull;
                    }
                }
                QueryMethod::Match(_) => cmp = PostgresCmp::Like,
                QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
            }
            builder.add_condition(ProjectModel::NOTES, cmp, q);
        }
        if let Some(ref q) = self.start_date {
            let cmp;
            match q {
                QueryMethod::NotNull(b) => {
                    if *b {
                        cmp = PostgresCmp::NotNull;
                    } else {
                        cmp = PostgresCmp::IsNull;
                    }
                }
                QueryMethod::Match(_) => cmp = PostgresCmp::Equal,
                QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
            }
            builder.add_condition(ProjectModel::START_DATE, cmp, q);
        }
        if let Some(ref q) = self.start_time {
            let cmp;
            match q {
                QueryMethod::NotNull(b) => {
                    if *b {
                        cmp = PostgresCmp::NotNull;
                    } else {
                        cmp = PostgresCmp::IsNull;
                    }
                }
                QueryMethod::Match(_) => cmp = PostgresCmp::Equal,
                QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
            }
            builder.add_condition(ProjectModel::START_TIME, cmp, q);
        }
        if let Some(ref q) = self.deadline {
            let cmp;
            match q {
                QueryMethod::NotNull(b) => {
                    if *b {
                        cmp = PostgresCmp::NotNull;
                    } else {
                        cmp = PostgresCmp::IsNull;
                    }
                }
                QueryMethod::Match(_) => cmp = PostgresCmp::Equal,
                QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
            }
            builder.add_condition(ProjectModel::DEADLINE, cmp, q);
        }

        if let Some(b) = self.completed {
            if b {
                builder.add_condition(ProjectModel::COMPLETED, PostgresCmp::NotNull, &NULL);
            } else {
                builder.add_condition(ProjectModel::COMPLETED, PostgresCmp::IsNull, &NULL);
            }
        }
        if let Some(b) = self.logged {
            if b {
                builder.add_condition(ProjectModel::LOGGED, PostgresCmp::NotNull, &NULL);
            } else {
                builder.add_condition(ProjectModel::LOGGED, PostgresCmp::IsNull, &NULL);
            }
        }
        if let Some(b) = self.trashed {
            if b {
                builder.add_condition(ProjectModel::TRASHED, PostgresCmp::NotNull, &NULL);
            } else {
                builder.add_condition(ProjectModel::TRASHED, PostgresCmp::IsNull, &NULL);
            }
        }

        if let Some(ref q) = self.area_id {
            let cmp;
            match q {
                QueryMethod::NotNull(b) => {
                    if *b {
                        cmp = PostgresCmp::NotNull;
                    } else {
                        cmp = PostgresCmp::IsNull;
                    }
                }
                QueryMethod::Match(_) => cmp = PostgresCmp::Equal,
                QueryMethod::Compare(_, c) => cmp = c.to_postgres_cmp(),
            }
            builder.add_condition(ProjectModel::AREA_ID, cmp, q);
        }
    }
}

#[cfg(test)]
mod create_schema_test {
    use chrono::Local;
    use uuid::Uuid;

    use crate::{
        model::project::ProjectModel,
        util::{AddToQuery, SQLQueryBuilder},
    };

    use super::CreateProjectSchema;

    const ID: Uuid = Uuid::nil();

    #[test]
    fn empty() {
        let schema = CreateProjectSchema::default();

        let mut builder = SQLQueryBuilder::new();
        builder.add_column(ProjectModel::USER_ID, &ID);
        schema.add_to_query(&mut builder);

        let (statement, params) = builder.build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.projects (user_id) VALUES ($1)"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn text_only() {
        let mut schema = CreateProjectSchema::default();
        schema.title = Some("Test Title".to_string());
        schema.notes = Some("Test Note".to_string());

        let mut builder = SQLQueryBuilder::new();
        builder.add_column(ProjectModel::USER_ID, &ID);
        schema.add_to_query(&mut builder);

        let (statement, params) = builder.build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.projects (user_id, project_title, notes) VALUES ($1, $2, $3)"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn date_time_only() {
        let now = Local::now();

        let mut schema = CreateProjectSchema::default();
        schema.start_date = Some(now.date_naive());
        schema.start_time = Some(now.time());
        schema.deadline = Some(now.date_naive());

        let mut builder = SQLQueryBuilder::new();
        builder.add_column(ProjectModel::USER_ID, &ID);
        schema.add_to_query(&mut builder);

        let (statement, params) = builder.build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.projects (user_id, start_date, start_time, deadline) VALUES ($1, $2, $3, $4)"
        );
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn id_only() {
        let mut schema = CreateProjectSchema::default();
        schema.area_id = Some(Uuid::new_v4());

        let mut builder = SQLQueryBuilder::new();
        builder.add_column(ProjectModel::USER_ID, &ID);
        schema.add_to_query(&mut builder);

        let (statement, params) = builder.build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.projects (user_id, area_id) VALUES ($1, $2)"
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn full() {
        let now = Local::now();

        let mut schema = CreateProjectSchema::default();
        schema.title = Some("Test Title".to_string());
        schema.notes = Some("Test Note".to_string());
        schema.start_date = Some(now.date_naive());
        schema.start_time = Some(now.time());
        schema.deadline = Some(now.date_naive());
        schema.area_id = Some(Uuid::new_v4());

        let mut builder = SQLQueryBuilder::new();
        builder.add_column(ProjectModel::USER_ID, &ID);
        schema.add_to_query(&mut builder);

        let (statement, params) = builder.build_insert();

        assert_eq!(
            statement,
            "INSERT INTO data.projects (user_id, project_title, notes, start_date, start_time, deadline, area_id) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        );
        assert_eq!(params.len(), 7);
    }

    #[test]
    fn return_some() {
        let now = Local::now();

        let mut schema = CreateProjectSchema::default();
        schema.title = Some("Test Title".to_string());
        schema.start_date = Some(now.date_naive());
        schema.deadline = Some(now.date_naive());
        schema.area_id = Some(Uuid::new_v4());

        let mut builder = SQLQueryBuilder::new();
        builder.add_column(ProjectModel::USER_ID, &ID);
        schema.add_to_query(&mut builder);
        builder.set_return(vec![ProjectModel::ID]);

        let (statement, params) = builder.build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.projects (user_id, project_title, start_date, deadline, area_id) VALUES ($1, $2, $3, $4, $5) RETURNING project_id"
        );
        assert_eq!(params.len(), 5);
    }

    #[test]
    fn return_all() {
        let now = Local::now();

        let mut schema = CreateProjectSchema::default();
        schema.title = Some("Test Title".to_string());
        schema.start_date = Some(now.date_naive());
        schema.deadline = Some(now.date_naive());
        schema.area_id = Some(Uuid::new_v4());

        let mut builder = SQLQueryBuilder::new();
        builder.add_column(ProjectModel::USER_ID, &ID);
        schema.add_to_query(&mut builder);
        builder.set_return_all();

        let (statement, params) = builder.build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO data.projects (user_id, project_title, start_date, deadline, area_id) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        );
        assert_eq!(params.len(), 5);
    }

    // TODO: make production examples
}

#[cfg(test)]
mod update_schema_test {
    use chrono::Local;
    use uuid::Uuid;

    use crate::{
        model::project::ProjectModel,
        schema::UpdateMethod,
        util::{AddToQuery, PostgresCmp, SQLQueryBuilder},
    };

    use super::UpdateProjectSchema;

    const ID: Uuid = Uuid::nil();

    #[test]
    fn text_only() {
        let mut schema = UpdateProjectSchema::default();
        schema.title = Some(UpdateMethod::Change("Test Title".to_string()));
        schema.notes = Some(UpdateMethod::Change("Test Note".to_string()));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.add_condition(ProjectModel::ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.projects SET project_title=$1, notes=$2 WHERE user_id = $3 AND project_id = $4"
        );
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn date_time_only() {
        let now = Local::now();

        let mut schema = UpdateProjectSchema::default();
        schema.start_date = Some(UpdateMethod::Change(now.date_naive()));
        schema.start_time = Some(UpdateMethod::Change(now.time()));
        schema.deadline = Some(UpdateMethod::Change(now.date_naive()));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.add_condition(ProjectModel::ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.projects SET start_date=$1, start_time=$2, deadline=$3 WHERE user_id = $4 AND project_id = $5"
        );
        assert_eq!(params.len(), 5);
    }

    #[test]
    fn bool_only() {
        let now = Local::now();

        let mut schema = UpdateProjectSchema::default();
        schema.completed = Some(true);
        schema.logged = Some(true);
        schema.trashed = Some(true);

        let mut builder = SQLQueryBuilder::new();
        builder.add_column(ProjectModel::UPDATED, &now);
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.add_condition(ProjectModel::ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.projects SET updated_on=$1, completed_on=$2, logged_on=$3, trashed_on=$4 WHERE user_id = $5 AND project_id = $6"
        );
        assert_eq!(params.len(), 6);
    }

    #[test]
    fn id_only() {
        let mut schema = UpdateProjectSchema::default();
        schema.area_id = Some(UpdateMethod::Change(Uuid::new_v4()));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.add_condition(ProjectModel::ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.projects SET area_id=$1 WHERE user_id = $2 AND project_id = $3"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn full() {
        let now = Local::now();

        let mut schema = UpdateProjectSchema::default();
        schema.title = Some(UpdateMethod::Change("Test Title".to_string()));
        schema.notes = Some(UpdateMethod::Change("Test Note".to_string()));
        schema.start_date = Some(UpdateMethod::Change(now.date_naive()));
        schema.start_time = Some(UpdateMethod::Change(now.time()));
        schema.deadline = Some(UpdateMethod::Change(now.date_naive()));
        schema.completed = Some(true);
        schema.logged = Some(true);
        schema.trashed = Some(true);
        schema.area_id = Some(UpdateMethod::Change(Uuid::new_v4()));

        let mut builder = SQLQueryBuilder::new();
        builder.add_column(ProjectModel::UPDATED, &now);
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.add_condition(ProjectModel::ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.projects SET updated_on=$1, project_title=$2, notes=$3, start_date=$4, start_time=$5, deadline=$6, completed_on=$7, logged_on=$8, trashed_on=$9, area_id=$10 WHERE user_id = $11 AND project_id = $12"
        );
        assert_eq!(params.len(), 12);
    }

    #[test]
    fn return_some() {
        let now = Local::now();

        let mut schema = UpdateProjectSchema::default();
        schema.title = Some(UpdateMethod::Change("Test Title".to_string()));
        schema.start_date = Some(UpdateMethod::Change(now.date_naive()));
        schema.deadline = Some(UpdateMethod::Change(now.date_naive()));
        schema.completed = Some(true);
        schema.area_id = Some(UpdateMethod::Change(Uuid::new_v4()));

        let mut builder = SQLQueryBuilder::new();
        builder.add_column(ProjectModel::UPDATED, &now);
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.add_condition(ProjectModel::ID, PostgresCmp::Equal, &ID);
        builder.set_return(vec![ProjectModel::ID]);

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.projects SET updated_on=$1, project_title=$2, start_date=$3, deadline=$4, completed_on=$5, area_id=$6 WHERE user_id = $7 AND project_id = $8 RETURNING project_id"
        );
        assert_eq!(params.len(), 8);
    }

    #[test]
    fn return_all() {
        let now = Local::now();

        let mut schema = UpdateProjectSchema::default();
        schema.title = Some(UpdateMethod::Change("Test Title".to_string()));
        schema.start_date = Some(UpdateMethod::Change(now.date_naive()));
        schema.deadline = Some(UpdateMethod::Change(now.date_naive()));
        schema.completed = Some(true);
        schema.area_id = Some(UpdateMethod::Change(Uuid::new_v4()));

        let mut builder = SQLQueryBuilder::new();
        builder.add_column(ProjectModel::UPDATED, &now);
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.add_condition(ProjectModel::ID, PostgresCmp::Equal, &ID);
        builder.set_return_all();

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE data.projects SET updated_on=$1, project_title=$2, start_date=$3, deadline=$4, completed_on=$5, area_id=$6 WHERE user_id = $7 AND project_id = $8 RETURNING *"
        );
        assert_eq!(params.len(), 8);
    }

    // TODO: make production example
}

#[cfg(test)]
mod query_schema_test {
    use chrono::Local;
    use uuid::Uuid;

    use crate::{
        model::project::ProjectModel,
        schema::{Compare, QueryMethod},
        util::{AddToQuery, PostgresCmp, SQLQueryBuilder},
    };

    use super::QueryProjectSchema;

    const ID: Uuid = Uuid::nil();

    #[test]
    fn empty() {
        let schema = QueryProjectSchema::default();

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE user_id = $1"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn text_only() {
        let mut schema = QueryProjectSchema::default();
        schema.title = Some(QueryMethod::Match("Test Title".to_string()));
        schema.notes = Some(QueryMethod::Match("Test Note".to_string()));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE project_title LIKE '%' || $1 || '%' AND notes LIKE '%' || $2 || '%' AND user_id = $3"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn date_time_eq_only() {
        let now = Local::now();

        let mut schema = QueryProjectSchema::default();
        schema.start_date = Some(QueryMethod::Match(now.date_naive()));
        schema.start_time = Some(QueryMethod::Match(now.time()));
        schema.deadline = Some(QueryMethod::Match(now.date_naive()));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE start_date = $1 AND start_time = $2 AND deadline = $3 AND user_id = $4"
        );
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn date_time_cmp_only() {
        let now = Local::now();

        let mut schema = QueryProjectSchema::default();
        schema.start_date = Some(QueryMethod::Compare(now.date_naive(), Compare::Less));
        schema.start_time = Some(QueryMethod::Compare(now.time(), Compare::LessEq));
        schema.deadline = Some(QueryMethod::Compare(now.date_naive(), Compare::Greater));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE start_date < $1 AND start_time <= $2 AND deadline > $3 AND user_id = $4"
        );
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn bool_t_only() {
        let mut schema = QueryProjectSchema::default();
        schema.completed = Some(true);
        schema.logged = Some(true);
        schema.trashed = Some(true);

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE completed_on NOTNULL AND logged_on NOTNULL AND trashed_on NOTNULL AND user_id = $1"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn bool_f_only() {
        let mut schema = QueryProjectSchema::default();
        schema.completed = Some(false);
        schema.logged = Some(false);
        schema.trashed = Some(false);

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE completed_on ISNULL AND logged_on ISNULL AND trashed_on ISNULL AND user_id = $1"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn id_only() {
        let mut schema = QueryProjectSchema::default();
        schema.area_id = Some(QueryMethod::Match(Uuid::new_v4()));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE area_id = $1 AND user_id = $2"
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn full() {
        let now = Local::now();

        let mut schema = QueryProjectSchema::default();
        schema.title = Some(QueryMethod::Match("Test Title".to_string()));
        schema.notes = Some(QueryMethod::Match("Test Note".to_string()));
        schema.start_date = Some(QueryMethod::Match(now.date_naive()));
        schema.start_time = Some(QueryMethod::Match(now.time()));
        schema.deadline = Some(QueryMethod::Compare(now.date_naive(), Compare::Greater));
        schema.completed = Some(false);
        schema.logged = Some(true);
        schema.trashed = Some(false);
        schema.area_id = Some(QueryMethod::Match(Uuid::new_v4()));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE project_title LIKE '%' || $1 || '%' AND notes LIKE '%' || $2 || '%' AND start_date = $3 AND start_time = $4 AND deadline > $5 AND completed_on ISNULL AND logged_on NOTNULL AND trashed_on ISNULL AND area_id = $6 AND user_id = $7"
        );
        assert_eq!(params.len(), 7);
    }

    #[test]
    fn limit() {
        let mut schema = QueryProjectSchema::default();
        schema.title = Some(QueryMethod::Match("Test Title".to_string()));
        schema.notes = Some(QueryMethod::Match("Test Note".to_string()));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.set_limit(25);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE project_title LIKE '%' || $1 || '%' AND notes LIKE '%' || $2 || '%' AND user_id = $3 LIMIT 25"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn offset() {
        let mut schema = QueryProjectSchema::default();
        schema.title = Some(QueryMethod::Match("Test Title".to_string()));
        schema.notes = Some(QueryMethod::Match("Test Note".to_string()));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.set_offset(50);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE project_title LIKE '%' || $1 || '%' AND notes LIKE '%' || $2 || '%' AND user_id = $3 OFFSET 50"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn limit_offset() {
        let mut schema = QueryProjectSchema::default();
        schema.title = Some(QueryMethod::Match("Test Title".to_string()));
        schema.notes = Some(QueryMethod::Match("Test Note".to_string()));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.set_limit(25);
        builder.set_offset(50);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE project_title LIKE '%' || $1 || '%' AND notes LIKE '%' || $2 || '%' AND user_id = $3 LIMIT 25 OFFSET 50"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn return_some() {
        let now = Local::now();

        let mut schema = QueryProjectSchema::default();
        schema.title = Some(QueryMethod::Match("Test Title".to_string()));
        schema.start_date = Some(QueryMethod::Compare(now.date_naive(), Compare::Less));
        schema.deadline = Some(QueryMethod::Compare(now.date_naive(), Compare::GreaterEq));
        schema.completed = Some(true);
        schema.area_id = Some(QueryMethod::NotNull(true));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.set_return(vec![ProjectModel::ID]);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT project_id FROM data.projects WHERE project_title LIKE '%' || $1 || '%' AND start_date < $2 AND deadline >= $3 AND completed_on NOTNULL AND area_id NOTNULL AND user_id = $4"
        );
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn return_all() {
        let now = Local::now();

        let mut schema = QueryProjectSchema::default();
        schema.title = Some(QueryMethod::Match("Test Title".to_string()));
        schema.start_date = Some(QueryMethod::Compare(now.date_naive(), Compare::Less));
        schema.deadline = Some(QueryMethod::Compare(now.date_naive(), Compare::GreaterEq));
        schema.completed = Some(true);
        schema.area_id = Some(QueryMethod::NotNull(true));

        let mut builder = SQLQueryBuilder::new();
        schema.add_to_query(&mut builder);
        builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &ID);
        builder.set_return_all();

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM data.projects WHERE project_title LIKE '%' || $1 || '%' AND start_date < $2 AND deadline >= $3 AND completed_on NOTNULL AND area_id NOTNULL AND user_id = $4"
        );
        assert_eq!(params.len(), 4);
    }

    // TODO: make production examples
}
