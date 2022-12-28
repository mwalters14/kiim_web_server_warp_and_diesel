use tracing_subscriber::fmt::format::FmtSpan;
use warp::{self, http::Method, Filter};

mod db;
mod helpers;
mod route_handlers;
mod routes;
mod schema;
mod types;

#[tokio::main]
async fn main() {
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "kiim_web_server=info, warp=error".to_owned());

    let file_appender =
        tracing_appender::rolling::hourly("/Users/mwalters/Desktop", "kiim_web_server.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_file(true)
                .with_line_number(true),
        )
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record async event when each span closes
        // This can be used to time our
        // routes durations!
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    // CORS -> Here we define what HTTP Options are available on cross-origin requests
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    // Create a new connecton per request. Is it faster to reuse the PooledConnections?
    let pool = helpers::pg_pool();

    let routes = routes::question::api_filters(pool)
        .with(cors)
        .recover(handle_errors::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 1337)).await;
}

// TODO -> ROUTE: DELETE /questions/:questionId {empty body; return HTTP status code}
// TODO -> ROUTE: POST /answers {URL encoded body; return HTTP status code}
