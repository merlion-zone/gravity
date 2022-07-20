package cli

import (
	"strconv"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/spf13/cobra"

	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"

	mgravitytypes "github.com/Gravity-Bridge/Gravity-Bridge/module/x/multigravity/types"
)

func GetQueryCmd() *cobra.Command {
	// nolint: exhaustruct
	gravityQueryCmd := &cobra.Command{
		Use:                        mgravitytypes.ModuleName,
		Short:                      "Querying commands for the gravity module",
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
		RunE:                       client.ValidateCmd,
	}
	gravityQueryCmd.AddCommand([]*cobra.Command{
		CmdGetCurrentValset(),
		CmdGetValsetRequest(),
		CmdGetValsetConfirm(),
		CmdGetPendingValsetRequest(),
		CmdGetPendingOutgoingTXBatchRequest(),
		CmdGetPendingSendToEth(),
		GetCmdPendingIbcAutoForwards(),
		GetCmdQueryParams(),
	}...)

	return gravityQueryCmd
}

func CmdGetCurrentValset() *cobra.Command {
	// nolint: exhaustruct
	cmd := &cobra.Command{
		Use:   "current-valset",
		Short: "Query current valset",
		Args:  cobra.NoArgs,
		RunE: func(cmd *cobra.Command, _ []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)
			queryClient := mgravitytypes.NewQueryClient(clientCtx)

			chainIdentifier, err := cmd.Flags().GetString(FlagEvmChainIdentifier)
			if err != nil {
				return err
			}

			req := &mgravitytypes.QueryCurrentValsetRequest{chainIdentifier}

			res, err := queryClient.CurrentValset(cmd.Context(), req)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}
	addQueryFlagsToCmd(cmd)
	return cmd
}

func CmdGetValsetRequest() *cobra.Command {
	// nolint: exhaustruct
	cmd := &cobra.Command{
		Use:   "valset-request [nonce]",
		Short: "Get requested valset with a particular nonce",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)
			queryClient := mgravitytypes.NewQueryClient(clientCtx)

			nonce, err := strconv.ParseUint(args[0], 10, 64)
			if err != nil {
				return err
			}

			chainIdentifier, err := cmd.Flags().GetString(FlagEvmChainIdentifier)
			if err != nil {
				return err
			}

			req := &mgravitytypes.QueryValsetRequestRequest{
				Nonce:           nonce,
				ChainIdentifier: chainIdentifier,
			}

			res, err := queryClient.ValsetRequest(cmd.Context(), req)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}
	addQueryFlagsToCmd(cmd)
	return cmd
}

func CmdGetValsetConfirm() *cobra.Command {
	// nolint: exhaustruct
	cmd := &cobra.Command{
		Use:   "valset-confirm [nonce] [bech32 validator address]",
		Short: "Get valset confirmation with a particular nonce from a particular validator",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)
			queryClient := mgravitytypes.NewQueryClient(clientCtx)

			nonce, err := strconv.ParseUint(args[0], 10, 64)
			if err != nil {
				return err
			}

			chainIdentifier, err := cmd.Flags().GetString(FlagEvmChainIdentifier)
			if err != nil {
				return err
			}

			req := &mgravitytypes.QueryValsetConfirmRequest{
				Nonce:           nonce,
				Address:         args[1],
				ChainIdentifier: chainIdentifier,
			}

			res, err := queryClient.ValsetConfirm(cmd.Context(), req)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}
	addQueryFlagsToCmd(cmd)
	return cmd
}

func CmdGetPendingValsetRequest() *cobra.Command {
	// nolint: exhaustruct
	cmd := &cobra.Command{
		Use:   "pending-valset-request [bech32 orchestrator address]",
		Short: "Get the latest valset request which has not been signed by a particular orchestrator",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)
			queryClient := mgravitytypes.NewQueryClient(clientCtx)

			chainIdentifier, err := cmd.Flags().GetString(FlagEvmChainIdentifier)
			if err != nil {
				return err
			}

			req := &mgravitytypes.QueryLastPendingValsetRequestByAddrRequest{
				Address:         args[0],
				ChainIdentifier: chainIdentifier,
			}

			res, err := queryClient.LastPendingValsetRequestByAddr(cmd.Context(), req)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}
	addQueryFlagsToCmd(cmd)
	return cmd
}

