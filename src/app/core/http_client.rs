use std::sync::Arc;

use warp::Filter;

pub type HttpClient = Arc<reqwest::Client>;

pub fn with_http_client(
    http_client: HttpClient,
) -> impl Filter<Extract = (HttpClient,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || http_client.clone())
}
