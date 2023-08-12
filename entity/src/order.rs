//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub slot_id: i32,
    pub order_id: String,
    pub currency: String,
    pub amount: i32,
    pub amount_paid: i32,
    pub amount_due: i32,
    pub receipt: String,
    pub offer_id: Option<String>,
    pub status: String,
    pub attempts: i16,
    pub updated_at: DateTime,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::review_slot::Entity",
        from = "Column::SlotId",
        to = "super::review_slot::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ReviewSlot,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::review_slot::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReviewSlot.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
