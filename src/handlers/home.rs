use crate::Context;
use axum::{extract::Extension, response::Html};
use tera::Tera;

use std::sync::Arc;

pub async fn home(
    Extension(tera): Extension<Tera>,
    Extension(_state): Extension<Arc<Context>>,
) -> Html<String> {
    let c = tera::Context::new();

    tracing::debug!("Main request");

    let r = tera.render("home.html", &c).unwrap();

    Html::from(r)
}
