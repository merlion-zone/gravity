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

	AttestationHandler interface {
		Handle(sdk.Context, types.Attestation, types.EthereumClaim) error
	}

	// Mapping chainIdentifier -> keeper.Keeper
	keepers map[string]keeper.Keeper
	app     *baseapp.BaseApp
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

		AttestationHandler: nil,

		keepers: map[string]keeper.Keeper{},
		app:     app,
	}
	attestationHandler := AttestationHandler{}
	k.AttestationHandler = attestationHandler

	return k
}

func (k Keeper) LoadSubKeepers(ctx sdk.Context) {
	for _, chain := range k.AllChains(ctx) {
		k.loadSubKeeper(ctx, chain)
	}
}

func (k Keeper) UpdateSubKeeper(ctx sdk.Context, chainIdentifier string, params types.Params) error {
	// if new chain, register it
	subKeeper, ok := k.keepers[chainIdentifier]
	if !ok {
		err := k.assertNoPrefix(ctx, chainIdentifier)
		if err != nil {
			return err
		}

		k.SetChain(ctx, chainIdentifier)
		subKeeper = k.loadSubKeeper(ctx, chainIdentifier)
	}

	subKeeper.SetParams(ctx, params)
	return nil
}

func (k Keeper) loadSubKeeper(ctx sdk.Context, chainIdentifier string) keeper.Keeper {
	key := fmt.Sprintf("%s-%s", k.storeKey.Name(), chainIdentifier)

	storeKey := sdk.NewKVStoreKey(key)
	k.app.MountStores(storeKey)

	k.paramsKeeper.Subspace(key)
	paramSpace, _ := k.paramsKeeper.GetSubspace(key)

	// set KeyTable if it has not already been set
	if !paramSpace.HasKeyTable() {
		paramSpace = paramSpace.WithKeyTable(types.ParamKeyTable())
	}

	subKeeper := keeper.NewKeeper(
		storeKey,
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
	k.keepers[chainIdentifier] = subKeeper
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

type SubKeeper struct {
	ChainIdentifier string
	Keeper          keeper.Keeper
}

func (k Keeper) SubKeepers() []SubKeeper {
	subKeepers := make([]SubKeeper, 0, len(k.keepers))
	for c, k := range k.keepers {
		subKeepers = append(subKeepers, SubKeeper{
			ChainIdentifier: c,
			Keeper:          k,
		})
	}
	sort.Slice(subKeepers, func(i, j int) bool {
		return subKeepers[i].ChainIdentifier < subKeepers[j].ChainIdentifier
	})
	return subKeepers
}

func (k Keeper) SubKeeper(chainIdentifier string) (keeper.Keeper, error) {
	subKeeper, ok := k.keepers[chainIdentifier]
	if !ok {
		return keeper.Keeper{}, sdkerrors.Wrap(mgravitytypes.ErrChainNotFound, "")
	}
	return subKeeper, nil
}

func (k Keeper) SubKeeperServers() []types.MsgServer {
	servers := make([]types.MsgServer, 0, len(k.keepers))
	for _, subKeeper := range k.SubKeepers() {
		servers = append(servers, keeper.NewMsgServerImpl(subKeeper.Keeper))
	}
	return servers
}

func (k Keeper) SubKeeperServer(chainIdentifier string) (types.MsgServer, error) {
	subKeeper, err := k.SubKeeper(chainIdentifier)
	if err != nil {
		return nil, err
	}
	return keeper.NewMsgServerImpl(subKeeper), nil
}
