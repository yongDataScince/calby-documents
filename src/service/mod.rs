use uuid::Builder;
use std::fs::{File, self};
use std::io::Read;
use std::{time::{SystemTime, UNIX_EPOCH}, fmt::Debug};
use tonic::{
  Request,
  Response,
  Status
};
use crate::data::DocType;
use crate::documents::{AddUserToFileResponse, AddUserToFileRequest};
use crate::utils::hash_string;
use crate::{documents::{
  SaveFilesRequest,
  SaveFilesResponse,
  GetFileRequest,
  GetFileResponse,
  documents_server::Documents
}, psql_handler::Db, data::Document};

#[derive(Clone)]
pub struct DocumentsServise {
  pub db: Db
}

impl Debug for DocumentsServise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Document service")
    }
}

fn read_file(filepath: &str) -> std::io::Result<Vec<u8>> {
  println!("{}", filepath);
  let mut file = File::open(filepath)?;
  let mut data = Vec::new();
  file.read_to_end(&mut data)?;

  return Ok(data);
}

#[tonic::async_trait]
impl Documents for DocumentsServise {
    async fn save_files(
      &self,
      request: Request<SaveFilesRequest>
    )
    -> Result<Response<SaveFilesResponse>, Status> {
      let req = request.into_inner();

      // Set timestamp
      let start = SystemTime::now();
      let since_the_epoch = start
          .duration_since(UNIX_EPOCH)
          .expect("Time went backwards");
      let timestamp = since_the_epoch.as_millis();
      // ----------------
  
      /*
        check if room directory exist
        if not exist than creatring directory
      */
      fs::create_dir(format!("./data/{}", req.room_id)).expect("Error to create directory");
      let req_file_type = DocType::from(req.doc_type);
      fs::create_dir(format!("./data/{}/{}", req.room_id, req_file_type.folder() )).expect("Error to create directory");
      
      // create uuid for new file
      let file_id = Builder::from_slice(
        &format!("{}{}{}", req.user_id, timestamp, req.file_name)
        .as_bytes()[..16]
      )
        .expect("error to create uuid")
        .into_uuid()
        .to_string();

      let mut document = Document::from(req.clone());
      document.file_id = file_id.to_owned();
      document.set_url(req.room_id).expect("error in set url for document");

      self.db.create_file(&document).await.expect("can't save file to db");
      document.save(req.data).expect("can't save file to local");

      let reply = SaveFilesResponse {
        successful: true,
        file_id: file_id.to_owned(),
        info_message: format!("Save file with id: {}", file_id.to_owned()),
      };

      Ok(Response::new(reply))
    }

    async fn get_file(
      &self,
      request: Request<GetFileRequest>
    ) -> Result<Response<GetFileResponse>, Status> {
      let req = request.into_inner();
      let user_hash = hash_string(req.user_id);
      let res = self.db.get_file(req.file_id, user_hash).await.expect("can't get file from db");
      let resp_doc =  Document {
        file_id: res.get(0),
        file_name: res.get(1),
        file_type: res.get(2),
        file_url: res.get(3),
        user_hash: res.get(4),
        room_id: res.get(5)
      };
      let json_doc = serde_json::to_string(&resp_doc).expect("can't stringify struct");
  
      let parsed_document: Document = serde_json::from_str(&json_doc).expect("can't parse");
      println!("{}", parsed_document);
      let reply = GetFileResponse {
        data: read_file(&parsed_document.file_url).unwrap(),
        file_id: parsed_document.file_id,
        file_name: parsed_document.file_name
      };

      Ok(Response::new(reply))
    }

    async fn add_user_to_file(
      &self,
      request: Request<AddUserToFileRequest>
    ) -> Result<Response<AddUserToFileResponse>, Status> {
      let reply = AddUserToFileResponse {
        successful: true,
      };
      Ok(Response::new(reply))
    }
  }


