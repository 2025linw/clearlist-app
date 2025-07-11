use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_jwt_auth::Claims;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

use crate::{error::ErrorResponse, models::{auth::token::Claim, project::tag::DatabaseModel}, util::{PostgresCmp, SQLQueryBuilder}, AppState};

pub async fn update_tags_handler(
    Claims(_): Claims<Claim>,
    State(data): State<AppState>,
    Path(project_id): Path<Uuid>,
    Json(body): Json<Vec<Uuid>>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut queries: Vec<(String, Vec<&(dyn ToSql + Sync)>)> = Vec::new();

    // Clear all tags related to project
    let mut query_builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
    query_builder
        .add_condition(DatabaseModel::PROJECT_ID, PostgresCmp::Equal, &project_id)
        .set_return(&[]);

    queries.push(query_builder.build_delete());

    // Add all tags
    for tag_id in body.iter() {
        let mut query_builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
        query_builder
            .add_column(DatabaseModel::PROJECT_ID, &project_id)
            .add_column(DatabaseModel::TAG_ID, tag_id);

        queries.push(query_builder.build_insert());
    }

    data.db_conn.query_transaction(queries).await?;

    Ok(StatusCode::CREATED)
}
