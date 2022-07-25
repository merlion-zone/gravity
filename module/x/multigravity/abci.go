package multigravity

import (
	sdk "github.com/cosmos/cosmos-sdk/types"

	"github.com/Gravity-Bridge/Gravity-Bridge/module/x/gravity"
	mgravitykeeper "github.com/Gravity-Bridge/Gravity-Bridge/module/x/multigravity/keeper"
)

// EndBlocker is called at the end of every block
func EndBlocker(ctx sdk.Context, k mgravitykeeper.Keeper) {
	for _, keeper := range k.SubKeepers(ctx) {
		gravity.Slashing(ctx, keeper)
		gravity.AttestationTally(ctx, keeper)
		gravity.CleanupTimedOutBatches(ctx, keeper)
		gravity.CleanupTimedOutLogicCalls(ctx, keeper)
		gravity.CreateValsetRequest(ctx, keeper)
		gravity.PruneValsets(ctx, keeper)
		gravity.PruneAttestations(ctx, keeper)
	}
}
