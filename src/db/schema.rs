use diesel::{Queryable, Identifiable, table, RunQueryDsl, insert_into};

use crate::data::Document;

use super::establish_connection;

table! {
  calby_files (id) {
      id -> Integer,
      file_id -> String,
      file_name -> VarChar,
      file_url -> VarChar,
      user_token -> VarChar,
      room_id -> VarChar,
      share_users -> Array<VarChar>,
  }
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[table_name="calby_files"]
pub struct File {
  pub id: i32,
  pub file_id: uuid::Uuid,
  pub file_name: String,
  pub file_url: String,
  pub user_token: String,
  pub room_id: String,
  pub share_users: Vec<String>
}

impl File {
    pub fn get_file(user_token: String, file_id: uuid::Uuid) -> Option<File> {
      use super::schema::calby_files::table;
      let conn = establish_connection();
      table.load::<File>(&conn)
        .unwrap()
        .into_iter()
        .find(
          |f| 
            f.file_id == file_id && ( f.user_token == user_token || f.share_users.contains(&user_token)
          )
        )
    }
    
    pub fn create(document: &Document) {
      use super::schema::calby_files::table;
      insert_into(table);
    }
}
