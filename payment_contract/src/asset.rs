use soroban_sdk::{contracttype, map, Bytes, Env, Map, Vec};

use crate::storage_types::DataKey;

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
pub struct Asset {
    asset_id: Bytes,
    submission_date: u64,
    state: AssetState,
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
pub enum AssetState {
    InReview,
    Approved,
    Rejected,
    Paid,
}

impl Asset {
    pub fn new(asset_id: Bytes, submission_date: u64) -> Self {
        Asset {
            asset_id,
            submission_date,
            state: AssetState::InReview,
        }
    }
}

pub(crate) fn store_assets(env: &Env, asset_ids: Vec<Bytes>, submission_date: u64) {
    let mut assets: Map<Bytes, Asset> = map![env];
    for asset_id in asset_ids.iter() {
        let id = asset_id.unwrap();
        let asset = Asset::new(id.clone(), submission_date);
        assets.set(id, asset);
    }
    write_assets(env, &assets)
}

pub(crate) fn write_assets(env: &Env, assets: &Map<Bytes, Asset>) {
    let key: DataKey = DataKey::CreatorAssets;
    env.storage().set(&key, assets)
}
