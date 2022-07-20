package types

// ValidateBasic validates genesis state
func (s GenesisState) ValidateBasic() error {
	return nil
}

// DefaultGenesisState returns empty genesis state
func DefaultGenesisState() *GenesisState {
	return &GenesisState{}
}
