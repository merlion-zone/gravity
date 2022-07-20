package types

import (
	"fmt"
	"strings"

	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
)

const (
	ProposalTypeUpdateChainParams = "UpdateChainParams"
	ProposalTypeUnhaltBridge      = "UnhaltBridge"
)

func (p *UpdateChainParamsProposal) GetTitle() string { return p.Title }

func (p *UpdateChainParamsProposal) GetDescription() string { return p.Description }

func (p *UpdateChainParamsProposal) ProposalRoute() string { return RouterKey }

func (p *UpdateChainParamsProposal) ProposalType() string {
	return ProposalTypeUpdateChainParams
}

func (p *UpdateChainParamsProposal) ValidateBasic() error {
	err := govtypes.ValidateAbstract(p)
	if err != nil {
		return err
	}
	return nil
}

func (p UpdateChainParamsProposal) String() string {
	var b strings.Builder
	b.WriteString(fmt.Sprintf(`Update Chain Params Proposal:
  Title:             %s
  Description:       %s
  ChainIdentifier:   %s
  Params:            %s
`, p.Title, p.Description, p.ChainIdentifier, p.Params.String()))
	return b.String()
}

func (p *UnhaltBridgeProposal) GetTitle() string { return p.Title }

func (p *UnhaltBridgeProposal) GetDescription() string { return p.Description }

func (p *UnhaltBridgeProposal) ProposalRoute() string { return RouterKey }

func (p *UnhaltBridgeProposal) ProposalType() string {
	return ProposalTypeUnhaltBridge
}

func (p *UnhaltBridgeProposal) ValidateBasic() error {
	err := govtypes.ValidateAbstract(p)
	if err != nil {
		return err
	}
	return nil
}

func (p UnhaltBridgeProposal) String() string {
	var b strings.Builder
	b.WriteString(fmt.Sprintf(`Unhalt Bridge Proposal:
  Title:          %s
  Description:    %s
  target_nonce:   %d
`, p.Title, p.Description, p.TargetNonce))
	return b.String()
}
