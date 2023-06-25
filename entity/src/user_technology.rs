//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_technology")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub technology_id: i32,
    pub user_id: i32,
    #[sea_orm(column_type = "Double")]
    pub score: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::technology::Entity",
        from = "Column::TechnologyId",
        to = "super::technology::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Technology,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::technology::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Technology.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}