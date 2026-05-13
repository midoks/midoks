use crate::rpc::client::rpc::{self, RequestAuth};
use crate::rpc::fastcdn::{PingRequest, PingResponse};

impl rpc::CommonRpc {
    pub async fn ping(
        &mut self,
        req: PingRequest,
    ) -> Result<PingResponse, Box<dyn std::error::Error>> {
        let request = self.prepare_request_with_metadata(req, RequestAuth::ADMIN)?;
        self.make_grpc_call(request, "/fastcdn.Ping/ping").await
    }
}
