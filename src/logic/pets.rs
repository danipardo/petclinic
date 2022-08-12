use std::collections::HashMap;

use chrono::naive::NaiveDateTime;
use rbatis::{crud::CRUD, crud_table, rbatis::Rbatis};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum PetType {
    Cat,
    Dog,
    Lizard,
    Horse,
}

pub fn types() -> HashMap<u32, PetType> {
    HashMap::from([
        (1, PetType::Cat),
        (2, PetType::Dog),
        (3, PetType::Lizard),
        (4, PetType::Horse),
    ])
}

#[crud_table]
#[derive(Default)]
pub struct Pet {
    pub id: u32,
    pub name: String,
    pub owner_name: String,
    pub owner_phone: String,
    pub age: u32,
    pub pet_type: u32,
    pub vet_id: Option<u32>,
    pub created_at: NaiveDateTime,
    pub created_by: u32,
}

pub async fn delete(rb: &Rbatis, pet: &Pet) -> Result<(), rbatis::Error> {
    rb.remove_by_column::<Pet, _>("id", &pet.id).await?;

    Ok(())
}

pub async fn search(rb: &Rbatis, name: Option<&String>) -> Result<Vec<Pet>, rbatis::Error> {
    let w = rb
        .new_wrapper()
        .like("name", name.unwrap_or(&String::new()));

    let pet_list: Vec<Pet> = rb.fetch_list_by_wrapper(w).await?;

    Ok(pet_list)
}

pub async fn get(rb: &Rbatis, id: u32) -> Result<Option<Pet>, rbatis::Error> {
    let c = rb.fetch_by_column("id", id).await?;

    Ok(c)
}

pub async fn save(rb: &Rbatis, pet: &Pet) -> Result<(), rbatis::Error> {
    if pet.id == 0 {
        let _result = rb.save(&pet, &[]).await?;
    } else {
        let w = rb.new_wrapper().eq("id", pet.id);
        let _result = rb.update_by_wrapper(&pet, w, &[]).await?;
    }

    Ok(())
}
