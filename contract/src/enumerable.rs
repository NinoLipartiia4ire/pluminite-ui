use crate::*;

#[near_bindgen]
impl NonFungibleTokenEnumeration for Contract {
    fn nft_total_supply(&self) -> U128 {
        U128(self.token_metadata_by_id.len() as u128)
    }

    fn nft_tokens(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        let keys = self.token_metadata_by_id.keys_as_vector();
        let start = u128::from(from_index.unwrap_or(U128(0)));
        keys.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }

    fn nft_supply_for_owner(
        &self,
        account_id: AccountId,
    ) -> U128 {
        let tokens_owner = self.tokens_per_owner.get(&account_id);
        if let Some(tokens_owner) = tokens_owner {
            U128(tokens_owner.len() as u128)
        } else {
            U128(0)
        }
    }

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        let tokens_owner = self.tokens_per_owner.get(&account_id);
        let tokens = if let Some(tokens_owner) = tokens_owner {
            tokens_owner
        } else {
            return vec![];
        };
        let keys = tokens.as_vector();
        let start = u128::from(from_index.unwrap_or(U128(0)));
        keys.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }
}

#[near_bindgen]
impl Contract {
    pub fn nft_tokens_from_end(
        &self,
        from_index: U64,
        limit: U64,
    ) -> Vec<JsonToken> {
        let mut tmp = vec![];
        let keys = self.token_metadata_by_id.keys_as_vector();
        let total_keys = keys.len();
        let from_index_prepared = u64::from(from_index);
        let mut limit_prepared = u64::from(limit);

        assert!(total_keys > from_index_prepared, "Illegal from_index");

        if total_keys - from_index_prepared < limit_prepared{
            limit_prepared = total_keys - from_index_prepared;
        }

        let start = total_keys - from_index_prepared - limit_prepared;
        let end = start + limit_prepared;
        
        for i in (start..end).rev() {
            tmp.push(self.nft_token(keys.get(i).unwrap()).unwrap());
        }
        tmp
    }

    pub fn nft_tokens_batch(
        &self,
        token_ids: Vec<String>,
    ) -> Vec<JsonToken> {
        let mut tmp = vec![];
        for i in 0..token_ids.len() {
            tmp.push(self.nft_token(token_ids[i].clone()).unwrap());
        }
        tmp
    }
    
    pub fn nft_supply_for_type(
        &self,
        token_type: &String,
    ) -> U64 {
        let tokens_per_type = self.tokens_per_type.get(&token_type);
        if let Some(tokens_per_type) = tokens_per_type {
            U64(tokens_per_type.len())
        } else {
            U64(0)
        }
    }

    pub fn nft_tokens_for_type(
        &self,
        token_type: String,
        from_index: U64,
        limit: U64,
    ) -> Vec<JsonToken> {
        let mut tmp = vec![];
        let tokens_per_type = self.tokens_per_type.get(&token_type);
        let tokens = if let Some(tokens_per_type) = tokens_per_type {
            tokens_per_type
        } else {
            return vec![];
        };
        let keys = tokens.as_vector();
        let start = u64::from(from_index);
        let end = min(start + u64::from(limit), keys.len());
        for i in start..end {
            tmp.push(self.nft_token(keys.get(i).unwrap()).unwrap());
        }
        tmp
    }

    pub fn nft_supply_for_creator(
        &self,
        account_id: AccountId,
    ) -> U64 {
        let tokens_creator = self.tokens_per_creator.get(&account_id);
        if let Some(tokens_creator) = tokens_creator {
            U64(tokens_creator.len())
        } else {
            U64(0)
        }
    }

    pub fn nft_tokens_for_creator(
        &self,
        account_id: AccountId,
        from_index: U64,
        limit: u16,
    ) -> Vec<JsonToken> {
        let mut tmp = vec![];
        let tokens_creator = self.tokens_per_creator.get(&account_id);
        let tokens = if let Some(tokens_creator) = tokens_creator {
            tokens_creator
        } else {
            return vec![];
        };
        let keys = tokens.as_vector();
        let start = u64::from(from_index);
        let end = min(start + u64::from(limit), keys.len());
        for i in start..end {
            tmp.push(self.nft_token(keys.get(i).unwrap()).unwrap());
        }
        tmp
    }
}
