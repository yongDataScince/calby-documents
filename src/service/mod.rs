use uuid::Builder;
use std::fs::{File, remove_file};
use std::io::Read;
use std::{time::{SystemTime, UNIX_EPOCH}, fmt::Debug};
use tonic::{
  Request,
  Response,
  Status
};
use crate::{documents::{
  SaveFilesRequest,
  SaveFilesResponse,
  GetFileRequest,
  GetFileResponse,
  EditFileRequest,
  EditFileResponse,
  RemoveFileRequest,
  RemoveFileResponse,
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
      let start = SystemTime::now();
      let since_the_epoch = start
          .duration_since(UNIX_EPOCH)
          .expect("Time went backwards");
      
      let timestamp = since_the_epoch.as_millis();
  
      let req = request.into_inner();

      let file_id = Builder::from_slice(
        &format!("{}{}{}", req.user_id, timestamp, req.file_name)
        .as_bytes()[..16]
      )
        .expect("error to create uuid")
        .into_uuid()
        .to_string();

      let mut document = Document::from(req.clone());
      document.file_id = file_id.to_owned();
      document.set_url().expect("error in set url for document");

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
      let res = self.db.get_file(req.file_id).await.expect("can't get file from db");
      let resp_doc =  Document {
        file_id: res.get(0),
        file_name: res.get(1),
        file_type: res.get(2),
        user_id: res.get(3),
        file_url: res.get(4)
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

    // async fn edit_file(
    //   &self,
    //   request: Request<EditFileRequest>
    // ) -> Result<Response<EditFileResponse>, Status> {
    //     todo!()
    // }

    // async fn remove_file(
    //   &self,
    //   request: Request<RemoveFileRequest>
    // ) -> Result<Response<RemoveFileResponse>, Status> {
        
    //     todo!()
    // }
  }


