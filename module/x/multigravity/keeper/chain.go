package keeper

import (
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"

	mgravitytypes "github.com/Gravity-Bridge/Gravity-Bridge/module/x/multigravity/types"
)

func (k Keeper) SetChain(ctx sdk.Context, chainIdentifier string) {
	store := ctx.KVStore(k.storeKey)
	store.Set(mgravitytypes.GetChainIdentifierKey(chainIdentifier), []byte{0x1})
}

func (k Keeper) HasChain(ctx sdk.Context, chainIdentifier string) bool {
	store := ctx.KVStore(k.storeKey)
	return store.Has(mgravitytypes.GetChainIdentifierKey(chainIdentifier))
}

func (k Keeper) AllChains(ctx sdk.Context) []string {
	prefixStore := prefix.NewStore(ctx.KVStore(k.storeKey), mgravitytypes.ChainIdentifierKey)
	iter := prefixStore.Iterator(nil, nil)
	defer iter.Close()

	var chains []string
	for ; iter.Valid(); iter.Next() {
		chains = append(chains, string(iter.Key()))
	}
	return chains
}

// func (k Keeper) SetChainParams(ctx sdk.Context, chainIdentifier string, params types.Params) {
// 	store := ctx.KVStore(k.storeKey)
// 	store.Set(mgravitytypes.GetChainParamsKey(chainIdentifier), k.cdc.MustMarshal(&params))
// }
//
// func (k Keeper) GetChainParams(ctx sdk.Context, chainIdentifier string) (types.Params, bool) {
// 	store := ctx.KVStore(k.storeKey)
// 	bz := store.Get(mgravitytypes.GetChainParamsKey(chainIdentifier))
// 	if bz == nil {
// 		return types.Params{}, false
// 	}
// 	var params types.Params
// 	k.cdc.MustUnmarshal(bz, &params)
// 	return params, true
// }
