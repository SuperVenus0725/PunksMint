use cosmwasm_std::Uint128;
use cw_storage_plus::{Map,Item};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::msg::{JunoPunksMsg};

pub const CONFIG: Item<State> = Item::new("config_state");
pub const METADATA: Item<Vec<JunoPunksMsg>> = Item::new("metadata");

pub const USERINFO: Map<&str, Uint128> = Map::new("offerings");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub total_nft:Uint128,
    pub owner:String,
    pub max_nft:Uint128,
    pub count : Uint128,
    pub check_mint:Vec<bool>,
    pub nft_address:String,
    pub url :String,
    pub image_url:String
}