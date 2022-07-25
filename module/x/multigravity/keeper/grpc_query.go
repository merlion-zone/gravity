package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"

	"github.com/Gravity-Bridge/Gravity-Bridge/module/x/gravity/types"
	mgravitytypes "github.com/Gravity-Bridge/Gravity-Bridge/module/x/multigravity/types"
)

var _ mgravitytypes.QueryServer = Keeper{}

func (k Keeper) Chains(c context.Context, req *mgravitytypes.QueryChainsRequest) (*mgravitytypes.QueryChainsResponse, error) {
	chains := k.AllChains(sdk.UnwrapSDKContext(c))
	return &mgravitytypes.QueryChainsResponse{chains}, nil
}

func (k Keeper) Params(c context.Context, req *mgravitytypes.QueryParamsRequest) (*mgravitytypes.QueryParamsResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.Params(c, &types.QueryParamsRequest{})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryParamsResponse{Params: rep.Params}, nil
}

func (k Keeper) CurrentValset(c context.Context, req *mgravitytypes.QueryCurrentValsetRequest) (*mgravitytypes.QueryCurrentValsetResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.CurrentValset(c, &types.QueryCurrentValsetRequest{})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryCurrentValsetResponse{Valset: rep.Valset}, nil
}

func (k Keeper) ValsetRequest(c context.Context, req *mgravitytypes.QueryValsetRequestRequest) (*mgravitytypes.QueryValsetRequestResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.ValsetRequest(c, &types.QueryValsetRequestRequest{Nonce: req.Nonce})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryValsetRequestResponse{Valset: rep.Valset}, nil
}

