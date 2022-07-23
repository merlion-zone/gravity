#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChainsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChainsResponse {
    #[prost(string, repeated, tag="1")]
    pub chain_identifiers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
    #[prost(string, tag="1")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<super::gravity::Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentValsetRequest {
    #[prost(string, tag="1")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentValsetResponse {
    #[prost(message, optional, tag="1")]
    pub valset: ::core::option::Option<super::gravity::Valset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetRequestRequest {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetRequestResponse {
    #[prost(message, optional, tag="1")]
    pub valset: ::core::option::Option<super::gravity::Valset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmRequest {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmResponse {
    #[prost(message, optional, tag="1")]
    pub confirm: ::core::option::Option<super::gravity::MsgValsetConfirm>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmsByNonceRequest {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmsByNonceResponse {
    #[prost(message, repeated, tag="1")]
    pub confirms: ::prost::alloc::vec::Vec<super::gravity::MsgValsetConfirm>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastValsetRequestsRequest {
    #[prost(string, tag="1")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastValsetRequestsResponse {
    #[prost(message, repeated, tag="1")]
    pub valsets: ::prost::alloc::vec::Vec<super::gravity::Valset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingValsetRequestByAddrRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingValsetRequestByAddrResponse {
    #[prost(message, repeated, tag="1")]
    pub valsets: ::prost::alloc::vec::Vec<super::gravity::Valset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchFeeRequest {
    #[prost(string, tag="1")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchFeeResponse {
    #[prost(message, repeated, tag="1")]
    pub batch_fees: ::prost::alloc::vec::Vec<super::gravity::BatchFees>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingBatchRequestByAddrRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingBatchRequestByAddrResponse {
    #[prost(message, repeated, tag="1")]
    pub batch: ::prost::alloc::vec::Vec<super::gravity::OutgoingTxBatch>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingLogicCallByAddrRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingLogicCallByAddrResponse {
    #[prost(message, repeated, tag="1")]
    pub call: ::prost::alloc::vec::Vec<super::gravity::OutgoingLogicCall>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutgoingTxBatchesRequest {
    #[prost(string, tag="1")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutgoingTxBatchesResponse {
    #[prost(message, repeated, tag="1")]
    pub batches: ::prost::alloc::vec::Vec<super::gravity::OutgoingTxBatch>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutgoingLogicCallsRequest {
    #[prost(string, tag="1")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutgoingLogicCallsResponse {
    #[prost(message, repeated, tag="1")]
    pub calls: ::prost::alloc::vec::Vec<super::gravity::OutgoingLogicCall>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchRequestByNonceRequest {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchRequestByNonceResponse {
    #[prost(message, optional, tag="1")]
    pub batch: ::core::option::Option<super::gravity::OutgoingTxBatch>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchConfirmsRequest {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchConfirmsResponse {
    #[prost(message, repeated, tag="1")]
    pub confirms: ::prost::alloc::vec::Vec<super::gravity::MsgConfirmBatch>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLogicConfirmsRequest {
    #[prost(bytes="vec", tag="1")]
    pub invalidation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub invalidation_nonce: u64,
    #[prost(string, tag="3")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLogicConfirmsResponse {
    #[prost(message, repeated, tag="1")]
    pub confirms: ::prost::alloc::vec::Vec<super::gravity::MsgConfirmLogicCall>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastEventNonceByAddrRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastEventNonceByAddrResponse {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErc20ToDenomRequest {
    #[prost(string, tag="1")]
    pub erc20: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErc20ToDenomResponse {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub cosmos_originated: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomToErc20Request {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomToErc20Response {
    #[prost(string, tag="1")]
    pub erc20: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub cosmos_originated: bool,
}
/// QueryAttestationsRequest defines the request structure for getting recent
/// attestations with optional query parameters. By default, a limited set of
/// recent attestations will be returned, defined by 'limit'. These attestations
/// can be ordered ascending or descending by nonce, that defaults to ascending.
/// Filtering criteria may also be provided, including nonce, claim type, and
/// height. Note, that an attestation will be returned if it matches ANY of the
/// filter query parameters provided.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationsRequest {
    /// limit defines how many attestations to limit in the response.
    #[prost(uint64, tag="1")]
    pub limit: u64,
    /// order_by provides ordering of atteststions by nonce in the response. Either
    /// 'asc' or 'desc' can be provided. If no value is provided, it defaults to
    /// 'asc'.
    #[prost(string, tag="2")]
    pub order_by: ::prost::alloc::string::String,
    /// claim_type allows filtering attestations by Ethereum claim type.
    #[prost(string, tag="3")]
    pub claim_type: ::prost::alloc::string::String,
    /// nonce allows filtering attestations by Ethereum claim nonce.
    #[prost(uint64, tag="4")]
    pub nonce: u64,
    /// height allows filtering attestations by Ethereum claim height.
    #[prost(uint64, tag="5")]
    pub height: u64,
    #[prost(string, tag="6")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationsResponse {
    #[prost(message, repeated, tag="1")]
    pub attestations: ::prost::alloc::vec::Vec<super::gravity::Attestation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSendToEth {
    #[prost(string, tag="1")]
    pub sender_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSendToEthResponse {
    #[prost(message, repeated, tag="1")]
    pub transfers_in_batches: ::prost::alloc::vec::Vec<super::gravity::OutgoingTransferTx>,
    #[prost(message, repeated, tag="2")]
    pub unbatched_transfers: ::prost::alloc::vec::Vec<super::gravity::OutgoingTransferTx>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingIbcAutoForwards {
    /// limit defines the number of pending forwards to return, in order of their SendToCosmos.EventNonce
    #[prost(uint64, tag="1")]
    pub limit: u64,
    #[prost(string, tag="2")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingIbcAutoForwardsResponse {
    #[prost(message, repeated, tag="1")]
    pub pending_ibc_auto_forwards: ::prost::alloc::vec::Vec<super::gravity::PendingIbcAutoForward>,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Query defines the gRPC querier service
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn chains(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryChainsRequest>,
        ) -> Result<tonic::Response<super::QueryChainsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/Chains",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/Params",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn current_valset(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCurrentValsetRequest>,
        ) -> Result<tonic::Response<super::QueryCurrentValsetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/CurrentValset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn valset_request(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValsetRequestRequest>,
        ) -> Result<tonic::Response<super::QueryValsetRequestResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/ValsetRequest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn valset_confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValsetConfirmRequest>,
        ) -> Result<tonic::Response<super::QueryValsetConfirmResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/ValsetConfirm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn valset_confirms_by_nonce(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValsetConfirmsByNonceRequest>,
        ) -> Result<
                tonic::Response<super::QueryValsetConfirmsByNonceResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/ValsetConfirmsByNonce",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn last_valset_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLastValsetRequestsRequest>,
        ) -> Result<
                tonic::Response<super::QueryLastValsetRequestsResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/LastValsetRequests",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn last_pending_valset_request_by_addr(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryLastPendingValsetRequestByAddrRequest,
            >,
        ) -> Result<
                tonic::Response<super::QueryLastPendingValsetRequestByAddrResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/LastPendingValsetRequestByAddr",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn last_pending_batch_request_by_addr(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryLastPendingBatchRequestByAddrRequest,
            >,
        ) -> Result<
                tonic::Response<super::QueryLastPendingBatchRequestByAddrResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/LastPendingBatchRequestByAddr",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn last_pending_logic_call_by_addr(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryLastPendingLogicCallByAddrRequest,
            >,
        ) -> Result<
                tonic::Response<super::QueryLastPendingLogicCallByAddrResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/LastPendingLogicCallByAddr",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn last_event_nonce_by_addr(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLastEventNonceByAddrRequest>,
        ) -> Result<
                tonic::Response<super::QueryLastEventNonceByAddrResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/LastEventNonceByAddr",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn batch_fees(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBatchFeeRequest>,
        ) -> Result<tonic::Response<super::QueryBatchFeeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/BatchFees",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn outgoing_tx_batches(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOutgoingTxBatchesRequest>,
        ) -> Result<
                tonic::Response<super::QueryOutgoingTxBatchesResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/OutgoingTxBatches",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn outgoing_logic_calls(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOutgoingLogicCallsRequest>,
        ) -> Result<
                tonic::Response<super::QueryOutgoingLogicCallsResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/OutgoingLogicCalls",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn batch_request_by_nonce(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBatchRequestByNonceRequest>,
        ) -> Result<
                tonic::Response<super::QueryBatchRequestByNonceResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/BatchRequestByNonce",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn batch_confirms(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBatchConfirmsRequest>,
        ) -> Result<tonic::Response<super::QueryBatchConfirmsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/BatchConfirms",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn logic_confirms(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLogicConfirmsRequest>,
        ) -> Result<tonic::Response<super::QueryLogicConfirmsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/LogicConfirms",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn erc20_to_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryErc20ToDenomRequest>,
        ) -> Result<tonic::Response<super::QueryErc20ToDenomResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/ERC20ToDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn denom_to_erc20(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomToErc20Request>,
        ) -> Result<tonic::Response<super::QueryDenomToErc20Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/DenomToERC20",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_attestations(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAttestationsRequest>,
        ) -> Result<tonic::Response<super::QueryAttestationsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/GetAttestations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_delegate_key_by_validator(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::gravity::QueryDelegateKeysByValidatorAddress,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::gravity::QueryDelegateKeysByValidatorAddressResponse,
                >,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/GetDelegateKeyByValidator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_delegate_key_by_eth(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::gravity::QueryDelegateKeysByEthAddress,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::gravity::QueryDelegateKeysByEthAddressResponse,
                >,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/GetDelegateKeyByEth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_delegate_key_by_orchestrator(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::gravity::QueryDelegateKeysByOrchestratorAddress,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::gravity::QueryDelegateKeysByOrchestratorAddressResponse,
                >,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/GetDelegateKeyByOrchestrator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_pending_send_to_eth(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPendingSendToEth>,
        ) -> Result<
                tonic::Response<super::QueryPendingSendToEthResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/GetPendingSendToEth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_pending_ibc_auto_forwards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPendingIbcAutoForwards>,
        ) -> Result<
                tonic::Response<super::QueryPendingIbcAutoForwardsResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Query/GetPendingIbcAutoForwards",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// MsgValsetConfirm
/// this is the message sent by the validators when they wish to submit their
/// signatures over the validator set at a given block height. A validator must
/// first call MsgSetEthAddress to set their Ethereum address to be used for
/// signing. Then someone (anyone) must make a ValsetRequest, the request is
/// essentially a messaging mechanism to determine which block all validators
/// should submit signatures over. Finally validators sign the validator set,
/// powers, and Ethereum addresses of the entire validator set at the height of a
/// ValsetRequest and submit that signature with this message.
///
/// If a sufficient number of validators (66% of voting power) (A) have set
/// Ethereum addresses and (B) submit ValsetConfirm messages with their
/// signatures it is then possible for anyone to view these signatures in the
/// chain store and submit them to Ethereum to update the validator set
/// -------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetConfirm {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub eth_address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetConfirmResponse {
}
/// MsgSendToEth
/// This is the message that a user calls when they want to bridge an asset
/// it will later be removed when it is included in a batch and successfully
/// submitted tokens are removed from the users balance immediately
/// -------------
/// AMOUNT:
/// the coin to send across the bridge, note the restriction that this is a
/// single coin not a set of coins that is normal in other Cosmos messages
/// FEE:
/// the fee paid for the bridge, distinct from the fee paid to the chain to
/// actually send this message in the first place. So a successful send has
/// two layers of fees for the user
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendToEth {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub eth_dest: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="4")]
    pub bridge_fee: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="5")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendToEthResponse {
}
/// MsgRequestBatch
/// this is a message anyone can send that requests a batch of transactions to
/// send across the bridge be created for whatever block height this message is
/// included in. This acts as a coordination point, the handler for this message
/// looks at the AddToOutgoingPool tx's in the store and generates a batch, also
/// available in the store tied to this message. The validators then grab this
/// batch, sign it, submit the signatures with a MsgConfirmBatch before a relayer
/// can finally submit the batch
/// -------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBatch {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBatchResponse {
}
/// MsgConfirmBatch
/// When validators observe a MsgRequestBatch they form a batch by ordering
/// transactions currently in the txqueue in order of highest to lowest fee,
/// cutting off when the batch either reaches a hardcoded maximum size (to be
/// decided, probably around 100) or when transactions stop being profitable
/// (TODO determine this without nondeterminism) This message includes the batch
/// as well as an Ethereum signature over this batch by the validator
/// -------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfirmBatch {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub eth_signer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfirmBatchResponse {
}
/// MsgConfirmLogicCall
/// When validators observe a MsgRequestBatch they form a batch by ordering
/// transactions currently in the txqueue in order of highest to lowest fee,
/// cutting off when the batch either reaches a hardcoded maximum size (to be
/// decided, probably around 100) or when transactions stop being profitable
/// (TODO determine this without nondeterminism) This message includes the batch
/// as well as an Ethereum signature over this batch by the validator
/// -------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfirmLogicCall {
    #[prost(string, tag="1")]
    pub invalidation_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub invalidation_nonce: u64,
    #[prost(string, tag="3")]
    pub eth_signer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfirmLogicCallResponse {
}
/// MsgSendToCosmosClaim
/// When more than 66% of the active validator set has
/// claimed to have seen the deposit enter the ethereum blockchain coins are
/// issued to the Cosmos address in question
/// -------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendToCosmosClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub ethereum_sender: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub cosmos_receiver: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendToCosmosClaimResponse {
}
/// MsgExecuteIbcAutoForwards
/// Prompts the forwarding of Pending IBC Auto-Forwards in the queue
/// The Pending forwards will be executed in order of their original SendToCosmos.EventNonce
/// The funds in the queue will be sent to a local gravity-prefixed address if IBC transfer is not possible
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteIbcAutoForwards {
    /// How many queued forwards to clear, be careful about gas limits
    #[prost(uint64, tag="1")]
    pub forwards_to_clear: u64,
    /// This message's sender
    #[prost(string, tag="2")]
    pub executor: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteIbcAutoForwardsResponse {
}
/// BatchSendToEthClaim claims that a batch of send to eth
/// operations on the bridge contract was executed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchSendToEthClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(uint64, tag="3")]
    pub batch_nonce: u64,
    #[prost(string, tag="4")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchSendToEthClaimResponse {
}
/// ERC20DeployedClaim allows the Cosmos module
/// to learn about an ERC20 that someone deployed
/// to represent a Cosmos asset
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgErc20DeployedClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub cosmos_denom: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub decimals: u64,
    #[prost(string, tag="8")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgErc20DeployedClaimResponse {
}
/// This informs the Cosmos module that a logic
/// call has been executed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLogicCallExecutedClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(bytes="vec", tag="3")]
    pub invalidation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub invalidation_nonce: u64,
    #[prost(string, tag="5")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLogicCallExecutedClaimResponse {
}
/// This informs the Cosmos module that a validator
/// set has been updated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetUpdatedClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub valset_nonce: u64,
    #[prost(uint64, tag="3")]
    pub block_height: u64,
    #[prost(message, repeated, tag="4")]
    pub members: ::prost::alloc::vec::Vec<super::gravity::BridgeValidator>,
    #[prost(string, tag="5")]
    pub reward_amount: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub reward_token: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetUpdatedClaimResponse {
}
/// This call allows the sender (and only the sender)
/// to cancel a given MsgSendToEth and recieve a refund
/// of the tokens
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSendToEth {
    #[prost(uint64, tag="1")]
    pub transaction_id: u64,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSendToEthResponse {
}
/// This call allows anyone to submit evidence that a
/// validator has signed a valset, batch, or logic call that never
/// existed on the Cosmos chain. 
/// Subject contains the batch, valset, or logic call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitBadSignatureEvidence {
    #[prost(message, optional, tag="1")]
    pub subject: ::core::option::Option<::prost_types::Any>,
    #[prost(string, tag="2")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub chain_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitBadSignatureEvidenceResponse {
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Msg defines the state transitions possible within gravity
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn valset_confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgValsetConfirm>,
        ) -> Result<tonic::Response<super::MsgValsetConfirmResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/ValsetConfirm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_to_eth(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSendToEth>,
        ) -> Result<tonic::Response<super::MsgSendToEthResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/SendToEth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn request_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRequestBatch>,
        ) -> Result<tonic::Response<super::MsgRequestBatchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/RequestBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn confirm_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConfirmBatch>,
        ) -> Result<tonic::Response<super::MsgConfirmBatchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/ConfirmBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn confirm_logic_call(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConfirmLogicCall>,
        ) -> Result<tonic::Response<super::MsgConfirmLogicCallResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/ConfirmLogicCall",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_to_cosmos_claim(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSendToCosmosClaim>,
        ) -> Result<
                tonic::Response<super::MsgSendToCosmosClaimResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/SendToCosmosClaim",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn execute_ibc_auto_forwards(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExecuteIbcAutoForwards>,
        ) -> Result<
                tonic::Response<super::MsgExecuteIbcAutoForwardsResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/ExecuteIbcAutoForwards",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn batch_send_to_eth_claim(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBatchSendToEthClaim>,
        ) -> Result<
                tonic::Response<super::MsgBatchSendToEthClaimResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/BatchSendToEthClaim",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn valset_update_claim(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgValsetUpdatedClaim>,
        ) -> Result<
                tonic::Response<super::MsgValsetUpdatedClaimResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/ValsetUpdateClaim",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn erc20_deployed_claim(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgErc20DeployedClaim>,
        ) -> Result<
                tonic::Response<super::MsgErc20DeployedClaimResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/ERC20DeployedClaim",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn logic_call_executed_claim(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLogicCallExecutedClaim>,
        ) -> Result<
                tonic::Response<super::MsgLogicCallExecutedClaimResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/LogicCallExecutedClaim",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_orchestrator_address(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::gravity::MsgSetOrchestratorAddress,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::gravity::MsgSetOrchestratorAddressResponse,
                >,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/SetOrchestratorAddress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cancel_send_to_eth(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelSendToEth>,
        ) -> Result<tonic::Response<super::MsgCancelSendToEthResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/CancelSendToEth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn submit_bad_signature_evidence(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubmitBadSignatureEvidence>,
        ) -> Result<
                tonic::Response<super::MsgSubmitBadSignatureEvidenceResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/multigravity.v1.Msg/SubmitBadSignatureEvidence",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
}
/// GenesisState struct, containing all persistent data required by the Gravity module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChainParamsProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub chain_identifier: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub params: ::core::option::Option<super::gravity::Params>,
}
/// UnhaltBridgeProposal defines a custom governance proposal useful for restoring
/// the bridge after a oracle disagreement. Once this proposal is passed bridge state will roll back events
/// to the nonce provided in target_nonce if and only if those events have not yet been observed (executed on the Cosmos chain). This allows for easy
/// handling of cases where for example an Ethereum hardfork has occured and more than 1/3 of the vlaidtor set
/// disagrees with the rest. Normally this would require a chain halt, manual genesis editing and restar to resolve
/// with this feature a governance proposal can be used instead
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnhaltBridgeProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub target_nonce: u64,
    #[prost(string, tag="4")]
    pub chain_identifier: ::prost::alloc::string::String,
}
