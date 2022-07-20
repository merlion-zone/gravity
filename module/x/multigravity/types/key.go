package types

import "github.com/Gravity-Bridge/Gravity-Bridge/module/x/gravity/types"

const (
	// ModuleName is the name of the module
	ModuleName = "mgravity"

	// StoreKey to be used when creating the KVStore
	StoreKey = ModuleName

	// RouterKey is the module name router key
	RouterKey = ModuleName

	// QuerierRoute to be used for querierer msgs
	QuerierRoute = ModuleName
)

var (
	ChainIdentifierKey = types.HashString("ChainIdentifierKey")
	ChainParamsKey     = types.HashString("ChainParamsKey")
)

func GetChainIdentifierKey(chainIdentifier string) []byte {
	return types.AppendBytes(ChainIdentifierKey, []byte(chainIdentifier))
}

func GetChainParamsKey(chainIdentifier string) []byte {
	return types.AppendBytes(ChainParamsKey, []byte(chainIdentifier))
}
