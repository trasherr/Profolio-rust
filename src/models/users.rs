use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::uuid::Uuid;
use rbs::to_value;
use serde::Serialize;
use serde::Deserialize;
use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbatis::Rbatis;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Users {
    pub id: Option<i64>,
    pub uuid: Option<Uuid>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub phone_code: Option<String>,
    pub password: Option<String>,
    pub experience: Option<i32>,
    pub company: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub created_at: Option<DateTime>,
}
crud!(Users{});//crud = insert+select_by_column+update_by_column+delete_by_column

impl_select!(Users{select_by_email(id:&str,name:&str) => "`where email = #{email} limit 1`"});
impl_select!(Users{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});
// impl_update!(Users{update_by_name(name:&str) => "`where id = 1`"});
// impl_delete!(Users {delete_by_name(name:&str) => "`where name= '2'`"});
// impl_select_page!(Users{select_page(name:&str) => "`where name != #{name}`"});

pub async fn up(rb: &Rbatis){

    let uuid = Uuid("df07fea2-b819-4e05-b86d-dfc15a5f52a9".to_string());
    let mut s = SqliteTableSync::default();
    s.sql_id = " PRIMARY KEY NOT NULL ".to_string();
    s.sync(rb.acquire().await.unwrap(), to_value!(Users {
        id: Some(0),
        uuid: Some(uuid),
        first_name: Some("2".into()),
        last_name: Some("2".into()),
        email: Some("1".into()),
        password: Some("2".into()),
        phone: Some("2".into()),
        phone_code: Some("2".into()),
        experience: Some(0),
        company: Some("2".into()),
        linkedin: Some("2".into()),
        github: Some("2".into()),
        created_at: Some(DateTime::utc()),

    }), "rb_users")
        .await
        .unwrap();

    not_null(&rb,"first_name").await;
    not_null(&rb,"created_at").await;
    not_null(&rb,"uuid").await;
    not_null(&rb,"email").await;
    not_null(&rb,"phone").await;
    not_null(&rb,"phone_code").await;
    not_null(&rb,"password").await;

    unique(&rb,"unique_uuid","uuid").await;
    unique(&rb,"unique_email","email").await;
    unique(&rb,"unique_phone","phone").await;

}

#[py_sql("drop table rb_users")]
pub async fn drop(rb: &Rbatis) -> Users { }

async fn unique(rb: &Rbatis, key: &str, constr: &str)  { 
    let query = "ALTER TABLE rb_users ADD CONSTRAINT ".to_owned() + &key + " UNIQUE(" +  &constr + ")";
    rb.query_decode::<Users>(&query,vec![]).await.map_err(|err| println!("{:?}", err)).ok();
 }

 async fn not_null(rb: &Rbatis, coll: &str)  { 
    let query = "ALTER TABLE rb_users ALTER COLUMN ".to_owned() + coll + " SET NOT NULL";
    rb.query_decode::<Users>(&query,vec![]).await.map_err(|err| println!("{:?}", err)).ok();
 }