func (k Keeper) ValsetConfirm(c context.Context, req *mgravitytypes.QueryValsetConfirmRequest) (*mgravitytypes.QueryValsetConfirmResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.ValsetConfirm(c, &types.QueryValsetConfirmRequest{
		Nonce:   req.Nonce,
		Address: req.Address,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryValsetConfirmResponse{Confirm: rep.Confirm}, nil
}

func (k Keeper) ValsetConfirmsByNonce(c context.Context, req *mgravitytypes.QueryValsetConfirmsByNonceRequest) (*mgravitytypes.QueryValsetConfirmsByNonceResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.ValsetConfirmsByNonce(c, &types.QueryValsetConfirmsByNonceRequest{Nonce: req.Nonce})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryValsetConfirmsByNonceResponse{Confirms: rep.Confirms}, nil
}

func (k Keeper) LastValsetRequests(c context.Context, req *mgravitytypes.QueryLastValsetRequestsRequest) (*mgravitytypes.QueryLastValsetRequestsResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.LastValsetRequests(c, &types.QueryLastValsetRequestsRequest{})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryLastValsetRequestsResponse{Valsets: rep.Valsets}, nil
}

func (k Keeper) LastPendingValsetRequestByAddr(c context.Context, req *mgravitytypes.QueryLastPendingValsetRequestByAddrRequest) (*mgravitytypes.QueryLastPendingValsetRequestByAddrResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.LastPendingValsetRequestByAddr(c, &types.QueryLastPendingValsetRequestByAddrRequest{Address: req.Address})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryLastPendingValsetRequestByAddrResponse{Valsets: rep.Valsets}, nil
}

func (k Keeper) LastPendingBatchRequestByAddr(c context.Context, req *mgravitytypes.QueryLastPendingBatchRequestByAddrRequest) (*mgravitytypes.QueryLastPendingBatchRequestByAddrResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.LastPendingBatchRequestByAddr(c, &types.QueryLastPendingBatchRequestByAddrRequest{Address: req.Address})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryLastPendingBatchRequestByAddrResponse{Batch: rep.Batch}, nil
}

func (k Keeper) LastPendingLogicCallByAddr(c context.Context, req *mgravitytypes.QueryLastPendingLogicCallByAddrRequest) (*mgravitytypes.QueryLastPendingLogicCallByAddrResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.LastPendingLogicCallByAddr(c, &types.QueryLastPendingLogicCallByAddrRequest{Address: req.Address})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryLastPendingLogicCallByAddrResponse{Call: rep.Call}, nil
}

func (k Keeper) LastEventNonceByAddr(c context.Context, req *mgravitytypes.QueryLastEventNonceByAddrRequest) (*mgravitytypes.QueryLastEventNonceByAddrResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.LastEventNonceByAddr(c, &types.QueryLastEventNonceByAddrRequest{Address: req.Address})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryLastEventNonceByAddrResponse{EventNonce: rep.EventNonce}, nil
}

func (k Keeper) BatchFees(c context.Context, req *mgravitytypes.QueryBatchFeeRequest) (*mgravitytypes.QueryBatchFeeResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.BatchFees(c, &types.QueryBatchFeeRequest{})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryBatchFeeResponse{BatchFees: rep.BatchFees}, nil
}

func (k Keeper) OutgoingTxBatches(c context.Context, req *mgravitytypes.QueryOutgoingTxBatchesRequest) (*mgravitytypes.QueryOutgoingTxBatchesResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.OutgoingTxBatches(c, &types.QueryOutgoingTxBatchesRequest{})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryOutgoingTxBatchesResponse{Batches: rep.Batches}, nil
}

func (k Keeper) OutgoingLogicCalls(c context.Context, req *mgravitytypes.QueryOutgoingLogicCallsRequest) (*mgravitytypes.QueryOutgoingLogicCallsResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.OutgoingLogicCalls(c, &types.QueryOutgoingLogicCallsRequest{})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryOutgoingLogicCallsResponse{Calls: rep.Calls}, nil
}

func (k Keeper) BatchRequestByNonce(c context.Context, req *mgravitytypes.QueryBatchRequestByNonceRequest) (*mgravitytypes.QueryBatchRequestByNonceResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.BatchRequestByNonce(c, &types.QueryBatchRequestByNonceRequest{
		Nonce:           req.Nonce,
		ContractAddress: req.ContractAddress,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryBatchRequestByNonceResponse{Batch: rep.Batch}, nil
}

func (k Keeper) BatchConfirms(c context.Context, req *mgravitytypes.QueryBatchConfirmsRequest) (*mgravitytypes.QueryBatchConfirmsResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.BatchConfirms(c, &types.QueryBatchConfirmsRequest{
		Nonce:           req.Nonce,
		ContractAddress: req.ContractAddress,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryBatchConfirmsResponse{Confirms: rep.Confirms}, nil
}

func (k Keeper) LogicConfirms(c context.Context, req *mgravitytypes.QueryLogicConfirmsRequest) (*mgravitytypes.QueryLogicConfirmsResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.LogicConfirms(c, &types.QueryLogicConfirmsRequest{
		InvalidationId:    req.InvalidationId,
		InvalidationNonce: req.InvalidationNonce,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryLogicConfirmsResponse{Confirms: rep.Confirms}, nil
}

func (k Keeper) ERC20ToDenom(c context.Context, req *mgravitytypes.QueryERC20ToDenomRequest) (*mgravitytypes.QueryERC20ToDenomResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.ERC20ToDenom(c, &types.QueryERC20ToDenomRequest{Erc20: req.Erc20})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryERC20ToDenomResponse{
		Denom:            rep.Denom,
		CosmosOriginated: rep.CosmosOriginated,
	}, nil
}

func (k Keeper) DenomToERC20(c context.Context, req *mgravitytypes.QueryDenomToERC20Request) (*mgravitytypes.QueryDenomToERC20Response, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.DenomToERC20(c, &types.QueryDenomToERC20Request{Denom: req.Denom})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryDenomToERC20Response{
		Erc20:            rep.Erc20,
		CosmosOriginated: rep.CosmosOriginated,
	}, nil
}

func (k Keeper) GetAttestations(c context.Context, req *mgravitytypes.QueryAttestationsRequest) (*mgravitytypes.QueryAttestationsResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.GetAttestations(c, &types.QueryAttestationsRequest{
		Limit:     req.Limit,
		OrderBy:   req.OrderBy,
		ClaimType: req.ClaimType,
		Nonce:     req.Nonce,
		Height:    req.Height,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryAttestationsResponse{Attestations: rep.Attestations}, nil
}

func (k Keeper) GetDelegateKeyByValidator(c context.Context, req *types.QueryDelegateKeysByValidatorAddress) (*types.QueryDelegateKeysByValidatorAddressResponse, error) {
	subKeepers := k.SubKeepers(sdk.UnwrapSDKContext(c))
	if len(subKeepers) == 0 {
		return nil, sdkerrors.Wrap(mgravitytypes.ErrChainNotFound, "")
	}
	return subKeepers[0].GetDelegateKeyByValidator(c, req)
}

func (k Keeper) GetDelegateKeyByEth(c context.Context, req *types.QueryDelegateKeysByEthAddress) (*types.QueryDelegateKeysByEthAddressResponse, error) {
	subKeepers := k.SubKeepers(sdk.UnwrapSDKContext(c))
	if len(subKeepers) == 0 {
		return nil, sdkerrors.Wrap(mgravitytypes.ErrChainNotFound, "")
	}
	return subKeepers[0].GetDelegateKeyByEth(c, req)
}

func (k Keeper) GetDelegateKeyByOrchestrator(c context.Context, req *types.QueryDelegateKeysByOrchestratorAddress) (*types.QueryDelegateKeysByOrchestratorAddressResponse, error) {
	subKeepers := k.SubKeepers(sdk.UnwrapSDKContext(c))
	if len(subKeepers) == 0 {
		return nil, sdkerrors.Wrap(mgravitytypes.ErrChainNotFound, "")
	}
	return subKeepers[0].GetDelegateKeyByOrchestrator(c, req)
}

func (k Keeper) GetPendingSendToEth(c context.Context, req *mgravitytypes.QueryPendingSendToEth) (*mgravitytypes.QueryPendingSendToEthResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.GetPendingSendToEth(c, &types.QueryPendingSendToEth{SenderAddress: req.SenderAddress})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryPendingSendToEthResponse{
		TransfersInBatches: rep.TransfersInBatches,
		UnbatchedTransfers: rep.UnbatchedTransfers,
	}, nil
}

func (k Keeper) GetPendingIbcAutoForwards(c context.Context, req *mgravitytypes.QueryPendingIbcAutoForwards) (*mgravitytypes.QueryPendingIbcAutoForwardsResponse, error) {
	subKeeper, err := k.SubKeeper(sdk.UnwrapSDKContext(c), req.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	rep, err := subKeeper.GetPendingIbcAutoForwards(c, &types.QueryPendingIbcAutoForwards{Limit: req.Limit})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.QueryPendingIbcAutoForwardsResponse{PendingIbcAutoForwards: rep.PendingIbcAutoForwards}, nil
}
