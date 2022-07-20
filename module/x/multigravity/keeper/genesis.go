package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"

	mgravitytypes "github.com/Gravity-Bridge/Gravity-Bridge/module/x/multigravity/types"
)

// InitGenesis starts a chain from a genesis state
func InitGenesis(ctx sdk.Context, k Keeper, data mgravitytypes.GenesisState) {
}

// ExportGenesis exports all the state needed to restart the chain
// from the current state of the chain
func ExportGenesis(ctx sdk.Context, k Keeper) mgravitytypes.GenesisState {
	return mgravitytypes.GenesisState{}
}
