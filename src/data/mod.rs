use std::fmt::Display;
use std::{fs::write, io, path::Path};
use postgres_types::{ToSql, FromSql};
use serde::{Serialize, Deserialize};
use crate::documents::SaveFilesRequest;
use crate::utils::hash_string;

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql, Hash)]
#[postgres(name="doc_types")]
pub enum DocType {
  #[postgres(name="DOCX")]
  DOCX,
  #[postgres(name="XLSX")]
  XLSX,
  #[postgres(name="XLS")]
  XLS,
  #[postgres(name="TXT")]
  TXT,
  #[postgres(name="JPG")]
  JPG,
  #[postgres(name="PNG")]
  PNG,
  #[postgres(name="SVG")]
  SVG
}

impl DocType {
    pub fn folder(&self) -> String {
      use DocType::*;
      match self {
          DOCX | XLS | XLSX | TXT => "files".to_owned().to_string(),
          JPG | PNG | SVG => "images".to_owned().to_string(),
      }
    }
}

impl From<i32> for DocType {
    fn from(v: i32) -> Self {
        use DocType::*;
        match v {
            1 => DOCX,
            2 => XLSX,
            3 => XLS,
            4 => TXT,
            5 => JPG,
            6 => PNG,
            7 => SVG,
            _ => panic!("Undeclarated doc type")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Document {
  pub file_id: String,
  pub file_name: String,
  pub file_type: DocType,
  pub user_hash: String,
  pub file_url: String,
  pub room_id: String
}

impl From<SaveFilesRequest> for Document {
    fn from(req: SaveFilesRequest) -> Self {
        let user_hash = hash_string(req.user_id);
        Document {
          file_id: "".to_string(),
          file_name: req.file_name,
          file_type: DocType::from(req.doc_type),
          user_hash,
          file_url: "".to_string(),
          room_id: req.room_id,
        }
    }
}

impl Document {
  pub fn set_url(&mut self, room_id: String) -> io::Result<()> {
    if self.file_id.len() == 0 {
      panic!("field `file_id` is empty")
    }

    let format_path = &format!("data/{}/{}/{}-{}", room_id, &self.file_type.folder(), &self.file_id, &self.file_name);

    let path = Path::new(format_path);

    if Path::exists(path) {
      panic!("file {} exist", path.to_str().unwrap().to_owned());
    }

    self.file_url = path.to_str().unwrap().to_string();

    Ok(())
  }

  pub fn save(&self, data: Vec<u8>) -> Result<(), io::Error> {
    write(self.file_url.to_owned(), data)
  }
}

impl Display for Document {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Document:\n  name: {};\n  doc type: {:?}\n file_id: {}\n  file_url: {}", self.file_name, self.file_type, self.file_id, self.file_url)
  }
}
