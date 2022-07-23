//! This crate provides Gravity proto definitions in Rust and also re-exports cosmos_sdk_proto for use by downstream
//! crates. By default around a dozen proto files are generated and places into the prost folder. We could then proceed
//! to fix up all these files and use them as the required dependencies to the Gravity file, but we chose instead to replace
//! those paths with references ot upstream cosmos-sdk-proto and delete the other files. This reduces cruft in this repo even
//! if it does make for a somewhat more confusing proto generation process.

pub use cosmos_sdk_proto;

pub mod gv {
    include!("prost/gravity.v1.rs");
}

pub mod multigravity {
    include!("prost/multigravity.v1.rs");
}

pub mod gravity {
    pub use super::gv::{
        BridgeValidator,
        Valset,
        LastObservedEthereumBlockHeight,
        Erc20ToDenom,
        AirdropProposal,
        IbcMetadataProposal,
        PendingIbcAutoForward,
        EventSetOperatorAddress,
        EventValsetConfirmKey,
        EventBatchCreated,
        EventBatchConfirmKey,
        EventBatchSendToEthClaim,
        EventClaim,
        EventBadSignatureEvidence,
        EventErc20DeployedClaim,
        EventValsetUpdatedClaim,
        EventMultisigUpdateRequest,
        EventOutgoingLogicCallCanceled,
        EventSignatureSlashing,
        EventOutgoingTxId,
        Attestation,
        Erc20Token,
        EventObservation,
        EventInvalidSendToCosmosReceiver,
        EventSendToCosmos,
        EventSendToCosmosLocal,
        EventSendToCosmosPendingIbcAutoForward,
        EventSendToCosmosExecutedIbcAutoForward,
        ClaimType,
        OutgoingTxBatch,
        OutgoingTransferTx,
        OutgoingLogicCall,
        EventOutgoingBatchCanceled,
        EventOutgoingBatch,
        Params,
        GenesisState,
        GravityNonces,
        IdSet,
        BatchFees,
        EventWithdrawalReceived,
        EventWithdrawCanceled,
        SignType,
        MsgSetOrchestratorAddress,
        MsgSetOrchestratorAddressResponse,
        QueryDelegateKeysByValidatorAddress,
        QueryDelegateKeysByValidatorAddressResponse,
        QueryDelegateKeysByEthAddress,
        QueryDelegateKeysByEthAddressResponse,
        QueryDelegateKeysByOrchestratorAddress,
        QueryDelegateKeysByOrchestratorAddressResponse,
    };
    pub use super::multigravity::*;
}