func CmdGetPendingOutgoingTXBatchRequest() *cobra.Command {
	// nolint: exhaustruct
	cmd := &cobra.Command{
		Use:   "pending-batch-request [bech32 orchestrator address]",
		Short: "Get the latest outgoing TX batch request which has not been signed by a particular orchestrator",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)
			queryClient := mgravitytypes.NewQueryClient(clientCtx)

			chainIdentifier, err := cmd.Flags().GetString(FlagEvmChainIdentifier)
			if err != nil {
				return err
			}

			req := &mgravitytypes.QueryLastPendingBatchRequestByAddrRequest{
				Address:         args[0],
				ChainIdentifier: chainIdentifier,
			}

			res, err := queryClient.LastPendingBatchRequestByAddr(cmd.Context(), req)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}
	addQueryFlagsToCmd(cmd)
	return cmd
}

func CmdGetPendingSendToEth() *cobra.Command {
	// nolint: exhaustruct
	cmd := &cobra.Command{
		Use:   "pending-send-to-eth [address]",
		Short: "Query transactions waiting to go to Ethereum",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)
			queryClient := mgravitytypes.NewQueryClient(clientCtx)

			chainIdentifier, err := cmd.Flags().GetString(FlagEvmChainIdentifier)
			if err != nil {
				return err
			}

			req := &mgravitytypes.QueryPendingSendToEth{
				SenderAddress:   args[0],
				ChainIdentifier: chainIdentifier,
			}

			res, err := queryClient.GetPendingSendToEth(cmd.Context(), req)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}
	addQueryFlagsToCmd(cmd)
	return cmd
}

func GetCmdPendingIbcAutoForwards() *cobra.Command {
	// nolint: exhaustruct
	cmd := &cobra.Command{
		Use:   "pending-ibc-auto-forwards [optional limit]",
		Short: "Query SendToCosmos transactions waiting to be forwarded over IBC",
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)
			queryClient := mgravitytypes.NewQueryClient(clientCtx)

			var limit uint64 = 0
			if args[0] != "" {
				var err error
				limit, err = strconv.ParseUint(args[0], 10, 0)
				if err != nil {
					return sdkerrors.Wrapf(err, "Unable to parse limit from %v", args[0])
				}
			}

			chainIdentifier, err := cmd.Flags().GetString(FlagEvmChainIdentifier)
			if err != nil {
				return err
			}

			req := &mgravitytypes.QueryPendingIbcAutoForwards{Limit: limit, ChainIdentifier: chainIdentifier}
			res, err := queryClient.GetPendingIbcAutoForwards(cmd.Context(), req)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}
	addQueryFlagsToCmd(cmd)
	return cmd
}

func GetCmdQueryParams() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "params",
		Args:  cobra.NoArgs,
		Short: "Query gravity params",
		RunE: func(cmd *cobra.Command, _ []string) error {
			clientCtx, err := client.GetClientQueryContext(cmd)
			if err != nil {
				return err
			}
			queryClient := mgravitytypes.NewQueryClient(clientCtx)

			chainIdentifier, err := cmd.Flags().GetString(FlagEvmChainIdentifier)
			if err != nil {
				return err
			}

			res, err := queryClient.Params(cmd.Context(), &mgravitytypes.QueryParamsRequest{chainIdentifier})
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(&res.Params)
		},
	}

	addQueryFlagsToCmd(cmd)
	return cmd
}

func addQueryFlagsToCmd(cmd *cobra.Command) {
	cmd.Flags().StringP(FlagEvmChainIdentifier, "e", "", "EVM chain identifier")
	flags.AddQueryFlagsToCmd(cmd)

	cmd.MarkFlagRequired(FlagEvmChainIdentifier)
}
