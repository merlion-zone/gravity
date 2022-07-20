package keeper

import (
	"strings"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"

	"github.com/Gravity-Bridge/Gravity-Bridge/module/x/gravity/types"
	mgravitytypes "github.com/Gravity-Bridge/Gravity-Bridge/module/x/multigravity/types"
)

func RegisterProposalTypes() {
	// use of prefix stripping to prevent a typo between the proposal we check
	// and the one we register, any issues with the registration string will prevent
	// the proposal from working. We must check for double registration so that cli commands
	// submitting these proposals work.
	// For some reason the cli code is run during app.go startup, but of course app.go is not
	// run during operation of one off tx commands, so we need to run this 'twice'
	prefix := "multigravity/"
	updateChainParams := "multigravity/UpdateChainParams"
	if !govtypes.IsValidProposalType(strings.TrimPrefix(updateChainParams, prefix)) {
		govtypes.RegisterProposalType(mgravitytypes.ProposalTypeUpdateChainParams)
		govtypes.RegisterProposalTypeCodec(&mgravitytypes.UpdateChainParamsProposal{}, updateChainParams)
	}
	unhalt := "multigravity/UnhaltBridge"
	if !govtypes.IsValidProposalType(strings.TrimPrefix(unhalt, prefix)) {
		govtypes.RegisterProposalType(mgravitytypes.ProposalTypeUnhaltBridge)
		govtypes.RegisterProposalTypeCodec(&mgravitytypes.UnhaltBridgeProposal{}, unhalt)
	}
}

func NewGravityProposalHandler(k Keeper) govtypes.Handler {
	return func(ctx sdk.Context, content govtypes.Content) error {
		switch c := content.(type) {
		case *mgravitytypes.UpdateChainParamsProposal:
			return k.HandleUpdateChainParamsProposal(ctx, c)
		case *mgravitytypes.UnhaltBridgeProposal:
			subKeeper, err := k.SubKeeper(c.ChainIdentifier)
			if err != nil {
				return err
			}
			return subKeeper.HandleUnhaltBridgeProposal(ctx, &types.UnhaltBridgeProposal{
				Title:       c.Title,
				Description: c.Description,
				TargetNonce: c.TargetNonce,
			})
		default:
			return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "unrecognized proposal content type: %T", c)
		}
	}
}

func (k Keeper) HandleUpdateChainParamsProposal(ctx sdk.Context, p *mgravitytypes.UpdateChainParamsProposal) error {
	return k.UpdateSubKeeper(ctx, p.ChainIdentifier, p.Params)
}
