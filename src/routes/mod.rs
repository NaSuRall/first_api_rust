use axum::Router;

pub fn create_router() -> Router {
    Router::new().nest("/users", super::routes::user_routes::router())
}
