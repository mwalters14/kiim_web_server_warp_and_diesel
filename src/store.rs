// use std::collections::HashMap;
// use std::sync::Arc; // Arc allows us to allocate data on the heap and allow multiple pointers to the data
// use tokio::sync::RwLock; // RwLock allows us to lock a thread to prevent multiple writes
// use crate::types::question::{Question, QuestionId};
// use diesel::pg::PgConnection;
// use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
// use dotenvy::dotenv;
// use std::env;
// use warp::{Filter, reject};

// type PgPool = Pool<ConnectionManager<PgConnection>>;
// type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

// fn pg_pool() -> PgPool {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
//     let manager =
//         ConnectionManager::<PgConnection>::new(database_url);
//         Pool::new(manager).expect("Postgres connection pool could not be established.")
// }
// /*
//     SETUP
//         Mock database  -> in-memory store for how a relational database might look
// */
// #[derive(Clone, Debug)]
// pub struct Store {
//     pub connection: Pool<ConnectionManager<PgConnection>>, // Here we wrap our data structure in Arc<RwLock to allow multiple pointers to the same data
// }

// impl Store {
//     pub fn new() -> Self {
//         let pool = pg_pool();
//         let pg = warp::any()
//             .map(move || pool.clone())
//             .and_then(|pool: PgPool| match pool.get() {
//                 Ok(conn) => Ok(conn),
//                 Err(_) => Err(reject::not_found()),
//             });

//         // This is a constructor to init a Type Store
//         Store {
//             connection: pool,
//         }
//     }
// }


// Create mock DB
    //let store = store::Store::new();

    /*
        warp::any()  -> match any request
        .map -> map on filter to pass a value to a function
        || -> Rust closure
        move -> keyword indicates capture by value, which means it moves the values into the closure and takes ownership of the values.
        clone -> We return a clone of the store to every function that applies this store_filter

        In a typical DB setup we would not need to clone the store
    */

    // create a path Filter
    // let get_questions = warp::get()
    //     .and(warp::path("questions"))
    //     .and(warp::path::end())
    //     .and(warp::query())
    //     .and(store_filter.clone())
    //     .and_then(routes::question::get_questions)
    //     .with(warp::trace(|info| {
    //         tracing::info_span!(
    //             "get_questions request",
    //             method = %info.method(),
    //             path = %info.path(),
    //             id = %Uuid::new_v4(),
    //         )
    //     }));

    // let get_a_question = warp::get()
    //     .and(warp::path("questions"))
    //     .and(warp::path::param::<i32>())
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and_then(routes::question::get_a_question);

    // let add_question = warp::post()
    //     .and(warp::path("questions"))
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and(warp::body::json())
    //     .and_then(routes::question::add_question);

    // let update_question = warp::put()
    //     .and(warp::path("questions"))
    //     .and(warp::path::param::<i32>())
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and(warp::body::json())
    //     .and_then(routes::question::update_question);

    // let delete_question = warp::delete()
    //     .and(warp::path("questions"))
    //     .and(warp::path::param::<i32>())
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and_then(routes::question::delete_question);

    // let routes = get_questions
    // // .or(get_a_question)
    // add_question
    // // .or(update_question)
    // // .or(delete_question)
    // .with(cors)
    // .with(warp::trace::request())
    // .recover(return_error);
