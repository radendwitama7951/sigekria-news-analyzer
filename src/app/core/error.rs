use std::convert::Infallible;

use serde::Serialize;
use warp::http::StatusCode;
use warp::{reject, Rejection, Reply};

#[derive(Debug)]
pub struct RegisterExistingUser;
impl reject::Reject for RegisterExistingUser {}

#[derive(Debug)]
pub struct IncorrectPassword;
impl reject::Reject for IncorrectPassword {}

#[derive(Debug)]
pub struct UserNotExist;
impl reject::Reject for UserNotExist {}

#[derive(Debug)]
pub struct BuildResponseError;
impl reject::Reject for BuildResponseError {}

#[derive(Debug)]
pub struct UnauthorizeRequest;
impl reject::Reject for UnauthorizeRequest {}

#[derive(Debug)]
pub struct InternalServerProblem;
impl reject::Reject for InternalServerProblem {}

/// An API error serializable to JSON.
#[derive(Serialize, Clone)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: &'static str,
    pub instructions: Vec<&'static str>,
}

pub async fn redirect_on_reject(err: Rejection) -> Result<impl Reply, Infallible> {
    let redirect_path;

    if err.is_not_found() {
        eprintln!("__redirect_on_reject__: NOT FOUND");
        redirect_path = "/error/not-found";
    } else if let Some(UnauthorizeRequest) = err.find() {
        eprintln!("__redirect_on_reject__: UNAUTHORIZE user not login yet");
        redirect_path = "/auth";
    } else if let Some(RegisterExistingUser) = err.find() {
        eprintln!("__redirect_on_reject__: EMAIL CONFLICT");
        redirect_path = "/error/email-taken";
    } else if let Some(UserNotExist) = err.find() {
        eprintln!("__redirect_on_reject__: User Not Exist");
        redirect_path = "/error/not-found";
    } else if let Some(IncorrectPassword) = err.find() {
        eprintln!("__redirect_on_reject__: User failed to login",);
        redirect_path = "/error/incorrect-password"
    } else if let Some(InternalServerProblem) = err.find() {
        eprintln!("__redirect_on_reject__: Something bad happen to server",);
        redirect_path = "/error/server-error"
    } else if let Some(BuildResponseError) = err.find() {
        eprintln!("__redirect_on_reject__: Building response error",);
        redirect_path = "/error/server-error"
    } else if let Some(_error_on_broken_req_body) =
        err.find::<warp::filters::body::BodyDeserializeError>()
    {
        eprintln!("__redirect_on_reject__: BAD REQUEST on input body deserialize");
        redirect_path = "/error/bad-request"
    } else if let Some(_error_on_wrong_req_http_method) =
        err.find::<warp::reject::MethodNotAllowed>()
    {
        eprintln!("__redirect_on_reject__: BAD REQUEST on wrong req http method");
        redirect_path = "/error/bad-request";
    } else {
        eprintln!("__redirect_on_reject__: INTERNAL SERVER ERROR of unknown");
        redirect_path = "/error/server-error";
    }

    Ok(warp::reply::with_header(
        warp::redirect::redirect(warp::http::Uri::from_static(redirect_path)),
        "HX-Location",
        redirect_path,
    ))
}

// UNUSE
pub async fn _handle_rejection(err: Rejection) -> impl Reply {
    let code;
    let message;

    match err.find() {
        Some(RegisterExistingUser) => {
            eprintln!("__handle_rejection__: conflict rejection -> {:?}", err);
            code = StatusCode::CONFLICT;
            message = "Email already taken";
        }
        _ => {
            eprintln!("__handle_rejection__: unhandle rejection -> {:?}", err);
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "UNHANDLED_REJECTION";
        }
    }

    warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
        instructions: vec!["some error"],
    })
}
