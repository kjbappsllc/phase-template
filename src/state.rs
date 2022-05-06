use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, CanonicalAddr, StdResult, Storage, Uint128};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {}

pub const CONFIG: Item<Config> = Item::new("config");

pub fn load_config(storage: &dyn Storage) -> StdResult<Config> { CONFIG.load(storage) }

pub fn save_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
  CONFIG.save(storage, config)
}
