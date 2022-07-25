package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"

	"github.com/Gravity-Bridge/Gravity-Bridge/module/x/gravity/keeper"
)

func ModuleBalanceInvariant(k Keeper) sdk.Invariant {
	return func(ctx sdk.Context) (string, bool) {
		for _, subKeeper := range k.SubKeepers(ctx) {
			reason, broken := keeper.ModuleBalanceInvariant(subKeeper)(ctx)
			if broken {
				return reason, broken
			}
		}
		return "", false
	}
}
