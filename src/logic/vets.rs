use rbatis::{crud::CRUD, crud_table, rbatis::Rbatis};

#[crud_table]
#[derive(Default)]
pub struct Vet {
    pub id: u32,
    pub name: String,
}

pub async fn delete(rb: &Rbatis, vet: &Vet) -> Result<(), rbatis::Error> {
    rb.remove_by_column::<Vet, _>("id", &vet.id).await?;

    Ok(())
}

pub async fn search(rb: &Rbatis, name: Option<&String>) -> Result<Vec<Vet>, rbatis::Error> {
    let w = rb
        .new_wrapper()
        .like("name", name.unwrap_or(&String::new()));

    let vet_list: Vec<Vet> = rb.fetch_list_by_wrapper(w).await?;

    Ok(vet_list)
}

pub async fn get(rb: &Rbatis, id: u32) -> Result<Option<Vet>, rbatis::Error> {
    let v = rb.fetch_by_column("id", id).await?;

    Ok(v)
}

pub async fn save(rb: &Rbatis, vet: &Vet) -> Result<(), rbatis::Error> {
    if vet.id == 0 {
        rb.save(&vet, &[]).await?;
    } else {
        let w = rb.new_wrapper().eq("id", vet.id);
        rb.update_by_wrapper(&vet, w, &[]).await?;
    }
    Ok(())
}
