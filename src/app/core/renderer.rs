use std::sync::Arc;

use handlebars::Handlebars;
use serde::Serialize;
use warp::Filter;

pub type Renderer = Arc<Handlebars<'static>>;

#[derive(Clone)]
pub struct WithTemplate<T: Serialize + Clone + Send> {
    pub name: &'static str,
    pub value: T,
}

pub fn with_template<T: Serialize + Clone + Send>(
    template: WithTemplate<T>,
) -> impl Filter<Extract = (WithTemplate<T>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || template.clone())
}

pub fn with_renderer(
    renderer: Renderer,
) -> impl Filter<Extract = (Renderer,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || renderer.clone())
}

pub fn render<T>(
    template: WithTemplate<T>,
    hbs: Arc<Handlebars<'_>>,
) -> Result<impl warp::Reply, std::convert::Infallible>
where
    T: Serialize + Clone + Send,
{
    Ok(warp::reply::html(
        hbs.render(template.name, &template.value)
            .unwrap_or_else(|err| err.to_string()),
    ))
}
