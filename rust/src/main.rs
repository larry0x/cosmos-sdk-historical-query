use {
    cosmos_sdk_proto::cosmos::{
        base::query::v1beta1::PageRequest,
        staking::v1beta1::{QueryDelegatorDelegationsRequest, QueryDelegatorDelegationsResponse},
    },
    prost::Message,
    tendermint::abci::Code,
    tendermint_rpc::{Client, HttpClient},
};

const RPC_URL:        &str = "https://celestia-rpc.mesa.newmetric.xyz";
const QUERY_PATH:     &str = "/cosmos.staking.v1beta1.Query/DelegatorDelegations";
const DELEGATOR_ADDR: &str = "celestia15rpm3yhl76ps7s74nu5pg06atpz70slal4kdk2";
const HEIGHT:         u32  = 123;
const LIMIT:          u64  = 10;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let abci_client = HttpClient::new(RPC_URL)?;

    let mut delegations = vec![];
    let mut next = vec![];

    loop {
        println!("performing query... next=\"{}\"", hex::encode(&next));

        let query = QueryDelegatorDelegationsRequest {
            delegator_addr: DELEGATOR_ADDR.into(),
            pagination: Some(PageRequest {
                key:   next,
                limit: LIMIT,
                ..Default::default()
            }),
        };

        let abci_res = abci_client
            .abci_query(
                Some(QUERY_PATH.into()),
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

        delegations.extend(response.delegation_responses);
        next = response.pagination.unwrap().next_key;

        if next.is_empty() {
            break;
        }
    }

    // DelegationResponse doesn't implement serde::Serialize, so we can't print
    // it out in pretty JSON as we can for the other two languages
    dbg!(delegations);

    Ok(())
}
