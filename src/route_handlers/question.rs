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

pub async fn get_questions_route_handle(
    mut db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = db_manager.get_question();

    Ok(warp::reply::json(&response))
}

// pub async fn get_a_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
//     match store
//         .questions
//         .read()
//         .await
//         .get(&QuestionId::new(id.clone()))
//     {
//         Some(res) => Ok(warp::reply::json(res)),
//         None => {
//             event!(
//                 target: "kiim_web_server",
//                 Level::ERROR,
//                 "Question Not Found Exception. ID {} does not exist.",
//                 id,
//             );
//             Err(warp::reject::custom(Error::QuestionNotFound(id)))
//         }
//     }
// }

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

pub async fn add_question_route_handle(
    mut db_manager: DBAccessManager,
    new_question: CreateQuestionDTO,
) -> Result<impl warp::Reply, warp::Rejection> {
    // let mut question_exist = false;

    // if store.questions.read().await.get(&question.id).is_some() {
    //     question_exist = true;
    // }

    // if question_exist {
    //     Err(warp::reject::custom(Error::QuestionExist))
    // } else {
    let create_book_dto = new_question.to_dto();

    let _response = db_manager
        .create_question(create_book_dto)
        .map(|question| IdResponse::new(question.id));
    // store
    //     .questions
    //     .write()
    //     .await
    //     .insert(question.id.clone(), question);

    Ok(warp::reply::with_status(
        "Question added success",
        StatusCode::OK,
    ))
    // }
}

// pub async fn update_question(
//     id: i32,
//     store: Store,
//     question: Question,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     match store
//         .questions
//         .write()
//         .await
//         .get_mut(&QuestionId::new(id.clone()))
//     {
//         Some(q) => *q = question,
//         None => return Err(warp::reject::custom(Error::QuestionNotFound(id))),
//     }

//     Ok(warp::reply::with_status(
//         "Question updated successfully",
//         StatusCode::OK,
//     ))
// }

// pub async fn delete_question(
//     id: i32,
//     store: Store,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     match store
//         .questions
//         .write()
//         .await
//         .remove(&QuestionId::new(id.clone()))
//     {
//         Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
//         None => Err(warp::reject::custom(Error::QuestionNotFound(id))),
//     }
// }

/*
    Rust
    WARP -> Route handling
    Hyper -> HTTP request
    Tokio -> Runtime
    Mio -> kernal API -> talks with OS
*/
