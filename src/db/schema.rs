use diesel::{Queryable, Insertable, Identifiable, table, PgConnection, RunQueryDsl, QueryDsl, pg};
use diesel_derive_enum::DbEnum;

use super::establish_connection;

#[derive(DbEnum, AsExpression, PartialEq, Debug)]
pub enum DocType {
  DOCX,
  XLSX,
  XLS,
  TXT,
  JPG,
  PNG,
  SVG
}

table! {
  use super::DocType;
  use diesel::types::*;
  files (id) {
      id -> Integer,
      file_id -> Uuid,
      file_name -> VarChar,
      file_type -> DocType,
      file_url -> VarChar,
      user_token -> VarChar,
      room_id -> VarChar,
      share_users -> Array<VarChar>,
  }
}

// // file_id varchar unique, file_name varchar, file_type doc_types, file_url varchar, user_token varchar, room_id varchar unique, share_users varchar[]
#[derive(Queryable, Identifiable, Debug, PartialEq)]
pub struct File {
  pub id: i32,
  pub file_id: uuid::Uuid,
  pub file_name: String,
  pub file_type: DocType,
  pub file_url: String,
  pub user_token: String,
  pub room_id: String,
  pub share_users: Vec<String>
}

impl File {
    pub fn get(user_token: String, file_id: uuid::Uuid) -> Option<File> {
      let conn = establish_connection();
      files::table.load::<File>(&conn).unwrap().into_iter().find(|file| file.user_token == user_token || file.share_users.contains(&user_token))
    }
}
