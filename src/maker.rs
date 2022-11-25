use crate::asset::{Asset, AssetInfo};
use crate::factory::UpdateAddr;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub astro_token_contract: String,
    pub factory_contract: String,
    pub staking_contract: String,
    pub governance_contract: Option<String>,
    pub governance_percent: Option<Uint64>,
    pub max_spread: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Collect {
        assets: Vec<AssetWithLimit>,
    },
    UpdateConfig {
        factory_contract: Option<String>,
        staking_contract: Option<String>,
        governance_contract: Option<UpdateAddr>,
        governance_percent: Option<Uint64>,
        max_spread: Option<Decimal>,
    },
    UpdateBridges {
        add: Option<Vec<(AssetInfo, AssetInfo)>>,
        remove: Option<Vec<AssetInfo>>,
    },
    SwapBridgeAssets {
        assets: Vec<AssetInfo>,
        depth: u64,
    },
    ProposeNewOwner {
        owner: String,
        expires_in: u64,
    },
    DropOwnershipProposal {},
    ClaimOwnership {},
    EnableRewards {
        blocks: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Balances { assets: Vec<AssetInfo> },
    Bridges {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: Addr,
    pub astro_token_contract: Addr,
    pub factory_contract: Addr,
    pub staking_contract: Addr,
    pub governance_contract: Option<Addr>,
    pub governance_percent: Uint64,
    pub max_spread: Decimal,
    pub remainder_reward: Uint128,
    pub pre_upgrade_astro_amount: Uint128,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalancesResponse {
    pub balances: Vec<Asset>,
}

#[cw_serde]
pub struct AssetWithLimit {
    pub info: AssetInfo,
    pub limit: Option<Uint128>,
}
