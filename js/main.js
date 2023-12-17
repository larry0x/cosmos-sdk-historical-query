const { Tendermint34Client } = require("@cosmjs/tendermint-rpc");
const { QueryClient } = require("@cosmjs/stargate");
const {
  QueryDelegatorDelegationsRequest,
  QueryDelegatorDelegationsResponse,
} = require("cosmjs-types/cosmos/staking/v1beta1/query");

const RPC_URL        = "https://celestia-rpc.mesa.newmetric.xyz";
const QUERY_PATH     = "/cosmos.staking.v1beta1.Query/DelegatorDelegations";
const DELEGATOR_ADDR = "celestia15rpm3yhl76ps7s74nu5pg06atpz70slal4kdk2";
const HEIGHT         = 123;
const LIMIT          = BigInt(10);

(async function () {
  const tmClient = await Tendermint34Client.connect(RPC_URL);
  const queryClient = new QueryClient(tmClient);

  let delegations = [];
  let next = [];

  while (true) {
    console.log(`performing query... next="${Buffer.from(next).toString("hex")}"`)

    const query = QueryDelegatorDelegationsRequest.fromPartial({
      delegatorAddr: DELEGATOR_ADDR,
      pagination: {
        key:   new Uint8Array(next),
        limit: LIMIT,
      }
    });

    const abciRes = await queryClient.queryAbci(
      QUERY_PATH,
      QueryDelegatorDelegationsRequest.encode(query).finish(),
      HEIGHT,
    );

    const response = QueryDelegatorDelegationsResponse.decode(abciRes.value);

    delegations = delegations.concat(response.delegationResponses);
    next = response.pagination.nextKey;

    if (next.length === 0) {
      break;
    }
  }

  console.log(JSON.stringify(delegations, null, 2));
})();
