use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PgConManager;
use core::marker::{Send, Sync};
use futures::future;
use std::boxed::Box;
use tokio_postgres::{ToStatement, types::ToSql, row::Row, NoTls};

use crate::data::Document;

type MResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Реализует операции ввода-вывода над пулом соединений с базой данных PostgreSQL.
#[derive(Debug, Clone)]
pub struct Db {
  pool: Pool<PgConManager<NoTls>>,
}

impl Db {
  /// Создаёт объект из пула соединений.
  pub fn new(pool: Pool<PgConManager<NoTls>>) -> Db {
    Db { pool }
  }

  pub async fn get_file(&self, file_id: String) -> MResult<Row> {
    let cli = self.pool.get().await?;
    Ok(cli.query_one("select * from calby_files where file_id = $1;", &[&file_id]).await?)
  }

  pub async fn get_all_files_by_user(&self, user_id: String) -> MResult<Vec<Row>> {
    let cli = self.pool.get().await?;
    Ok(cli.query("select * from calby_files where user_id = $1;", &[&user_id]).await?)
  }

  pub async fn create_file(&self, document: &Document) -> MResult<String> {
    let cli = self.pool.get().await?;

    let req = cli.query_one("insert into calby_files values ($1, $2, $3, $4, $5);", &[
      &document.file_id,
      &document.file_name,
      &document.file_type,
      &document.user_id,
      &document.file_url
    ]).await;
    let _ = match req {
        Ok(res) => Some(res),
        Err(_) => None
    };
    Ok(document.file_id.to_owned())
  }

  /// Считывает одну строку из базы данных.
  pub async fn read<T>(&self, statement: &T, params: &[&(dyn ToSql + Sync)]) -> MResult<Row>
  where T: ?Sized + ToStatement {
    let cli = self.pool.get().await?;
    Ok(cli.query_one(statement, params).await?)
  }
  
  /// Записывает одно выражение в базу данных.
  pub async fn write<T>(&self, statement: &T, params: &[&(dyn ToSql + Sync)]) -> MResult<()>
  where T: ?Sized + ToStatement {
    let mut cli = self.pool.get().await?;
    let tr = cli.transaction().await?;
    tr.execute(statement, params).await?;
    tr.commit().await?;
    Ok(())
  }
  
  /// Считывает несколько значений по одной строке из базы данных.
  pub async fn read_mul<T>(&self, parts: Vec<(&T, Vec<&(dyn ToSql + Sync)>)>) -> MResult<Vec<Row>>
  where T: ?Sized + ToStatement + Send + Sync {
    let cli = self.pool.get().await?;
    let mut tasks = Vec::new();
    for i in 0..parts.len() {
      tasks.push(cli.query_one(parts[i].0, &parts[i].1));
    };
    let results = future::try_join_all(tasks).await?;
    Ok(results)
  }
  
  /// Записывает несколько значений в базу данных.
  pub async fn write_mul<T>(&self, parts: Vec<(&T, Vec<&(dyn ToSql + Sync)>)>) -> MResult<()>
  where T: ?Sized + ToStatement + Send + Sync {
    let mut cli = self.pool.get().await?;
    let tr = cli.transaction().await?;
    let mut tasks = Vec::new();
    for i in 0..parts.len() {
      tasks.push(tr.execute(parts[i].0, &parts[i].1));
    };
    future::try_join_all(tasks).await?;
    tr.commit().await?;
    Ok(())
  }
}

/// Настраивает базу данных.
///
/// Создаёт таблицы, которые будут предназначаться для хранения данных приложения.
pub async fn db_setup(db: &Db) -> MResult<()> {
  db.write_mul(vec![
    // ("CREATE TYPE doc_types AS ENUM ('DOCX', 'XLSX', 'XLS', 'TXT', 'JPG', 'PNG', 'SVG');", vec![]),
    ("create table if not exists calby_files (file_id varchar unique, file_name varchar, file_type doc_types, user_id varchar, file_url varchar);", vec![]),
  ]).await
}

// pub async fn create_user(db: &Db, sign_up_credentials: &SignUpCredentials) -> MResult<i64> {
//   let (salt, salted_pass) = key_gen::salt_pass(sign_up_credentials.pass.clone())?;
//   let id: i64 = db.read("select nextval(pg_get_serial_sequence('calby_users', 'id'));", &[]).await?.get(0);
//   let user_credentials = UserCredentials { salt, salted_pass, tokens: vec![] };
//   let user_credentials = serde_json::to_string(&user_credentials)?;
//   let billing = AccountPlanDetails {
//     billed_forever: false,
//     payment_data: String::new(),
//     is_paid_whenever: false,
//     last_payment: Utc::now()
//   };
//   let billing = serde_json::to_string(&billing)?;
//   db.write("insert into calby_users values ($1, $2, '[]', $3, $4);", &[&id, &sign_up_credentials.login, &user_credentials, &billing]).await?;
//   Ok(id)
// }
