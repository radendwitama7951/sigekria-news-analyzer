use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;
use warp::Filter;

use super::error::{InternalServerProblem, UnauthorizeRequest};
use super::models::UserIdT;

pub type SessionIdT = String;

pub type UserSessions = Arc<Mutex<HashMap<SessionIdT, UserIdT>>>;

pub fn with_sessions(
    sessions: UserSessions,
) -> impl Filter<Extract = (UserSessions,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || sessions.clone())
}

// pub fn check_user_exist(cred: PublicUserCred, client: HttpClient) -> bool {
//     todo!()
// }

pub fn generate_session_id() -> SessionIdT {
    Uuid::new_v4().to_string()
}

pub fn with_cookies_session_auth(
    sessions: UserSessions,
) -> impl Filter<Extract = (UserIdT,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::cookie::optional::<String>("session_id"))
        .and(with_sessions(sessions.clone()))
        .and_then(auth_cookie_session)
}

async fn auth_cookie_session(
    cookie_session_id: Option<String>,
    sessions: UserSessions,
) -> Result<UserIdT, warp::Rejection> {
    match cookie_session_id {
        Some(session_id) => match sessions.try_lock() {
            Ok(locked_session) => match locked_session.get(&session_id) {
                Some(user_id) => {
                    println!("__with_cookies_session_auth__: UserID -> {user_id}");
                    Ok(user_id.to_string())
                }
                None => {
                    eprintln!("__with_cookies_session_auth__: User session not found");
                    Err(warp::reject::custom(UnauthorizeRequest))
                }
            },

            Err(_locking_session_fail) => {
                eprintln!("__with_cookies_session_auth__: Something wrong during the mutex lock");
                Err(warp::reject::custom(InternalServerProblem))
            }
        },
        None => {
            eprintln!("__with_cookies_session_auth__: User not login yet");
            Err(warp::reject::custom(UnauthorizeRequest))
        }
    }
}

pub async fn add_user_to_sessions(
    id: UserIdT,
    sessions: UserSessions,
) -> Result<SessionIdT, warp::Rejection> {
    let new_session_id = generate_session_id();

    match sessions.try_lock() {
        Ok(mut sessions_lock) => {
            sessions_lock.insert(new_session_id.to_string(), id.to_string());
            println!("__add_user_to_sessions__: adding user {id}, into session {new_session_id}");
            Ok(new_session_id)
        }
        Err(_) => {
            println!("__add_user_to_sessions__: Something wrong with the mutex lock");
            Err(warp::reject::custom(InternalServerProblem))
        }
    }
}
