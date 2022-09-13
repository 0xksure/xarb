use std::fmt::Error;

use reqwest::Client;

use crate::state::*;

pub struct Lifi {
    uri: String,
    client: Client,
}

impl Lifi {
    /// Constructor
    pub fn new(uri: &str) -> Self {
        Lifi {
            uri: String::from(uri),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_quote(
        &self,
        from_chain: &str,
        to_chain: &str,
        from_token: &str,
        to_token: &str,
        from_amount: &str,
        from_address: &str,
    ) -> Result<LifiQuote, Error> {
        let params = [
            ("fromChain", from_chain),
            ("toChain", to_chain),
            ("fromToken", from_token),
            ("toToken", to_token),
            ("fromAmount", from_amount),
            ("fromAddress", from_address),
        ];
        let url = self.uri.clone() + "/quote";
        let res = self.client.post(url).form(&params).send().await.unwrap();
        let quote = res.json::<LifiQuote>().await.unwrap();

        Ok(quote)
    }

    pub async fn get_token(
        &self,
        token: &str,
        chain: &str,
    ) -> Result<TokenDetails, reqwest::Error> {
        let params = [("chain", chain), ("token", token)];
        let url = self.uri.clone() + "/token";
        let res = self
            .client
            .get(url)
            .header("Accept", "application/json")
            .query(&params)
            .send()
            .await?;
        let token_details = res.json::<TokenDetails>().await.unwrap();
        Ok(token_details)
    }

    pub async fn get_tokens(&self) -> Result<TokenList, reqwest::Error> {
        let url = self.uri.clone() + "/tokens";
        let res = self
            .client
            .get(url)
            .header("Accept", "application/json")
            .send()
            .await
            .unwrap()
            .error_for_status()?;
        let tokens = res.json::<TokenList>().await.unwrap();
        Ok(tokens)
    }

    /// TODO: implement
    pub async fn get_chains(&self) {}
}

#[cfg(test)]
pub mod test_lifi {
    use super::Lifi;

    #[tokio::test]
    async fn test_get_quote() {
        let from_chain = "POL";
        let from_token = "USDC";
        let to_chain = "DAI";
        let to_token = "USDC";
        let from_amount = "1000000000";
        let from_address = "0xbc2e384c8A15168fE5dF409CF2A260Cc4f2c398a";

        let lifi = Lifi::new("https://li.quest/v1");
        let quote = lifi
            .get_quote(
                from_chain,
                to_chain,
                from_token,
                to_token,
                from_amount,
                from_address,
            )
            .await
            .unwrap();
        assert!(quote.action.from_address == from_address);
    }

    #[tokio::test]
    async fn test_get_token() {
        let lifi = Lifi::new("https://li.quest/v1");
        let token_details = lifi.get_token("DAI", "POL").await.unwrap();
        println!("token details address {}", token_details.chain_id);
    }

    #[tokio::test]
    async fn test_get_tokens() {
        let lifi = Lifi::new("https://li.quest/v1");
        let tokens = lifi.get_tokens().await.unwrap();
        assert!(tokens.tokens.one.len() > 0);
    }
}
