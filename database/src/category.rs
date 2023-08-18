//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::native_enum::Categories;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "category")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub position: i32,
  pub last_position: i32,
  pub active: bool,
  pub nft_id: i32,
  pub name: Categories,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::nft::Entity",
    from = "Column::NftId",
    to = "super::nft::Column::Id",
    on_update = "NoAction",
    on_delete = "Cascade"
  )]
  Nft,
}

impl Related<super::nft::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Nft.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
