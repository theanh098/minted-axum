//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  #[sea_orm(column_type = "Text")]
  pub address: String,
  pub is_admin: bool,
  pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::nft::Entity")]
  Nft,
}

impl Related<super::nft::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Nft.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}