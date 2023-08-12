pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230623_113733_create_tech_table;
mod m20230624_102249_create_user_tech_table;
mod m20230623_114507_seed_tech_table;
mod m20230712_062519_create_league_table;
mod m20230712_063039_seed_league_table;
mod m20230712_131825_create_review_table;
mod m20230712_132508_create_community_chat;
mod m20230713_162119_create_slots;
mod m20230719_171213_create_roadmap;
mod m20230719_172144_create_roadmap_user;
mod m20230720_060928_seed_user_table;
mod m20230809_042320_create_order_table;
mod m20230809_045010_create_order_signature_table;





pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230623_113733_create_tech_table::Migration),
            Box::new(m20230624_102249_create_user_tech_table::Migration),
            Box::new(m20230712_062519_create_league_table::Migration),
            Box::new(m20230712_131825_create_review_table::Migration),
            Box::new(m20230713_162119_create_slots::Migration),
            Box::new(m20230719_171213_create_roadmap::Migration),
            Box::new(m20230719_172144_create_roadmap_user::Migration),
            Box::new(m20230809_042320_create_order_table::Migration),
            Box::new(m20230809_045010_create_order_signature_table::Migration),

            //  uncomment for seeding // only works after creating entities
            Box::new(m20230720_060928_seed_user_table::Migration),
            Box::new(m20230623_114507_seed_tech_table::Migration),
            Box::new(m20230712_063039_seed_league_table::Migration),
        ]
    }
}
