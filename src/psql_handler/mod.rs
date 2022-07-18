use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PgConManager;
use core::marker::{Send, Sync};
use futures::future;
use std::boxed::Box;
use tokio_postgres::{ToStatement, types::ToSql, row::Row, NoTls};
use crate::{data::Document};

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

  pub async fn get_file(&self, file_id: String, user_hash: String) -> MResult<Row> {
    let cli = self.pool.get().await?;
    Ok(cli.query_one("select * from calby_files where file_id = $1 and user_token = $2;", &[&file_id, &user_hash]).await?)
  }

  pub async fn get_all_files_by_user(&self, user_hash: String) -> MResult<Vec<Row>> {
    let cli = self.pool.get().await?;
    Ok(cli.query("select * from calby_files where user_token = $1;", &[&user_hash]).await?)
  }

  pub async fn create_file(&self, document: &Document) -> MResult<String> {
    let cli = self.pool.get().await?;

    let req = cli.query_one("insert into calby_files values ($1, $2, $3, $4, $5, $6, $7);", &[
      &document.file_id,
      &document.file_name,
      &document.file_type,
      &document.file_url,
      &document.user_hash,
      &document.room_id,
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
  println!("Setup database");
  db.write_mul(vec![
    ("DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'doc_types') THEN CREATE TYPE  doc_types AS ENUM ('DOCX', 'XLSX', 'XLS', 'TXT', 'JPG', 'PNG', 'SVG'); END IF; END$$;", vec![]),
    ("create table if not exists calby_files (file_id varchar unique, file_name varchar, file_type doc_types, file_url varchar, user_token varchar, room_id varchar unique, share_users varchar[]);", vec![]),
  ]).await
}

