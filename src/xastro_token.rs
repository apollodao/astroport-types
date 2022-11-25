use cw20::{Cw20Coin, Logo, MinterResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMarketingInfo {
    pub description: Option<String>,
    pub project: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct InstantiateMsg {
    pub name: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub symbol: String,
    pub marketing: Option<InstantiateMarketingInfo>,
    pub mint: Option<MinterResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    BalanceAt {
        address: String,
        block: u64,
    },
    TotalSupplyAt {
        block: u64,
    },
    Balance {
        address: String,
    },
    TokenInfo {},
    Minter {},
    Allowance {
        owner: String,
        spender: String,
    },
    MarketingInfo {},
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    DownloadLogo {},
}
