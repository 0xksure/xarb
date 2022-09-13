use std::fmt::Error;

use serde::{Deserialize, Serialize};
pub const API_URL: &'static str = "https://li.quest/v1";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolDetails {
    key: String,
    name: String,
    #[serde(rename = "logoURI")]
    logo_uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDetails {
    pub address: String,
    pub symbol: String,
    pub decimals: u8,
    pub chain_id: u16,
    pub name: String,
    pub coin_key: String,
    #[serde(rename = "priceUSD")]
    pub price_usd: String,
    #[serde(rename = "logoURI", default)]
    pub logo_uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenSubList {
    #[serde(rename = "1")]
    pub one: Vec<TokenDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenList {
    #[serde(rename = "tokens")]
    pub tokens: TokenSubList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteAction {
    pub from_chain_id: u32,
    pub to_chain_id: u32,
    pub from_token: TokenDetails,
    pub to_token: TokenDetails,
    pub from_amount: String,
    pub slippage: f32,
    pub from_address: String,
    pub to_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    pub name: String,
    pub percentage: String,
    pub token: TokenDetails,
    pub amount: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gas {
    #[serde(rename = "type")]
    pub gas_type: String,
    pub price: String,
    pub estimate: String,
    pub limit: String,
    pub amount: String,
    #[serde(rename = "amountUSD")]
    pub amount_usd: String,
    pub token: TokenDetails,
}

pub struct NativeCurrency {
    name: String,
    symbol: String,
    decimals: u8,
}
pub struct Metamask {
    chainId: String,
    blockExplorerUrls: Vec<String>,
    chainName: String,
    nativeCurrency: NativeCurrency,
    rpcUrls: Vec<String>,
}
pub struct Chain {
    key: String,
    chainType: String,
    name: String,
    coin: String,
    id: u16,
    mainnet: bool,
    logoURI: String,
    tokenlistUrl: String,
    multicallAddress: String,
    metamask: Metamask,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteData {
    pub amount_out: String,
    pub rate: String,
    pub price_impact: String,
    pub required_liquidity: String,
    pub lp_fees: String,
    pub adjusted_bonder_fee: String,
    pub adjusted_destination_tx_fee: String,
    pub total_fee: String,
    pub estimated_received: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteEstimate {
    pub from_amount: String,
    pub to_amount: String,
    pub to_amount_min: String,
    pub approval_address: String,
    pub execution_duration: f32,
    pub fee_costs: Vec<Fee>,
    pub gas_costs: Vec<Gas>,
    pub data: QuoteData,
    #[serde(rename = "fromAmountUSD")]
    pub from_amount_usd: String,
    #[serde(rename = "toAmountUSD")]
    pub to_amount_usd: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    pub data: String,
    pub to: String,
    pub value: String,
    pub from: String,
    pub chain_id: u16,
    pub gas_limit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiQuote {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub tool: String,
    pub tool_details: ToolDetails,
    pub action: QuoteAction,
    pub estimate: QuoteEstimate,
    pub transaction_request: TransactionRequest,
}
