use latencies::latency_fetcher_server::{LatencyFetcher, LatencyFetcherServer};
use latencies::{LatencyRequest, LatencyResponse, LatencyStreamRequest};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

pub mod latencies {
    tonic::include_proto!("latencies");
}

#[derive(Debug, Default)]
pub struct LatencyService {}
#[tonic::async_trait]
impl LatencyFetcher for LatencyService {
    async fn get_latency(
        &self,
        request: Request<LatencyRequest>,
    ) -> Result<Response<LatencyResponse>, Status> {
        println!("Got a request: {:?}", request);

        let res = latencies::LatencyResponse {
            service: request.into_inner().service.into(),
            response_time: 0.00,
        };

        Ok(Response::new(res))
    }

    type GetLatencyStreamStream = ReceiverStream<Result<LatencyResponse, Status>>;
    async fn get_latency_stream(
        &self,
        request: Request<LatencyStreamRequest>,
    ) -> Result<Response<LatencyResponse>, Status> {
        Ok(())
    }
}

fn main() {}
