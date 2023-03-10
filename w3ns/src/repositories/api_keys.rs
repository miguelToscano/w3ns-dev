use crate::domain::api_keys::types::ApiKey;
use ic_kit::candid::Principal;
use std::collections::HashMap;

#[derive(Default)]
pub struct ApiKeys(HashMap<Principal, ApiKey>);

impl ApiKeys {
    pub fn archive(&mut self) -> Vec<(Principal, ApiKey)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, ApiKey)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn create(&mut self, api_key: &ApiKey) -> Result<(), ()> {
        self.0.insert(api_key.clone().owner, api_key.clone());
        Ok(())
    }

    pub fn delete(&mut self, owner: &Principal) -> Result<(), ()> {
        self.0.remove(owner);
        Ok(())
    }

    pub fn get(&self, owner: &Principal) -> Option<ApiKey> {
        self.0.get(owner).cloned()
    }

    pub fn get_all(&self) -> Vec<ApiKey> {
        self.0.clone().into_values().collect()
    }
}
