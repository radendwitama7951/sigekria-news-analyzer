use warp::Filter;

use super::error::ErrorMessage;
use super::renderer::{render, with_renderer, with_template, Renderer, WithTemplate};

// ROUTES
pub fn error_routes(
    renderer: Renderer,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    error_404_page(renderer.clone())
        .or(error_400_page(renderer.clone()))
        .or(error_500_page(renderer.clone()))
        .or(error_registering_existing_user_page(renderer.clone()))
        .or(error_incorrect_password(renderer.clone()))
}

fn error_incorrect_password(
    renderer: Renderer,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("error")
        .and(warp::path("incorrect-password"))
        .and(warp::path::end())
        .and(warp::get())
        .and(with_template(WithTemplate {
            name: "error_page",
            value: ErrorMessage {
                code: warp::http::StatusCode::UNAUTHORIZED.as_u16(),
                message: "Unauthorizer",
                instructions: vec![
                    "Pastikan password dan email sesuai",
                    "Hubungi pihak pengembang untuk mengganti password",
                ],
            },
        }))
        .and(with_renderer(renderer.clone()))
        .map(render)
}

fn error_400_page(
    renderer: Renderer,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("error")
        .and(warp::path("bad-request"))
        .and(warp::path::end())
        .and(warp::get())
        .and(with_template(WithTemplate {
            name: "error_page",
            value: ErrorMessage {
                code: warp::http::StatusCode::BAD_REQUEST.as_u16(),
                message: "Bad Request",
                instructions: vec!["Pastikan input sudah benar", "Hubungi pihak pengembang"],
            },
        }))
        .and(with_renderer(renderer.clone()))
        .map(render)
}

fn error_404_page(
    renderer: Renderer,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("error")
        .and(warp::path("not-found"))
        .and(warp::path::end())
        .and(warp::get())
        .and(with_template(WithTemplate {
            name: "error_page",
            value: ErrorMessage {
                code: warp::http::StatusCode::NOT_FOUND.as_u16(),
                message: "Not Found",
                instructions: vec!["Pastikan input sudah benar", "Hubungi pihak pengembang"],
            },
        }))
        .and(with_renderer(renderer.clone()))
        .map(render)
}

fn error_500_page(
    renderer: Renderer,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("error")
        .and(warp::path("server-error"))
        .and(warp::get())
        .and(warp::path::end())
        .and(with_template(WithTemplate {
            name: "error_page",
            value: ErrorMessage {
                code: warp::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Internal Server Error",
                instructions: vec![
                    "Coba kembali dan refresh halaman",
                    "Hubungi pihak pengembang",
                ],
            },
        }))
        .and(with_renderer(renderer.clone()))
        .map(render)
}

fn error_registering_existing_user_page(
    renderer: Renderer,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("error")
        .and(warp::path("email-taken"))
        .and(warp::get())
        .and(warp::path::end())
        .and(with_template(WithTemplate {
            name: "error_page",
            value: ErrorMessage {
                code: warp::http::StatusCode::CONFLICT.as_u16(),
                message: "Conflict",
                instructions: vec!["Coba gunakan email lain", "Hubungi pihak pengembang"],
            },
        }))
        .and(with_renderer(renderer.clone()))
        .map(render)
}
