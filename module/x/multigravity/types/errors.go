package types

import sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"

var (
	ErrChainNotFound = sdkerrors.Register(ModuleName, 101, "EVM chain not found")
	ErrChainExisting = sdkerrors.Register(ModuleName, 102, "EVM chain existing")
)
