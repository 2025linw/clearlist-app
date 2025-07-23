use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_jwt_auth::Claims;
use serde_json::json;
use uuid::Uuid;

use crate::{
    AppState,
    data::{create_project, delete_project, query_project, retrieve_project, update_project},
    error::{ErrorResponse, INTERNAL},
    models::{
        FilterOptions, ToResponse,
        jwt::Claim,
        project::{
            CreateRequest, DeleteRequest, QueryRequest, ResponseModel, RetrieveRequest,
            UpdateRequest,
        },
    },
};

const NOT_FOUND: &str = "project not found";
const NO_UPDATES: &str = "no project updates were requested";

pub async fn create_handler(
    Claims(claim): Claims<Claim>,
    State(data): State<AppState>,
    Json(body): Json<CreateRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut conn = data.db_conn.get_conn().await?;

    let user_id = claim.sub;

    let mut schema = body;
    schema.set_user_id(user_id);

    if !schema.is_valid() {
        return Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            INTERNAL,
        ));
    }

    let project_id = create_project(&mut conn, schema).await?;

    let schema = RetrieveRequest::new(project_id, user_id);
    let project = match retrieve_project(&conn, schema).await? {
        Some(p) => p,
        None => return Err(ErrorResponse::new(StatusCode::NOT_FOUND, NOT_FOUND)),
    };

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "data": json!({
                "project": project.to_response(),
            })
        })),
    ))
}

pub async fn retrieve_handler(
    Claims(claim): Claims<Claim>,
    State(data): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let conn = data.db_conn.get_conn().await?;
    let user_id = claim.sub;

    let schema = RetrieveRequest::new(project_id, user_id);

    if !schema.is_valid() {
        return Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            INTERNAL,
        ));
    }

    let project = match retrieve_project(&conn, schema).await? {
        Some(p) => p,
        None => return Err(ErrorResponse::new(StatusCode::NOT_FOUND, NOT_FOUND)),
    };

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "data": json!({
                "project": project.to_response(),
            })
        })),
    ))
}

pub async fn update_handler(
    Claims(claim): Claims<Claim>,
    State(data): State<AppState>,
    Path(project_id): Path<Uuid>,
    Json(body): Json<UpdateRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut conn = data.db_conn.get_conn().await?;

    let user_id = claim.sub;

    if body.is_empty() {
        return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, NO_UPDATES));
    }

    let mut schema = body;
    schema.set_project_id(project_id);
    schema.set_user_id(user_id);

    if !schema.is_valid() {
        return Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            INTERNAL,
        ));
    }

    let project_id = match update_project(&mut conn, schema).await? {
        Some(p) => {
            assert_eq!(
                p, project_id,
                "error occured with query, as the project ids do not match after update"
            );

            p
        }
        None => return Err(ErrorResponse::new(StatusCode::NOT_FOUND, NOT_FOUND)),
    };

    let schema = RetrieveRequest::new(project_id, user_id);
    let project = match retrieve_project(&conn, schema).await? {
        Some(p) => p,
        None => return Err(ErrorResponse::new(StatusCode::NOT_FOUND, NOT_FOUND)),
    };

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "data": json!({
                "project": project.to_response(),
            })
        })),
    ))
}

pub async fn delete_handler(
    Claims(claim): Claims<Claim>,
    State(data): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut conn = data.db_conn.get_conn().await?;

    let user_id = claim.sub;

    let mut schema = DeleteRequest::new(project_id, user_id);
    schema.set_user_id(user_id);

    if !schema.is_valid() {
        return Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            INTERNAL,
        ));
    }

    if delete_project(&mut conn, schema).await?.is_none() {
        // TODO: consider other reasons for this function to return none

        return Err(ErrorResponse::new(StatusCode::NOT_FOUND, NOT_FOUND));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn query_handler(
    Claims(claim): Claims<Claim>,
    State(data): State<AppState>,
    Query(opts): Query<FilterOptions>,
    Json(body): Json<QueryRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let conn = data.db_conn.get_conn().await?;
    let user_id = claim.sub;

    let page = opts.page.unwrap_or(1);
    let limit = opts.limit.unwrap_or(25);
    let offset = (page - 1) * limit;

    let mut schema = body;
    schema.set_user_id(user_id);
    schema.set_limit(limit);
    schema.set_offset(offset);

    if !schema.is_valid() {
        return Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            INTERNAL,
        ));
    }

    let projects = query_project(&conn, schema).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "data": json!({
                "count": projects.len(),
                "projects": projects.into_iter().map(|p| p.to_response()).collect::<Vec<ResponseModel>>(),
            }),
        })),
    ))
}

#[cfg(test)]
mod project_handler {
    #[test]
    fn todo() {
        assert!(false);
    }
}
