pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230623_113733_create_tech_table;
mod m20230624_102249_create_user_tech_table;
mod m20230623_114507_seed_tech_table;
mod m20230625_113658_create_community_table;
mod m20230625_131307_create_community_user_table;
mod m20230712_062519_create_league_table;
mod m20230712_063039_seed_league_table;
mod m20230712_131825_create_review_table;
mod m20230712_132508_create_community_chat;
mod m20230713_162119_create_slots;





pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230623_113733_create_tech_table::Migration),
            Box::new(m20230624_102249_create_user_tech_table::Migration),
            Box::new(m20230625_113658_create_community_table::Migration),
            Box::new(m20230625_131307_create_community_user_table::Migration),
            Box::new(m20230712_062519_create_league_table::Migration),
            Box::new(m20230712_131825_create_review_table::Migration),
            Box::new(m20230713_162119_create_slots::Migration),

            //  uncomment for seeding // only works after creating entities
            Box::new(m20230623_114507_seed_tech_table::Migration),
            Box::new(m20230712_063039_seed_league_table::Migration),
        ]
    }
}
