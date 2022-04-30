///This contract offers the functionality of minting non-fungible-tokens (NFT)
///Users of this contracr can mint NFTs to their own accounts.
///The minted NFTs will show up in the account that's passed to the contract as receiver_id.
//Required types for the contract to work.
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedSet};
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //tokenizer handles minting tokens
    tokenizer: NonFungibleToken, //NonFungibleToken type contains all logic for managing tokens

    //metadata is the metadata of the contract (name, description, icon, etc)
    //that appears in user's wallet when we use this contract for minting
    metadata: LazyOption<NFTContractMetadata>, //LazyOption prevents a contract from deserializing the metadata until it's needed
}

//Contract icon in svg-data format. It appears in user's wallet when we use this contract for minting
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'><circle r='50' cx='50' cy='50' fill='orange'/><circle r='41' cx='47' cy='50' fill='orange'/><circle r='33' cx='48' cy='53' fill='orange'/><circle r='25' cx='49' cy='51' fill='yellowgreen'/><circle r='17' cx='52' cy='50' fill='lightseagreen'/><circle r='9' cx='55' cy='48' fill='orange'/></svg>";

#[derive(BorshSerialize, BorshStorageKey)]
//managing storage keys
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    TokensPerOwner { account_hash: Vec<u8> },
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` and metadata
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Ukraine Zoo".to_string(),
                symbol: "MHNT".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: Some("https://t.me/msauditech".to_string()),
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokenizer: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    //mint an NFT
    //passing a token id, receiver account id, and token metadata
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: ValidAccountId,
        token_metadata: Option<TokenMetadata>,
    ) -> String {
        let owner_id: AccountId = receiver_id.into();

        //this part of the code adjused from this repo: https://github.com/ligebit/spring_near_nft_minter/blob/master/contracts/src/lib.rs
        self.tokenizer.owner_by_id.insert(&token_id, &owner_id);
        self.tokenizer
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &token_metadata.as_ref().unwrap()));

        if let Some(tokens_per_owner) = &mut self.tokenizer.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(&owner_id).unwrap_or_else(|| {
                UnorderedSet::new(StorageKey::TokensPerOwner {
                    account_hash: env::sha256(owner_id.as_bytes()),
                })
            });
            token_ids.insert(&token_id);
            tokens_per_owner.insert(&owner_id, &token_ids);
        }

        "Done".to_string()
    }
}

//required implementation from the nft standard library for the minting process to work
near_contract_standards::impl_non_fungible_token_core!(Contract, tokenizer);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokenizer);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokenizer);

//returns the contract metadata
#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
