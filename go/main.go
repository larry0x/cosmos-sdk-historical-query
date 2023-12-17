package main

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"

	cmtrpcclient "github.com/cometbft/cometbft/rpc/client"
	cmthttpclient "github.com/cometbft/cometbft/rpc/client/http"
	"github.com/cosmos/cosmos-sdk/codec"
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdkquery "github.com/cosmos/cosmos-sdk/types/query"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
)

const (
	rpcURL               = "https://celestia-rpc.mesa.newmetric.xyz:443"
	queryPath            = "/cosmos.staking.v1beta1.Query/DelegatorDelegations"
	delegatorAddr        = "celestia15rpm3yhl76ps7s74nu5pg06atpz70slal4kdk2"
	height        int64  = 123
	limit         uint64 = 10
)

func main() {
	client, err := cmthttpclient.New(rpcURL, "/websocket")
	if err != nil {
		panic(err)
	}

	var (
		cdc         = codec.NewProtoCodec(codectypes.NewInterfaceRegistry())
		delegations = []stakingtypes.DelegationResponse{}
		next        = []byte{}
	)

	for {
		fmt.Printf("performing query... next=\"%s\"\n", hex.EncodeToString(next))

		query := stakingtypes.QueryDelegatorDelegationsRequest{
			DelegatorAddr: delegatorAddr,
			Pagination: &sdkquery.PageRequest{
				Key:   next,
				Limit: limit,
			},
		}

		queryBin, err := cdc.Marshal(&query)
		if err != nil {
			panic(err)
		}

		result, err := client.ABCIQueryWithOptions(
			context.Background(),
			queryPath,
			queryBin,
			cmtrpcclient.ABCIQueryOptions{Height: height},
		)
		if err != nil {
			panic(err)
		}

		if result.Response.Code != 0 {
			panic(fmt.Sprintf(
				"query failed! codespace: %s, code: %d, log: %s",
				result.Response.Codespace,
				result.Response.Code,
				result.Response.Log,
			))
		}

		response := stakingtypes.QueryDelegatorDelegationsResponse{}
		if err = cdc.Unmarshal(result.Response.Value, &response); err != nil {
			panic(err)
		}

		delegations = append(delegations, response.DelegationResponses...)
		next = response.Pagination.NextKey

		if len(next) == 0 {
			break
		}
	}

	delegationsStr, err := json.MarshalIndent(delegations, "", "  ")
	if err != nil {
		panic(err)
	}

	fmt.Println(string(delegationsStr))
}
