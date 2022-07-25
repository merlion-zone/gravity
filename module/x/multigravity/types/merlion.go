package types

import banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"

var GasCoinMetadata banktypes.Metadata

func SetGasCoinMetata(metadata banktypes.Metadata) {
	GasCoinMetadata = metadata
}
