package keeper

import (
	"errors"
	"fmt"
	"sort"
	"strings"

	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	distrkeeper "github.com/cosmos/cosmos-sdk/x/distribution/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	slashingkeeper "github.com/cosmos/cosmos-sdk/x/slashing/keeper"
	ibctransferkeeper "github.com/cosmos/ibc-go/v3/modules/apps/transfer/keeper"
	bech32ibckeeper "github.com/osmosis-labs/bech32-ibc/x/bech32ibc/keeper"
	"github.com/tendermint/tendermint/libs/log"

	"github.com/Gravity-Bridge/Gravity-Bridge/module/x/gravity/keeper"
	"github.com/Gravity-Bridge/Gravity-Bridge/module/x/gravity/types"
	mgravitytypes "github.com/Gravity-Bridge/Gravity-Bridge/module/x/multigravity/types"
)

type Keeper struct {
	storeKey   sdk.StoreKey
	paramSpace paramtypes.Subspace

	cdc               codec.BinaryCodec // The wire codec for binary encoding/decoding.
	bankKeeper        bankkeeper.Keeper
	StakingKeeper     types.StakingKeeper
	SlashingKeeper    *slashingkeeper.Keeper
	DistKeeper        *distrkeeper.Keeper
	accountKeeper     *authkeeper.AccountKeeper
	ibcTransferKeeper *ibctransferkeeper.Keeper
	bech32IbcKeeper   *bech32ibckeeper.Keeper
	paramsKeeper      *paramskeeper.Keeper

	app *baseapp.BaseApp
}

func NewKeeper(
	app *baseapp.BaseApp,
	storeKey sdk.StoreKey,
	paramSpace paramtypes.Subspace,
	cdc codec.BinaryCodec,
	bankKeeper bankkeeper.Keeper,
	stakingKeeper types.StakingKeeper,
	slashingKeeper *slashingkeeper.Keeper,
	distKeeper *distrkeeper.Keeper,
	accKeeper *authkeeper.AccountKeeper,
	ibcTransferKeeper *ibctransferkeeper.Keeper,
	bech32IbcKeeper *bech32ibckeeper.Keeper,
	paramsKeeper *paramskeeper.Keeper,
) Keeper {
	k := Keeper{
		storeKey:   storeKey,
		paramSpace: paramSpace,

		cdc:               cdc,
		bankKeeper:        bankKeeper,
		StakingKeeper:     stakingKeeper,
		SlashingKeeper:    slashingKeeper,
		DistKeeper:        distKeeper,
		accountKeeper:     accKeeper,
		ibcTransferKeeper: ibcTransferKeeper,
		bech32IbcKeeper:   bech32IbcKeeper,
		paramsKeeper:      paramsKeeper,

		app: app,
	}

	return k
}

func (k Keeper) UpdateSubKeeper(ctx sdk.Context, chainIdentifier string, params types.Params) error {
	subKeeper := k.loadSubKeeper(chainIdentifier)

	// if new chain, register it
	if !k.HasChain(ctx, chainIdentifier) {
		err := k.assertNoPrefix(ctx, chainIdentifier)
		if err != nil {
			return err
		}

		k.SetChain(ctx, chainIdentifier)

		// init genesis states for the new gravity keeper
		keeper.InitGenesis(ctx, subKeeper, *types.DefaultGenesisState())
	}

	subKeeper.SetParams(ctx, params)
	return nil
}

func (k Keeper) loadSubKeeper(chainIdentifier string) keeper.Keeper {
	subspaceKey := fmt.Sprintf("%s-%s", k.storeKey.Name(), chainIdentifier)
	paramSpace, ok := k.paramsKeeper.GetSubspace(subspaceKey)
	if !ok {
		paramSpace = k.paramsKeeper.Subspace(subspaceKey)
	}

	subKeeper := keeper.NewKeeper(
		k.storeKey,
		paramSpace,
		k.cdc,
		k.bankKeeper,
		k.StakingKeeper,
		k.SlashingKeeper,
		k.DistKeeper,
		k.accountKeeper,
		k.ibcTransferKeeper,
		k.bech32IbcKeeper,
	)
	subKeeper.ChainIdentifier = chainIdentifier

	return subKeeper
}

// assertNoPrefix will panic if there are two keys: k1 and k2 in keys, such that
// k1 is a prefix of k2
func (k Keeper) assertNoPrefix(ctx sdk.Context, chainIdentifier string) error {
	sorted := k.AllChains(ctx)
	sorted = append(sorted, chainIdentifier)
	sort.Strings(sorted)
	for i := 1; i < len(sorted); i++ {
		if strings.HasPrefix(sorted[i], sorted[i-1]) {
			return errors.New(fmt.Sprint("Potential key collision between KVStores:", sorted[i], " - ", sorted[i-1]))
		}
	}
	return nil
}

func (k Keeper) SubKeepers(ctx sdk.Context) []keeper.Keeper {
	chains := k.AllChains(ctx)
	subKeepers := make([]keeper.Keeper, 0, len(chains))
	for _, chain := range chains {
		subKeepers = append(subKeepers, k.loadSubKeeper(chain))
	}

	sort.Slice(subKeepers, func(i, j int) bool {
		return subKeepers[i].ChainIdentifier < subKeepers[j].ChainIdentifier
	})
	return subKeepers
}

func (k Keeper) SubKeeper(ctx sdk.Context, chainIdentifier string) (keeper.Keeper, error) {
	if !k.HasChain(ctx, chainIdentifier) {
		return keeper.Keeper{}, sdkerrors.Wrap(mgravitytypes.ErrChainNotFound, "")
	}
	return k.loadSubKeeper(chainIdentifier), nil
}

func (k Keeper) SubKeeperServers(ctx sdk.Context) []types.MsgServer {
	subKeepers := k.SubKeepers(ctx)
	servers := make([]types.MsgServer, 0, len(subKeepers))
	for _, subKeeper := range subKeepers {
		servers = append(servers, keeper.NewMsgServerImpl(subKeeper))
	}
	return servers
}

func (k Keeper) SubKeeperServer(ctx sdk.Context, chainIdentifier string) (types.MsgServer, error) {
	subKeeper, err := k.SubKeeper(ctx, chainIdentifier)
	if err != nil {
		return nil, err
	}
	return keeper.NewMsgServerImpl(subKeeper), nil
}

func (k Keeper) logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", mgravitytypes.ModuleName))
}
