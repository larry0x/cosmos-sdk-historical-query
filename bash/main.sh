#!/bin/bash

rpcUrl="https://celestia-rpc.mesa.newmetric.xyz:443"
queryPath="/cosmos.staking.v1beta1.Query/DelegatorDelegations"
delegatorAddr="celestia15rpm3yhl76ps7s74nu5pg06atpz70slal4kdk2"
height=123
limit=10

delegations="[]"
offset=0

while true; do
  echo "performing query... offset=$offset"
  new=$(celestia-appd q staking delegations $delegatorAddr --node $rpcUrl --height $height --limit $limit --offset $offset --output json | jq '.delegation_responses')
  if [ "$(echo $new | jq 'length')" -gt 0 ]; then
    delegations=$(echo -e $delegations"\n"$new | jq -s 'add')
    offset=$(echo "$offset + $limit" | bc)
  else
    break
  fi
done

echo $delegations | jq
