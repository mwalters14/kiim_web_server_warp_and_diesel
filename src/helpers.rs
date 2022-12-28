use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use handle_errors::{AppError, ErrorType};
use serde::de::DeserializeOwned;
use warp::{reject, Filter};

use crate::db;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn pg_pool() -> PgPool {
    let manager =
        ConnectionManager::<PgConnection>::new("postgres://postgres:docker@localhost/postgres");
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

pub fn with_db_access_manager(
    pool: PgPool,
) -> impl Filter<Extract = (db::data_access::DBAccessManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {
            match pool.get() {
                Ok(conn) => Ok(db::data_access::DBAccessManager::new(conn)),
                Err(err) => Err(reject::custom(AppError::new(
                    format!("Error getting connection from pool: {}", err.to_string()).as_str(),
                    ErrorType::Internal,
                ))),
            }
        })
}

pub fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
