use {
    cosmos_sdk_proto::cosmos::{
        base::query::v1beta1::PageRequest,
        staking::v1beta1::{QueryDelegatorDelegationsRequest, QueryDelegatorDelegationsResponse},
    },
    prost::Message,
    serde::Serialize,
    tendermint::abci::Code,
    tendermint_rpc::{Client, HttpClient},
};

const RPC_ENDPOINT:   &str = "https://celestia-rpc.mesa.newmetric.xyz";
const DELEGATOR:      &str = "celestia15rpm3yhl76ps7s74nu5pg06atpz70slal4kdk2";
const QUERY_TYPE_URL: &str = "/cosmos.staking.v1beta1.Query/DelegatorDelegations";
const HEIGHT:         u32  = 123;
const LIMIT:          u64  = 10;

#[derive(Debug, Serialize)]
struct Delegation {
    validator: String,
    amount:    u128,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let abci_client = HttpClient::new(RPC_ENDPOINT)?;

    let mut delegations = vec![];
    let mut next = vec![];

    loop {
        println!("performing query... next=\"{}\"", hex::encode(&next));

        let query = QueryDelegatorDelegationsRequest {
            delegator_addr: DELEGATOR.into(),
            pagination: Some(PageRequest {
                key:   next,
                limit: LIMIT,
                ..Default::default()
            }),
        };

        let abci_res = abci_client
            .abci_query(
                Some(QUERY_TYPE_URL.into()),
                query.encode_to_vec(),
                Some(HEIGHT.into()),
                false,
            )
            .await?;

        if abci_res.code != Code::Ok {
            panic!(
                "query failed! codespace: {}, code: {:?}, log: {}",
                abci_res.codespace,
                abci_res.code,
                abci_res.log,
            );
        }

        let response = QueryDelegatorDelegationsResponse::decode(abci_res.value.as_slice())?;

        delegations.extend(
            response
                .delegation_responses
                .into_iter()
                .map(|res| Delegation {
                    validator: res.delegation.unwrap().validator_address,
                    amount:    res.balance.unwrap().amount.parse().unwrap(),
                }),
        );

        next = response.pagination.unwrap().next_key;
        if next.is_empty() {
            break;
        }
    }

    println!("{}", serde_json::to_string_pretty(&delegations)?);

    Ok(())
}
