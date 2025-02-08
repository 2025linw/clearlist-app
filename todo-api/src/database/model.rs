pub mod task {
    use serde::{Deserialize, Serialize};
    use tokio_postgres::Row;

    use chrono::{DateTime, Local, NaiveDate, NaiveTime};
    use uuid::Uuid;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TaskModel {
        task_id: Uuid,

        task_title: Option<String>,
        task_notes: Option<String>,
        start_date: Option<NaiveDate>,
        start_time: Option<NaiveTime>,
        deadline: Option<NaiveDate>,
        completed_on: Option<DateTime<Local>>,
        logged_on: Option<DateTime<Local>>,
        trashed_on: Option<DateTime<Local>>,

        area_id: Option<Uuid>,
        project_id: Option<Uuid>,

        user_id: Uuid,
        created_on: DateTime<Local>,
        updated_on: DateTime<Local>,
    }

    impl TaskModel {
        pub const TABLE: &str = "todo_data.tasks";

        pub const ID: &str = "task_id";

        pub const TITLE: &str = "task_title";
        pub const NOTES: &str = "task_notes";
        pub const START_DATE: &str = "start_date";
        pub const START_TIME: &str = "start_time";
        pub const DEADLINE: &str = "deadline";
        pub const COMPLETED: &str = "completed_on";
        pub const LOGGED: &str = "logged_on";
        pub const TRASHED: &str = "trashed_on";

        pub const AREA_ID: &str = "area_id";
        pub const PROJECT_ID: &str = "project_id";

        pub const USER_ID: &str = "user_id";
        pub const CREATED: &str = "created_on";
        pub const UPDATED: &str = "updated_on";
    }

    impl From<Row> for TaskModel {
        fn from(value: Row) -> Self {
            Self {
                task_id: value.get(TaskModel::ID),

                task_title: value.try_get(TaskModel::TITLE).ok(),
                task_notes: value.try_get(TaskModel::NOTES).ok(),
                start_date: value.try_get(TaskModel::START_DATE).ok(),
                start_time: value.try_get(TaskModel::START_TIME).ok(),
                deadline: value.try_get(TaskModel::DEADLINE).ok(),
                completed_on: value.try_get(TaskModel::COMPLETED).ok(),
                logged_on: value.try_get(TaskModel::LOGGED).ok(),
                trashed_on: value.try_get(TaskModel::TRASHED).ok(),

                area_id: value.try_get(TaskModel::AREA_ID).ok(),
                project_id: value.try_get(TaskModel::PROJECT_ID).ok(),

                user_id: value.get(TaskModel::USER_ID),
                created_on: value.get(TaskModel::CREATED),
                updated_on: value.get(TaskModel::UPDATED),
            }
        }
    }
}

pub mod project {
    use serde::{Deserialize, Serialize};
    use tokio_postgres::Row;

    use chrono::{DateTime, Local, NaiveDate, NaiveTime};
    use uuid::Uuid;

    pub const PROJECT_TABLE: &str = "todo_data.projects";

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ProjectModel {
        project_id: Uuid,

        project_title: Option<String>,
        project_notes: Option<String>,
        start_date: Option<NaiveDate>,
        start_time: Option<NaiveTime>,
        deadline: Option<NaiveDate>,
        completed_on: Option<DateTime<Local>>,
        logged_on: Option<DateTime<Local>>,
        trashed_on: Option<DateTime<Local>>,

        area_id: Option<Uuid>,

        user_id: Uuid,
        created_on: DateTime<Local>,
        updated_on: DateTime<Local>,
    }

    impl ProjectModel {
        pub const ID: &str = "project_id";

        pub const TITLE: &str = "project_title";
        pub const NOTES: &str = "project_notes";
        pub const START_DATE: &str = "start_date";
        pub const START_TIME: &str = "start_time";
        pub const DEADLINE: &str = "deadline";
        pub const COMPLETED: &str = "completed_on";
        pub const LOGGED: &str = "logged_on";
        pub const TRASHED: &str = "trashed_on";

        pub const AREA_ID: &str = "area_id";

        pub const USER_ID: &str = "user_id";
        pub const CREATED: &str = "created_on";
        pub const UPDATED: &str = "updated_on";
    }

