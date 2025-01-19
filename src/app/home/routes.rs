use std::collections::HashMap;
use std::error::Error;
use std::future::Future;

use serde_json::json;
use warp::Filter;

use crate::app::core::app_config::{load_config, with_app_config, AppConfig};
use crate::app::core::authenticator::{with_cookies_session_auth, UserSessions};
use crate::app::core::http_client::{with_http_client, HttpClient};
use crate::app::core::models::{NewsContent, UserHistoryT, UserIdT};
use crate::app::core::renderer::{render, with_renderer, Renderer, WithTemplate};

pub fn home_routes(
    renderer: Renderer,
    http_client: HttpClient,
    sessions: UserSessions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    home_page(renderer.clone(), sessions.clone())
        .or(analyzer_search(
            renderer.clone(),
            http_client.clone(),
            sessions.clone(),
        ))
        .or(user_history(renderer.clone(), http_client.clone()))
}

fn home_page(
    renderer: Renderer,
    sessions: UserSessions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("home")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_cookies_session_auth(sessions.clone()))
        .map(|user_id| {
            println!("__home_page__: Extracting user ID based on cookie session -> {user_id}");
            WithTemplate {
                name: "home_page",
                value: json!({
                    "title": "Warptest",
                    "subtitle": "testing some warp app",
                    "user_id": user_id
                }),
            }
        })
        // -- y -> renderer()
        .and(with_renderer(renderer.clone()))
        // -- render(x, y)
        .map(render)
}

fn analyzer_search(
    renderer: Renderer,
    http_client: HttpClient,
    sessions: UserSessions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("analyze")
        .and(warp::get())
        .and(warp::query())
        .map(|q: HashMap<String, String>| match q.get("url") {
            Some(url) => url.into(),
            None => "invalid query key!".into(),
        })
        .and(with_http_client(http_client.clone()))
        .and(with_cookies_session_auth(sessions.clone()))
        .and(with_app_config(load_config().clone()))
        .map(
            |url: String, client: HttpClient, user_id: UserIdT, app_config: AppConfig| async move {
                Ok(client
                    .post(format!(
                        "{}/{}/news-contents/parse-news-url",
                        app_config.local_backend_api, user_id,
                    ))
                    .query(&[("news_url", url)])
                    .send()
                    .await?
                    .json()
                    .await?)
            },
        )
        .and(with_renderer(renderer.clone()))
        .and(with_cookies_session_auth(sessions.clone()))
        .and(with_app_config(load_config().clone()))
        .and_then(render_result)
}

fn user_history(
    renderer: Renderer,
    http_client: HttpClient,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    type UserID = String;
    warp::any()
        .and(warp::path::param::<UserID>()) // s(String,)
        .and(warp::path("history")) // ((),)
        .and(warp::path::end())
        .and(warp::get())
        .and(with_http_client(http_client.clone())) // s(String, HttpClient,)
        .and(with_app_config(load_config())) // s(String, Http Client, AppConfig,)
        .map(
            |user_id, client: HttpClient, config: AppConfig| async move {
                // let user_history = UserHistoryT::default();
                // let _user_history_endpoint =
                // "http://localhost:8000/api/v0/users/fa160d0b-2922-496e-a7b0-abc133c48ca7/history";
                Ok(client
                    .get(format!(
                        "{}/users/{}/history",
                        config.local_backend_api, user_id
                    ))
                    .send()
                    .await?
                    .json()
                    .await?)
            },
        ) // (Result<NewsContent[], Error>)
        .and(with_renderer(renderer.clone())) // (Result<..>, Renderer,)
        .and_then(render_history_list)
}

async fn render_result(
    news_content: impl Future<Output = Result<NewsContent, reqwest::Error>>,
    renderer: Renderer,
    user_id: UserIdT,
    app_config: AppConfig,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    match news_content.await {
        Ok(content) => {
            // println!("__content__: {:?}", content);
            let content_id_clone = content.id.clone().unwrap();
            let summarizer_service_endpoint = format!(
                "{}/{}/news_contents/summarize-news-content-stream?news_content_id={}",
                app_config.local_backend_api, user_id, content_id_clone
            );
            // println!(
            //     "__render_result__: Content ID -> {}\nEndpoint -> {summarizer_endpoint}",
            //     content.id.clone().unwrap()
            // );
            Ok(render(
                WithTemplate {
                    name: "analyze_result_component",
                    value: json!({
                        "news_content": content,
                        "summarizer_endpoint": summarizer_service_endpoint
                    }),
                },
                renderer.clone(),
            ))
        }
        Err(e) => {
            println!("__render_result__: ERROR {}", e.to_string());
            Ok(render(
                WithTemplate {
                    name: "analyze_result_error_component",
                    value: json!({
                        "error": e.to_string(),
                        "message": e.source().unwrap().to_string(),
                        "instructions": [
                        "Pastikan url mengarah ke suatu media berita",
                        "Coba gunakan media berita mainstream",
                        "Coba hubungi pihak pengembang"
                    ]
                    }),
                },
                renderer.clone(),
            ))
        }
    }
}

async fn render_history_list(
    user_history: impl Future<Output = Result<UserHistoryT, reqwest::Error>>,
    renderer: Renderer,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    match user_history.await {
        Ok(h) => Ok(render(
            WithTemplate {
                name: "history_drawer_component",
                value: json!({
                    "history": h.0
                }),
            },
            renderer.clone(),
        )),
        Err(_) => Ok(render(
            {
                WithTemplate {
                    name: "history_drawer_component",
                    value: json!({
                        "error": "some error!"
                    }),
                }
            },
            renderer.clone(),
        )),
    }
}
