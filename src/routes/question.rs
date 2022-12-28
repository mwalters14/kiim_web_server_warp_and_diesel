use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use warp::Filter;

use crate::helpers::{with_db_access_manager, with_json_body};
use crate::route_handlers::question::{
    add_question_route_handle, get_questions_route_handle, update_question_route_handle,
};
use crate::types::question::{CreateQuestionDTO, UpdateQuestionDTO};

type PgPool = Pool<ConnectionManager<PgConnection>>;
pub fn api_filters(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / ..) // Add path prefix /api/v1 to all our routes
        .and(
            add_question_route(pool.clone())
                .or(get_qustions_route(pool.clone()))
                .or(update_question_route(pool.clone())),
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

// ROUTE: PUT /questions {UpdateQuestionDTO body; return {REPLY, HTTP status code}}
pub fn update_question_route(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("questions" / i32)
        .and(warp::put())
        .and(warp::body::json::<UpdateQuestionDTO>())
        .and(with_db_access_manager(pool))
        .and_then(update_question_route_handle)
}

// #[instrument] // instrument automatically creates a span  which is a period of tme
// pub async fn get_questions(
//     params: HashMap<String, String>,
//     PgC: Store,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     if !params.is_empty() {
//         let pagination = extract_pagination(params)?;

//         let res: Vec<Question> = store
//             .questions
//             .read() // read request data from RwLock thread
//             .await // We must await the data because a lock could be on the data from another thread
//             .values()
//             .cloned()
//             .collect();

//         if pagination.start >= res.len() || pagination.end > res.len() {
//             event!( // events are moments in time within a span
//                 target: "kiim_webserver_events",
//                 Level::ERROR,
//                 "Out of bounds Exception.");
//             Err(warp::reject::custom(Error::OutOfBounds))
//         } else {
//             let res = &res[pagination.start..pagination.end];
//             Ok(warp::reply::json(&res))
//         }
//     } else {
//         info!(pagination = false);
//         let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
//         Ok(warp::reply::json(&res))
//     }
// }