    impl From<Row> for ProjectModel {
        fn from(value: Row) -> Self {
            Self {
                project_id: value.get(ProjectModel::ID),

                project_title: value.try_get(ProjectModel::TITLE).ok(),
                project_notes: value.try_get(ProjectModel::NOTES).ok(),
                start_date: value.try_get(ProjectModel::START_DATE).ok(),
                start_time: value.try_get(ProjectModel::START_TIME).ok(),
                deadline: value.try_get(ProjectModel::DEADLINE).ok(),
                completed_on: value.try_get(ProjectModel::COMPLETED).ok(),
                logged_on: value.try_get(ProjectModel::LOGGED).ok(),
                trashed_on: value.try_get(ProjectModel::TRASHED).ok(),

                area_id: value.try_get(ProjectModel::AREA_ID).ok(),

                user_id: value.get(ProjectModel::USER_ID),
                created_on: value.get(ProjectModel::CREATED),
                updated_on: value.get(ProjectModel::UPDATED),
            }
        }
    }
}

pub mod area {
    use serde::{Deserialize, Serialize};
    use tokio_postgres::Row;

    use chrono::{DateTime, Local};
    use uuid::Uuid;

    pub const AREA_TABLE: &str = "todo_data.areas";

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AreaModel {
        area_id: Uuid,

        area_name: Option<String>,
        icon_url: Option<String>,

        user_id: Uuid,
        created_on: DateTime<Local>,
        updated_on: DateTime<Local>,
    }

    impl AreaModel {
        pub const ID: &str = "area_id";

        pub const NAME: &str = "area_name";
        pub const ICON_URL: &str = "icon_url";

        pub const USER_ID: &str = "user_id";
        pub const CREATED: &str = "created_on";
        pub const UPDATED: &str = "updated_on";
    }

    impl From<Row> for AreaModel {
        fn from(value: Row) -> Self {
            Self {
                area_id: value.get(AreaModel::ID),

                area_name: value.try_get(AreaModel::NAME).ok(),
                icon_url: value.try_get(AreaModel::ICON_URL).ok(),

                user_id: value.get(AreaModel::USER_ID),
                created_on: value.get(AreaModel::CREATED),
                updated_on: value.get(AreaModel::UPDATED),
            }
        }
    }
}

pub mod tag {
    use serde::{Deserialize, Serialize};
    use tokio_postgres::Row;

    use chrono::{DateTime, Local};
    use hex_color::HexColor;
    use uuid::Uuid;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TagModel {
        tag_id: Uuid,

        tag_label: Option<String>,
        tag_category: Option<String>,
        tag_color: Option<HexColor>,

        user_id: Uuid,
        created_on: DateTime<Local>,
        updated_on: DateTime<Local>,
    }

    impl TagModel {
        pub const TABLE: &str = "todo_data.tags";

        pub const ID: &str = "tag_id";

        pub const LABEL: &str = "tag_label";
        pub const CATEGORY: &str = "tag_category";
        pub const COLOR: &str = "tag_color";

        pub const USER_ID: &str = "user_id";
        pub const CREATED: &str = "created_on";
        pub const UPDATED: &str = "updated_on";
    }

    impl From<Row> for TagModel {
        fn from(value: Row) -> Self {
            Self {
                tag_id: value.get(TagModel::ID),

                tag_label: value.try_get(TagModel::LABEL).ok(),
                tag_category: value.try_get(TagModel::CATEGORY).ok(),
                tag_color: value
                    .try_get(TagModel::COLOR)
                    .map_or(None, |s| HexColor::parse_rgb(s).ok()),

                user_id: value.get(TagModel::USER_ID),
                created_on: value.get(TagModel::CREATED),
                updated_on: value.get(TagModel::UPDATED),
            }
        }
    }

    #[derive(Debug)]
    pub struct TaskTagModel {
        task_id: Uuid,
        tag_id: Uuid,
    }

    impl TaskTagModel {
        pub const TABLE: &str = "todo_data.task_tags";

        pub const TASK_ID: &str = "task_id";
        pub const TAG_ID: &str = "tag_id";
    }

    #[derive(Debug)]
    pub struct ProjectTagModel {
        project_id: Uuid,
        tag_id: Uuid,
    }

    impl ProjectTagModel {
        pub const TABLE: &str = "todo_data.project_tags";

        pub const PROJECT_ID: &str = "project_id";
        pub const TAG_ID: &str = "tag_id";
    }
}
