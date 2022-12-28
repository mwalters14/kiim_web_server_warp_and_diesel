use warp::http::StatusCode;

use crate::db::data_access::DBAccessManager;
use crate::types::question::{CreateQuestionDTO, UpdateQuestionDTO};

pub struct IdResponse {
    pub id: i32,
}

impl IdResponse {
    pub fn new(id: i32) -> IdResponse {
        IdResponse { id }
    }
}

pub async fn get_questions_route_handle(
    mut db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = db_manager.get_question();

    Ok(warp::reply::json(&response))
}

pub async fn update_question_route_handle(
    id: i32,
    updated_question: UpdateQuestionDTO,
    mut db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    let create_updated_dto = updated_question.to_dto();
    let _response = db_manager
        .update_question(id, create_updated_dto)
        .map(|question| question);

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question_route_handle(
    id: i32,
    mut db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _response = db_manager.delete_question(id);

    Ok(warp::reply::with_status(
        "Question delete success.",
        StatusCode::OK
    ))
}

pub async fn add_question_route_handle(
    mut db_manager: DBAccessManager,
    new_question: CreateQuestionDTO,
) -> Result<impl warp::Reply, warp::Rejection> {
    let create_book_dto = new_question.to_dto();

    let _response = db_manager
        .create_question(create_book_dto)
        .map(|question| IdResponse::new(question.id));

    Ok(warp::reply::with_status(
        "Question added success",
        StatusCode::OK,
    ))
}

/*
    Rust
    WARP -> Route handling
    Hyper -> HTTP request
    Tokio -> Runtime
    Mio -> kernal API -> talks with OS
*/
