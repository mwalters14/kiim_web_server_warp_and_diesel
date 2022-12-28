use warp::Filter;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::types::question::{CreateQuestionDTO, UpdateQuestionDTO};
use crate::route_handlers::question::{
    get_questions_route_handle, 
    update_question_route_handle, 
    add_question_route_handle, 
    delete_question_route_handle
};
use crate::helpers::{with_db_access_manager, with_json_body};

type PgPool = Pool<ConnectionManager<PgConnection>>;
pub fn api_filters(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / ..) // Add path prefix /api/v1 to all our routes
        .and(
            add_question_route(pool.clone())
                .or(get_qustions_route(pool.clone()))
                .or(update_question_route(pool.clone()))
                .or(delete_question_route(pool.clone())),
        )
}

// ROUTE: POST /questions {CreateQuestionDTO body; return {REPLY, HTTP status code}}
pub fn add_question_route(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("questions")
        .and(warp::path::end()) 
        .and(warp::post()) 
        .and(with_db_access_manager(pool)) 
        .and(with_json_body::<CreateQuestionDTO>()) 
        .and_then(add_question_route_handle)
}

// ROUTE: GET /questions {empty body; return {JSON}}
pub fn get_qustions_route(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("questions")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db_access_manager(pool)) 
        .and_then(get_questions_route_handle)
}

// ROUTE: PUT /questions/:id {UpdateQuestionDTO body; return {REPLY, HTTP status code}}
pub fn update_question_route(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("questions" / i32)
        .and(warp::put())
        .and(warp::body::json::<UpdateQuestionDTO>())
        .and(with_db_access_manager(pool))
        .and_then(update_question_route_handle)
}

// ROUTE DELETE /questions/:id {empty body; return {warp::Reply}}
pub fn delete_question_route(
    pool: PgPool,
) -> impl Filter<Extract= impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("questions" / i32)
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(delete_question_route_handle)
}
