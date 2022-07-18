use diesel::{Queryable, Identifiable, table, PgConnection, RunQueryDsl, QueryDsl};

use super::establish_connection;

table! {
  files (id) {
      id -> Integer,
      file_id -> String,
      file_name -> VarChar,
      file_url -> VarChar,
      user_token -> VarChar,
      room_id -> VarChar,
      share_users -> Array<VarChar>,
  }
}

#[derive(Queryable, Identifiable, PartialEq)]
#[table_name="files"]
pub struct File {
  pub id: i32,
  pub file_id: uuid::Uuid,
  pub file_name: String,
  pub file_url: String,
  pub user_token: String,
  pub room_id: String,
  pub share_users: Vec<String>
}

impl File{
    pub fn get(user_token: String, file_id: uuid::Uuid) -> Option<File> {
      let conn = establish_connection();
      files::table
        .load::<File>(&conn)
        .unwrap()
        .into_iter()
        .find(
          |f| 
            f.file_id == file_id && ( f.user_token == user_token || f.share_users.contains(&user_token)
          )
        )
    }
}
