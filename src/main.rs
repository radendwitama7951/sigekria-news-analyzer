use std::collections::HashMap;
use std::sync::Arc;

use handlebars::Handlebars;
use tokio::sync::Mutex;
use warp::Filter;
use warptest::app::core::app_config::load_config;
use warptest::app::core::authenticator::SessionIdT;
use warptest::app::core::error::redirect_on_reject;
use warptest::app::core::models::UserIdT;
use warptest::app::core::routes::error_routes;
use warptest::app::routes::app_routes;
use warptest::{project, register_templates};

#[tokio::main]
async fn main() {
    // -- BLOCK: CONFIGURE_APP
    //
    let _app_config = { load_config() };

    let users_sessions = { Arc::new(Mutex::new(HashMap::<SessionIdT, UserIdT>::new())) };

    let hb = {
        let mut hb = Handlebars::new();
        register_templates(&mut hb);
        Arc::new(hb)
    };

    let rqwest = { Arc::new(reqwest::Client::new()) };

    let cors = {
        warp::cors()
            .allow_any_origin()
            .allow_methods(vec!["GET", "POST", "UPDATE", "DELETE"])
    };

    let root_redirect = {
        warp::path::end()
            .and(warp::get())
            .and(warp::path::end())
            .map(|| warp::redirect(warp::http::Uri::from_static("auth")))
    };

    let assets_route =
        { warp::get().and(warp::path("assets").and(warp::fs::dir(project("/assets")))) };

    let _version_prefix = { warp::path("v1") };

    let routes = {
        root_redirect
            .or(assets_route)
            .or(app_routes(
                hb.clone(),
                rqwest.clone(),
                users_sessions.clone(),
            ))
            .or(error_routes(hb.clone()))
            // .or(warp::any()
            //     .map(|| warp::redirect(warp::http::Uri::from_static("/error/not-found"))))
            .with(cors)
            .recover(redirect_on_reject)
    };
    // -- ENDBLOCK: CONFIGURE_APP

    warp::serve(routes).run(([127, 0, 0, 1], 6969)).await;
}
