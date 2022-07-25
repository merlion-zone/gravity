package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"

	"github.com/Gravity-Bridge/Gravity-Bridge/module/x/gravity/types"
)

// ModuleCdc is the codec for the module
var ModuleCdc = codec.NewLegacyAmino()

func init() {
	RegisterCodec(ModuleCdc)
}

// nolint: exhaustruct
// RegisterInterfaces registers the interfaces for the proto stuff
func RegisterInterfaces(registry codectypes.InterfaceRegistry) {
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgValsetConfirm{},
		&MsgSendToEth{},
		&MsgRequestBatch{},
		&MsgConfirmBatch{},
		&MsgConfirmLogicCall{},
		&MsgSendToCosmosClaim{},
		&MsgBatchSendToEthClaim{},
		&MsgERC20DeployedClaim{},
		&types.MsgSetOrchestratorAddress{},
		&MsgLogicCallExecutedClaim{},
		&MsgValsetUpdatedClaim{},
		&MsgCancelSendToEth{},
		&MsgSubmitBadSignatureEvidence{},
	)

	registry.RegisterInterface(
		"gravity.v1beta1.EthereumClaim",
		(*types.EthereumClaim)(nil),
		&MsgSendToCosmosClaim{},
		&MsgBatchSendToEthClaim{},
		&MsgERC20DeployedClaim{},
		&MsgLogicCallExecutedClaim{},
		&MsgValsetUpdatedClaim{},
	)

	registry.RegisterImplementations((*govtypes.Content)(nil), &UpdateChainParamsProposal{}, &UnhaltBridgeProposal{})

	registry.RegisterInterface("gravity.v1beta1.EthereumSigned", (*types.EthereumSigned)(nil), &types.Valset{}, &types.OutgoingTxBatch{}, &types.OutgoingLogicCall{})

	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)
}

// nolint: exhaustruct
// RegisterCodec registers concrete types on the Amino codec
func RegisterCodec(cdc *codec.LegacyAmino) {
	types.RegisterCodec(cdc)

	cdc.RegisterConcrete(&MsgValsetConfirm{}, "multigravity/MsgValsetConfirm", nil)
	cdc.RegisterConcrete(&MsgSendToEth{}, "multigravity/MsgSendToEth", nil)
	cdc.RegisterConcrete(&MsgRequestBatch{}, "multigravity/MsgRequestBatch", nil)
	cdc.RegisterConcrete(&MsgConfirmBatch{}, "multigravity/MsgConfirmBatch", nil)
	cdc.RegisterConcrete(&MsgConfirmLogicCall{}, "multigravity/MsgConfirmLogicCall", nil)
	cdc.RegisterConcrete(&MsgSendToCosmosClaim{}, "multigravity/MsgSendToCosmosClaim", nil)
	cdc.RegisterConcrete(&MsgBatchSendToEthClaim{}, "multigravity/MsgBatchSendToEthClaim", nil)
	cdc.RegisterConcrete(&MsgERC20DeployedClaim{}, "multigravity/MsgERC20DeployedClaim", nil)
	cdc.RegisterConcrete(&MsgLogicCallExecutedClaim{}, "multigravity/MsgLogicCallExecutedClaim", nil)
	cdc.RegisterConcrete(&MsgValsetUpdatedClaim{}, "multigravity/MsgValsetUpdatedClaim", nil)
	cdc.RegisterConcrete(&MsgCancelSendToEth{}, "multigravity/MsgCancelSendToEth", nil)
	cdc.RegisterConcrete(&MsgSubmitBadSignatureEvidence{}, "multigravity/MsgSubmitBadSignatureEvidence", nil)
}
