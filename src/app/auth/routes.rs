use serde_json::json;
use warp::Filter;

use crate::app::core::app_config::{load_config, with_app_config, AppConfig};
use crate::app::core::authenticator::{with_sessions, UserSessions};
use crate::app::core::http_client::{with_http_client, HttpClient};
use crate::app::core::models::PublicUserCred;
use crate::app::core::renderer::{render, with_renderer, Renderer, WithTemplate};

use super::handlers::{handle_login, handle_register};

pub fn auth_routes(
    renderer: Renderer,
    http_client: HttpClient,
    sessions: UserSessions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let default_redirect = warp::path("auth")
        .and(warp::path::end())
        .map(|| warp::redirect(warp::http::Uri::from_static("/auth/login")));

    default_redirect
        .or(register_page(renderer.clone()))
        .or(login_page(renderer.clone()))
        .or(register_route(http_client.clone(), sessions.clone()))
        .or(login_route(http_client.clone(), sessions.clone()))
}

fn register_page(
    renderer: Renderer,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("auth")
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(warp::get())
        .map(|| WithTemplate {
            name: "auth_page",
            value: json!({
                "title": "AUTH Register",
                "child_component": "register_page"
            }),
        })
        .and(with_renderer(renderer.clone()))
        .map(render)
}

fn login_page(
    renderer: Renderer,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("auth")
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::get())
        .map(|| WithTemplate {
            name: "auth_page",
            value: json!({
                "title": "Auth Login",
                "child_component": "login_page"
            }),
        })
        .and(with_renderer(renderer.clone()))
        .map(render)
}

fn register_route(
    http_client: HttpClient,
    sessions: UserSessions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("auth")
        .and(warp::path("register"))
        .and(warp::body::form::<PublicUserCred>())
        .and(warp::path::end())
        .and(warp::post())
        .and(with_http_client(http_client.clone()))
        .and(with_app_config(load_config().clone()))
        .map(
            |user_cred: PublicUserCred, client: HttpClient, app_config: AppConfig| async move {
                println!("__register_route__: UserCredentials {:?}", user_cred);
                let res = client
                    .post(format!("{}/users", &app_config.local_backend_api))
                    .json(&user_cred)
                    .send()
                    .await?;

                Ok(res)
            },
        )
        .and(with_sessions(sessions.clone()))
        .and_then(handle_register)
}

fn login_route(
    http_client: HttpClient,
    sessions: UserSessions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("auth")
        .and(warp::path("login"))
        .and(warp::body::form::<PublicUserCred>())
        .and(warp::path::end())
        .and(warp::post())
        .and(with_http_client(http_client.clone()))
        .and(with_app_config(load_config().clone()))
        .map(
            |user_cred: PublicUserCred, client: HttpClient, app_config: AppConfig| async move {
                println!("__login_route__: UserCredentials {:?}", user_cred);
                Ok::<reqwest::Response, reqwest::Error>(
                    client
                        .post(format!("{}/users/login", &app_config.local_backend_api))
                        .json(&user_cred)
                        .send()
                        .await?,
                )
            },
        )
        .and(with_sessions(sessions.clone()))
        .and_then(handle_login)
}
