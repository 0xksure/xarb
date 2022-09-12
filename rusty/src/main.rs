use ethers::{signers::LocalWallet, utils::Anvil, *};
use serde::{Deserialize, Serialize};
use std::{io::Error, string};
const API_URL: &'static str = "https://li.quest/v1";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ToolDetails {
    key: String,
    name: String,
    #[serde(rename = "logoURI")]
    logo_uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TokenDetails {
    address: String,
    symbol: String,
    decimals: u8,
    chain_id: u16,
    name: String,
    coin_key: String,
    #[serde(rename = "priceUSD")]
    price_usd: String,
    #[serde(rename = "logoURI")]
    logo_uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuoteAction {
    from_chain_id: u32,
    to_chain_id: u32,
    from_token: TokenDetails,
    to_token: TokenDetails,
    from_amount: String,
    slippage: f32,
    from_address: String,
    to_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LifiQuote {
    id: String,
    #[serde(rename = "type")]
    type_field: String,
    tool: String,
    tool_details: ToolDetails,
    action: QuoteAction,
}

async fn getQuote(
    from_chain: &str,
    to_chain: &str,
    from_token: &str,
    to_token: &str,
    from_amount: &str,
    from_address: &str,
) -> Result<LifiQuote, Error> {
    let client = reqwest::Client::new();
    let params = [
        ("fromChain", from_chain),
        ("toChain", to_chain),
        ("fromToken", from_token),
        ("toToken", to_token),
        ("fromAmount", from_amount),
        ("fromAddress", from_address),
    ];
    let url = API_URL.to_owned() + "/quote";
    let res = client.post(url).form(&params).send().await.unwrap();

    let json = res.json::<LifiQuote>().await.unwrap();

    Ok(json)
}

#[tokio::main]
async fn main() {
    let from_chain = "DAI";
    let from_token = "USDC";
    let to_chain = "POL";
    let to_token = "USDC";
    let from_amount = "1000000";
    let from_address = "0xbc2e384c8A15168fE5dF409CF2A260Cc4f2c398a";

    getQuote(
        from_chain,
        to_chain,
        from_token,
        to_token,
        from_amount,
        from_address,
    )
    .await
    .unwrap();
}
