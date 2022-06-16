use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DocType {
    DOCX,
    XLSX,
    XLS,
    TXT,
}

#[derive(Serialize, Deserialize)]
pub struct Document {
  pub name: String,
  pub doc_type: DocType,
  pub data: Vec<u8>
}

impl Display for Document {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Document:\n\tname: {};\n\tdoc type: {:?}", self.name, self.doc_type)
  }
}


