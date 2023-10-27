#![warn(clippy::all)]

use handle_errors::return_error;
use warp::{http::Method, Filter};

mod routes;
mod store;
mod types;

/// WRAP Server
#[tokio::main]
async fn main() {
    // env_logger::init();
    // log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    //      ━━━━━━━━━━━━━━━━━━━━━━━ LOGGING With WRAP ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // let log = warp::log::custom(|info| {
    //     // eprintln!(
    //     log::info!(
    //         "{} {} {} {:?} from {} with {:?}",
    //         info.method(),
    //         info.path(),
    //         info.status(),
    //         info.elapsed(),
    //         info.remote_addr().unwrap(),
    //         info.request_headers(),
    //     );
    // });

    // log::error!("This is an error!");
    // log::info!("This is an info!");
    // log::warn!("This is a warning!");

    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec![Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and(id_filter)
        .and_then(routes::question::get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    //         ╭──────────────────────────────────────────────────────────╮
    //         │                       WRAP Server                        │
    //         ╰──────────────────────────────────────────────────────────╯

    // let routes = add_question.recover(return_error);

    let routes = get_questions
        .or(update_question)
        .or(add_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        // .with(log)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
