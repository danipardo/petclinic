use crate::{
    logic::{
        users::User,
        vets::{self, Vet},
    },
    AppError, Context,
};
use axum::{
    extract::{Extension, Path, Query},
    response::{Html, IntoResponse, Redirect},
};

use serde::Deserialize;
use tera::Tera;

use std::{collections::HashMap, sync::Arc};

#[derive(Deserialize)]
pub struct VetForm {
    id: u32,
    name: String,
}
pub async fn save(
    vet: axum_extra::extract::Form<VetForm>,
    Extension(state): Extension<Arc<Context>>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(mut v) = vets::get(&state.rb, vet.id).await? {
        v.name = vet.name.clone();
        vets::save(&state.rb, &v).await?;
    } else {
        // Adding a new one
        let v = Vet {
            id: 0,
            name: vet.name.clone(),
        };
        vets::save(&state.rb, &v).await?;
    }
    Ok(Redirect::to("/vets"))
}

pub async fn list(
    Extension(tera): Extension<Tera>,
    Extension(state): Extension<Arc<Context>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, AppError> {
    let mut c = tera::Context::new();

    let name = params.get("name");
    let vets: Vec<Vet> = vets::search(&state.rb, name).await?;

    c.insert("vets", &vets);
    let r = tera.render("vet/list.html", &c).unwrap();

    Ok(Html::from(r))
}
pub async fn get(
    Extension(tera): Extension<Tera>,
    Extension(state): Extension<Arc<Context>>,
    _user: User,
    Path(id): Path<u32>,
) -> Result<Html<String>, AppError> {
    let mut c = tera::Context::new();

    let mut vet = vets::get(&state.rb, id).await?;

    if id == 0 {
        vet = Some(Vet::default());
    }
    if vet.is_none() {
        return Ok(Html::from("Vet not found".to_string()));
    }

    c.insert("vet", &vet);
    let r = tera.render("vet/edit.html", &c).unwrap();

    Ok(Html::from(r))
}

pub async fn delete(
    Extension(state): Extension<Arc<Context>>,
    _user: User,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, AppError> {
    let vet = vets::get(&state.rb, id).await?;
    if let Some(vet) = vet {
        vets::delete(&state.rb, &vet).await?;
    }
    Ok(Redirect::to("/vets"))
}
