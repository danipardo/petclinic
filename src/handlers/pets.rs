use crate::{
    logic::{
        pets::{self, Pet},
        users::User,
        vets::{self, Vet},
    },
    AppError, Context,
};
use axum::{
    extract::{Extension, Path, Query},
    response::{Html, IntoResponse, Redirect, Response},
};

use axum_extra::extract::Form;
use chrono::Utc;
use serde::Deserialize;
use tera::Tera;

use std::{collections::HashMap, sync::Arc};

#[derive(Deserialize)]
pub struct PetForm {
    id: u32,
    name: String,
    owner_name: String,
    owner_phone: String,
    age: u32,
    current_vet: u32,
    pet_type: u32,
}

impl From<Form<PetForm>> for Pet {
    fn from(form: Form<PetForm>) -> Pet {
        Pet {
            id: form.id,
            name: form.name.clone(),
            owner_name: form.owner_name.clone(),
            owner_phone: form.owner_phone.clone(),
            age: form.age,
            vet_id: match form.current_vet {
                0 => None,
                n => Some(n),
            },
            pet_type: form.pet_type,
            created_by: 0,
            created_at: Utc::now().naive_utc(),
        }
    }
}

pub async fn save(
    pet_form: axum_extra::extract::Form<PetForm>,
    user: User,
    Extension(state): Extension<Arc<Context>>,
) -> Result<impl IntoResponse, AppError> {
    // let mut txn = state.pets.get_pool().begin().await.unwrap();

    let c = pets::get(&state.rb, pet_form.id).await?;

    if pet_form.id == 0 {
        let mut pet: Pet = pet_form.into();
        pet.created_by = user.id;
        pets::save(&state.rb, &pet).await?;
    } else {
        if c.is_none() {
            return Ok(Redirect::to("/pets"));
        }
        let mut c = c.unwrap();

        c.name = pet_form.name.clone();
        c.owner_name = pet_form.owner_name.clone();
        c.owner_phone = pet_form.owner_phone.clone();
        c.age = pet_form.age;
        c.pet_type = pet_form.pet_type;

        if pet_form.current_vet > 0 {
            c.vet_id = Some(pet_form.current_vet);
        } else {
            c.vet_id = None
        }
        pets::save(&state.rb, &c).await?;
    }

    Ok(Redirect::to("/pets"))
}

pub async fn list(
    Extension(tera): Extension<Tera>,
    Extension(state): Extension<Arc<Context>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, AppError> {
    let mut c = tera::Context::new();

    let name = params.get("name");
    let pets = pets::search(&state.rb, name).await?;

    let types = pets::types();
    c.insert("pets", &pets);
    c.insert("pet_types", &types);
    let r = tera.render("pet/list.html", &c).unwrap();

    Ok(Html::from(r))
}

pub async fn delete(
    Extension(state): Extension<Arc<Context>>,
    _user: User,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, AppError> {
    let pet = pets::get(&state.rb, id).await?;
    if let Some(pet) = pet {
        pets::delete(&state.rb, &pet).await?;
    }
    Ok(Redirect::to("/pets"))
}

pub async fn get(
    Extension(tera): Extension<Tera>,
    Extension(state): Extension<Arc<Context>>,
    _user: User,
    Path(id): Path<u32>,
) -> Result<Response, AppError> {
    let mut c = tera::Context::new();

    let mut pet = pets::get(&state.rb, id).await?;

    if id == 0 {
        pet = Some(pets::Pet::default());
    }
    if pet.is_none() {
        return Ok(Redirect::to("/pets").into_response());
    }
    let pet = pet.unwrap();

    let vets: Vec<Vet> = vets::search(&state.rb, None).await?;

    //    let current_vet: Option<Vet> = vets::of_pet(&state.rb, &pet).await;

    let types = pets::types();

    c.insert("pet_types", &types);
    //  c.insert("current_vet", &current_vet);
    c.insert("pet", &pet);
    c.insert("vets", &vets);
    let r = tera.render("pet/edit.html", &c).unwrap();

    Ok(Html::from(r).into_response())
}
