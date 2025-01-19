use std::future::Future;

use crate::app::core::authenticator::{add_user_to_sessions, UserSessions};
use crate::app::core::error::{
    BuildResponseError, IncorrectPassword, RegisterExistingUser, UserNotExist,
};
use crate::app::core::models::PublicUserWithId;

pub async fn handle_register(
    response: impl Future<Output = Result<reqwest::Response, reqwest::Error>>,
    sessions: UserSessions,
) -> Result<impl warp::Reply, warp::Rejection> {
    match response.await {
        Ok(result) => match result.status() {
            reqwest::StatusCode::OK => match result.json::<PublicUserWithId>().await {
                Ok(new_user) => {
                    println!(
                        "__handle_register__: When register success, returning user -> {:?}",
                        new_user
                    );

                    // -- BLOCK: ADD_USER_TO_SESSIONS
                    let new_session_id =
                        add_user_to_sessions(new_user.id, sessions.clone()).await?;
                    // -- ENDBLOCK: ADD_USER_TO_SESSIONS
                    match warp::http::Response::builder()
                        .status(warp::http::StatusCode::MOVED_PERMANENTLY.as_u16())
                        .header("Location", "/home")
                        .header("HX-Location", "/home")
                        .header(
                            "set-cookie",
                            format!("session_id={new_session_id}; path=/; max-age=34560"),
                        )
                        .body("")
                    {
                        Ok(r) => Ok(r),
                        Err(_somehow_error_when_building_response) => {
                            eprintln!("__handle_login__: Build Response Fail!");
                            Err(warp::reject::custom(BuildResponseError))
                        }
                    }
                }
                Err(_deserialize_response_error) => {
                    println!(
                        "__register__: deserialize user fail-> {:?}",
                        _deserialize_response_error
                    );
                    Err(warp::reject())
                }
            },
            reqwest::StatusCode::CONFLICT => {
                eprintln!("__handle_register__: conflict!");
                Err(warp::reject::custom(RegisterExistingUser))
            }
            _other_error => {
                eprintln!("__handle_register__: unknown!");
                Err(warp::reject())
            }
        },
        Err(_error_cause_by_invalid_request) => Err(warp::reject()),
    }
}

pub async fn handle_login(
    response: impl Future<Output = Result<reqwest::Response, reqwest::Error>>,
    sessions: UserSessions,
) -> Result<impl warp::Reply, warp::Rejection> {
    match response.await {
        Ok(res) => match res.status() {
            reqwest::StatusCode::OK => match res.json::<PublicUserWithId>().await {
                Ok(user) => {
                    println!(
                        "__handle_login__: When Login Success get user -> {:?}",
                        user
                    );

                    // -- BLOCK: ADD_USER_TO_SESSIONS
                    let new_session_id = add_user_to_sessions(user.id, sessions.clone()).await?;
                    // -- ENDBLOCK: ADD_USER_TO_SESSIONS

                    match warp::http::Response::builder()
                        .status(warp::http::StatusCode::MOVED_PERMANENTLY.as_u16())
                        .header("Location", "/home")
                        .header("HX-Location", "/home")
                        .header(
                            "set-cookie",
                            format!("session_id={new_session_id}; path=/; max-age=34560"),
                        )
                        .body("")
                    {
                        Ok(r) => Ok(r),
                        Err(_somehow_error_when_building_response) => {
                            eprintln!("__handle_login__: Build Response Fail!");
                            Err(warp::reject::custom(BuildResponseError))
                        }
                    }
                }
                Err(_deserialize_response_error) => {
                    println!(
                        "__handle_login__: deserialize user fail-> {:?}",
                        _deserialize_response_error
                    );
                    Err(warp::reject())
                }
            },
            reqwest::StatusCode::NOT_FOUND => {
                eprintln!("__handle_login__: Not Found!");
                Err(warp::reject::custom(UserNotExist))
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                eprintln!("__handle_login__: Unauthorized!");
                Err(warp::reject::custom(IncorrectPassword))
            }
            _other_error => {
                eprintln!("__handle_login__: another response error!");
                Err(warp::reject())
            }
        },
        Err(_error_cause_by_invalid_request) => Err(warp::reject()),
    }
}
