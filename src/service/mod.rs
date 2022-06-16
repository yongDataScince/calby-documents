use tonic::{
  Request,
  Response,
  Status
};
use crate::documents::{
  documents_server::Documents,
  SendFilesRequest, SendFilesResponse
};

#[derive(Debug, Default)]
pub struct DocumentsServise;

#[tonic::async_trait]
impl Documents for DocumentsServise {
  async fn send_files(
    &self,
    request: Request<SendFilesRequest>
  ) -> Result<Response<SendFilesResponse>, Status> {
    let req = request.into_inner();
    let reply = SendFilesResponse {
      successful: true,
      message: format!("file name: {}; file type: {}", req.file_name, req.doc_type)
    };

    Ok(Response::new(reply))
  }
}
