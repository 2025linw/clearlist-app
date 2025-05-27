use std::sync::Arc;

use axum::{
    Router,
    handler::Handler,
    routing::{get, post},
};
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};

use crate::{
    AppState,
    handler::{area, auth::*, health_check_handler, project, tag, task},
};

pub fn create_api_router() -> Router<AppState> {
    let governor_default_conf = Arc::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(1)
            .burst_size(8)
            .finish()
            .unwrap(),
    );
    let governor_secure_conf = Arc::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(4)
            .burst_size(2)
            .finish()
            .unwrap(),
    );

    let auth_routes = Router::new()
        .route("/register", post(registration_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_handler))
        .layer(GovernorLayer {
            config: governor_secure_conf,
        });

    let task_routes = create_resource_router(
        task::create_handler,
        task::retrieve_handler,
        task::update_handler,
        task::delete_handler,
        task::query_handler,
    );
    let project_routes = create_resource_router(
        project::create_handler,
        project::retrieve_handler,
        project::update_handler,
        project::delete_handler,
        project::query_handler,
    );
    let area_routes = create_resource_router(
        area::create_handler,
        area::retrieve_handler,
        area::update_handler,
        area::delete_handler,
        area::query_handler,
    );
    let tag_routes = create_resource_router(
        tag::create_handler,
        tag::retrieve_handler,
        tag::update_handler,
        tag::delete_handler,
        tag::query_handler,
    );

    let api_routes = Router::new()
        .nest("/tasks", task_routes)
        .nest("/projects", project_routes)
        .nest("/areas", area_routes)
        .nest("/tags", tag_routes)
        .layer(GovernorLayer {
            config: governor_default_conf,
        });

    Router::new()
        .route("/healthcheck", get(health_check_handler))
        .nest("/auth", auth_routes)
        .merge(api_routes)
}

fn create_resource_router<C, R, U, D, Q, T1, T2, T3, T4, T5>(
    create_handler: C,
    retrieve_handler: R,
    update_handler: U,
    delete_handler: D,
    query_handler: Q,
) -> Router<AppState>
where
    C: Handler<T1, AppState>,
    R: Handler<T2, AppState>,
    U: Handler<T3, AppState>,
    D: Handler<T4, AppState>,
    Q: Handler<T5, AppState>,
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
{
    Router::new()
        .route("/", post(query_handler))
        .route("/create", post(create_handler))
        .route(
            "/{id}",
            get(retrieve_handler)
                .patch(update_handler)
                .delete(delete_handler),
        )
}
