package keeper

import (
	"context"

	"github.com/Gravity-Bridge/Gravity-Bridge/module/x/gravity/types"
	mgravitytypes "github.com/Gravity-Bridge/Gravity-Bridge/module/x/multigravity/types"
)

type msgServer struct {
	Keeper
}

// NewMsgServerImpl returns an implementation of the gov MsgServer interface
// for the provided Keeper.
func NewMsgServerImpl(keeper Keeper) mgravitytypes.MsgServer {
	return &msgServer{Keeper: keeper}
}

func (m msgServer) SetOrchestratorAddress(c context.Context, msg *types.MsgSetOrchestratorAddress) (*types.MsgSetOrchestratorAddressResponse, error) {
	for _, server := range m.SubKeeperServers() {
		_, err := server.SetOrchestratorAddress(c, msg)
		if err != nil {
			return nil, err
		}
	}
	return &types.MsgSetOrchestratorAddressResponse{}, nil
}

func (m msgServer) ValsetConfirm(c context.Context, msg *mgravitytypes.MsgValsetConfirm) (*mgravitytypes.MsgValsetConfirmResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.ValsetConfirm(c, &types.MsgValsetConfirm{
		Nonce:        msg.Nonce,
		Orchestrator: msg.Orchestrator,
		EthAddress:   msg.EthAddress,
		Signature:    msg.Signature,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgValsetConfirmResponse{}, nil
}

func (m msgServer) SendToEth(c context.Context, msg *mgravitytypes.MsgSendToEth) (*mgravitytypes.MsgSendToEthResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.SendToEth(c, &types.MsgSendToEth{
		Sender:    msg.Sender,
		EthDest:   msg.EthDest,
		Amount:    msg.Amount,
		BridgeFee: msg.BridgeFee,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgSendToEthResponse{}, nil
}

func (m msgServer) RequestBatch(c context.Context, msg *mgravitytypes.MsgRequestBatch) (*mgravitytypes.MsgRequestBatchResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.RequestBatch(c, &types.MsgRequestBatch{
		Sender: msg.Sender,
		Denom:  msg.Denom,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgRequestBatchResponse{}, err
}

func (m msgServer) ConfirmBatch(c context.Context, msg *mgravitytypes.MsgConfirmBatch) (*mgravitytypes.MsgConfirmBatchResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.ConfirmBatch(c, &types.MsgConfirmBatch{
		Nonce:         msg.Nonce,
		TokenContract: msg.TokenContract,
		EthSigner:     msg.EthSigner,
		Orchestrator:  msg.Orchestrator,
		Signature:     msg.Signature,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgConfirmBatchResponse{}, err
}

func (m msgServer) ConfirmLogicCall(c context.Context, msg *mgravitytypes.MsgConfirmLogicCall) (*mgravitytypes.MsgConfirmLogicCallResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.ConfirmLogicCall(c, &types.MsgConfirmLogicCall{
		InvalidationId:    msg.InvalidationId,
		InvalidationNonce: msg.InvalidationNonce,
		EthSigner:         msg.EthSigner,
		Orchestrator:      msg.Orchestrator,
		Signature:         msg.Signature,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgConfirmLogicCallResponse{}, err
}

func (m msgServer) SendToCosmosClaim(c context.Context, msg *mgravitytypes.MsgSendToCosmosClaim) (*mgravitytypes.MsgSendToCosmosClaimResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.SendToCosmosClaim(c, &types.MsgSendToCosmosClaim{
		EventNonce:     msg.EventNonce,
		BlockHeight:    msg.BlockHeight,
		TokenContract:  msg.TokenContract,
		Amount:         msg.Amount,
		EthereumSender: msg.EthereumSender,
		CosmosReceiver: msg.CosmosReceiver,
		Orchestrator:   msg.Orchestrator,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgSendToCosmosClaimResponse{}, err
}

func (m msgServer) ExecuteIbcAutoForwards(c context.Context, msg *mgravitytypes.MsgExecuteIbcAutoForwards) (*mgravitytypes.MsgExecuteIbcAutoForwardsResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.ExecuteIbcAutoForwards(c, &types.MsgExecuteIbcAutoForwards{
		ForwardsToClear: msg.ForwardsToClear,
		Executor:        msg.Executor,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgExecuteIbcAutoForwardsResponse{}, err
}

func (m msgServer) BatchSendToEthClaim(c context.Context, msg *mgravitytypes.MsgBatchSendToEthClaim) (*mgravitytypes.MsgBatchSendToEthClaimResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.BatchSendToEthClaim(c, &types.MsgBatchSendToEthClaim{
		EventNonce:    msg.EventNonce,
		BlockHeight:   msg.BlockHeight,
		BatchNonce:    msg.BatchNonce,
		TokenContract: msg.TokenContract,
		Orchestrator:  msg.Orchestrator,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgBatchSendToEthClaimResponse{}, err
}

func (m msgServer) ValsetUpdateClaim(c context.Context, msg *mgravitytypes.MsgValsetUpdatedClaim) (*mgravitytypes.MsgValsetUpdatedClaimResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.ValsetUpdateClaim(c, &types.MsgValsetUpdatedClaim{
		EventNonce:   msg.EventNonce,
		ValsetNonce:  msg.ValsetNonce,
		BlockHeight:  msg.BlockHeight,
		Members:      msg.Members,
		RewardAmount: msg.RewardAmount,
		RewardToken:  msg.RewardToken,
		Orchestrator: msg.Orchestrator,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgValsetUpdatedClaimResponse{}, err
}

func (m msgServer) ERC20DeployedClaim(c context.Context, msg *mgravitytypes.MsgERC20DeployedClaim) (*mgravitytypes.MsgERC20DeployedClaimResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.ERC20DeployedClaim(c, &types.MsgERC20DeployedClaim{
		EventNonce:    msg.EventNonce,
		BlockHeight:   msg.BlockHeight,
		CosmosDenom:   msg.CosmosDenom,
		TokenContract: msg.TokenContract,
		Name:          msg.Name,
		Symbol:        msg.Symbol,
		Decimals:      msg.Decimals,
		Orchestrator:  msg.Orchestrator,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgERC20DeployedClaimResponse{}, err
}

func (m msgServer) LogicCallExecutedClaim(c context.Context, msg *mgravitytypes.MsgLogicCallExecutedClaim) (*mgravitytypes.MsgLogicCallExecutedClaimResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.LogicCallExecutedClaim(c, &types.MsgLogicCallExecutedClaim{
		EventNonce:        msg.EventNonce,
		BlockHeight:       msg.BlockHeight,
		InvalidationId:    msg.InvalidationId,
		InvalidationNonce: msg.InvalidationNonce,
		Orchestrator:      msg.Orchestrator,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgLogicCallExecutedClaimResponse{}, err
}

func (m msgServer) CancelSendToEth(c context.Context, msg *mgravitytypes.MsgCancelSendToEth) (*mgravitytypes.MsgCancelSendToEthResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.CancelSendToEth(c, &types.MsgCancelSendToEth{
		TransactionId: msg.TransactionId,
		Sender:        msg.Sender,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgCancelSendToEthResponse{}, err
}

func (m msgServer) SubmitBadSignatureEvidence(c context.Context, msg *mgravitytypes.MsgSubmitBadSignatureEvidence) (*mgravitytypes.MsgSubmitBadSignatureEvidenceResponse, error) {
	server, err := m.SubKeeperServer(msg.ChainIdentifier)
	if err != nil {
		return nil, err
	}
	_, err = server.SubmitBadSignatureEvidence(c, &types.MsgSubmitBadSignatureEvidence{
		Subject:   msg.Subject,
		Signature: msg.Signature,
		Sender:    msg.Sender,
	})
	if err != nil {
		return nil, err
	}
	return &mgravitytypes.MsgSubmitBadSignatureEvidenceResponse{}, err
}
