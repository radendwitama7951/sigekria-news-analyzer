use warp::Filter;

use super::auth::routes::auth_routes;
use super::core::authenticator::UserSessions;
use super::core::http_client::HttpClient;
use super::core::renderer::Renderer;
use super::home::routes::home_routes;

pub fn app_routes(
    renderer: Renderer,
    http_client: HttpClient,
    session: UserSessions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let some_app = warp::path("app")
        .and(warp::path::end())
        .and(warp::get())
        .map(|| warp::http::Response::builder().body("App works!"));

    some_app
        .or(auth_routes(
            renderer.clone(),
            http_client.clone(),
            session.clone(),
        ))
        .or(home_routes(
            renderer.clone(),
            http_client.clone(),
            session.clone(),
        ))
}
