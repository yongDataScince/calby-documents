use std::{fs::File, io::Write};
use uuid::Uuid;
use tonic::{
  Request,
  Response,
  Status
};
use crate::documents::{
  SaveFilesRequest,
  SaveFilesResponse,
  GetFileRequest,
  GetFileResponse,
  documents_server::Documents
};

#[derive(Debug, Default, Clone, Copy)]
pub struct DocumentsServise;

#[tonic::async_trait]
impl Documents for DocumentsServise {
    async fn save_files(
      &self,
      request: Request<SaveFilesRequest>
    )
    -> Result<Response<SaveFilesResponse>, Status> {
      let req = request.into_inner();
      let file_id: String = Uuid::parse_str(&format!("{}{}", req.file_name, req.user_id)).expect("error to create uuid").to_string();

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
      todo!()
    }
}


