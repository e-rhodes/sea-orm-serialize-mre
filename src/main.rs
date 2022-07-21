use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "group_id")]
    field: u32,
    #[sea_orm(column_name = "group_db")]
    field2: String,
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[tokio::main]
async fn main() {
    let conn = sea_orm::Database::connect(std::env::var("DATABASE_URL").unwrap()).await.unwrap();

    let results_into_json = Entity::find().into_json().one(&conn).await.unwrap().unwrap();
    let results_into_model = serde_json::to_value(Entity::find().one(&conn).await.unwrap()).unwrap();

    assert_eq!(results_into_json, results_into_model);
}
