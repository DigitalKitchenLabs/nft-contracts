#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

use cosmwasm_std::{from_binary, Addr, DepsMut, Empty};

use cw721::{ContractInfoResponse, Cw721Query, NftInfoResponse, OwnerOfResponse};
use cw721_base::msg::{
    CollectionInfo, CollectionInfoResponse, RoyaltyInfoResponse, UpdateCollectionInfoMsg,
};
use cw_ownable::OwnershipError;

use crate::error::ContractError;
use crate::msg::Metadata;
use crate::{
    Cw721Contract, Cw721TraitContract, ExecuteMsg, Extension, InstantiateMsg, MinterResponse,
    QueryMsg,
};

const MINTER: &str = "merlin";
const CONTRACT_NAME: &str = "Magic Power";
const SYMBOL: &str = "MGK";

fn setup_contract(deps: DepsMut<'_>) -> Cw721Contract<'static, Extension, Empty, Empty, Empty> {
    let contract = Cw721TraitContract::default();
    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: String::from(MINTER),
        collection_info: CollectionInfo {
            creator: "creator".into(),
            description: "description".into(),
            image: Some("https://example.com/image.png".into()),
            external_link: None,
            explicit_content: None,
            royalty_info: None,
        },
    };
    let info = mock_info("creator", &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();
    let contract = Cw721TraitContract::default();

    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: String::from(MINTER),
        collection_info: CollectionInfo {
            creator: "creator".into(),
            description: "description".into(),
            image: Some("https://example.com/image.png".into()),
            external_link: None,
            explicit_content: None,
            royalty_info: None,
        },
    };
    let info = mock_info("creator", &[]);

    // we can just call .unwrap() to assert this was a success
    let res = contract
        .instantiate(deps.as_mut(), mock_env(), info, msg)
        .unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let res = contract.minter(deps.as_ref()).unwrap();
    assert_eq!(Some(MINTER.to_string()), res.minter);
    let info = contract.contract_info(deps.as_ref()).unwrap();
    assert_eq!(
        info,
        ContractInfoResponse {
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
        }
    );

    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(0, count.count);

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(0, tokens.tokens.len());
}

#[test]
fn minting() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let token_id = "1".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("medusa"),
        token_uri: None,
        extension: Metadata {
            trait_type: String::from("hair"),
            trait_value: String::from("red"),
            trait_rarity: String::from("common"),
        },
    };

    // random cannot mint
    let random = mock_info("random", &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), random, mint_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));

    // minter can mint
    let allowed = mock_info(MINTER, &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), allowed, mint_msg)
        .unwrap();

    // ensure num tokens increases
    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(1, count.count);

    // unknown nft returns error
    let _ = contract
        .nft_info(deps.as_ref(), "unknown".to_string())
        .unwrap_err();

    // this nft info is correct
    let info = contract.nft_info(deps.as_ref(), token_id.clone()).unwrap();
    assert_eq!(
        info,
        NftInfoResponse::<Extension> {
            token_uri: None,
            extension: Metadata {
                trait_type: String::from("hair"),
                trait_value: String::from("red"),
                trait_rarity: String::from("common"),
            },
        }
    );

    // owner info is correct
    let owner = contract
        .owner_of(deps.as_ref(), mock_env(), token_id.clone(), true)
        .unwrap();
    assert_eq!(
        owner,
        OwnerOfResponse {
            owner: String::from("medusa"),
            approvals: vec![],
        }
    );

    // Cannot mint same token_id again
    let mint_msg2 = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("hercules"),
        token_uri: None,
        extension: Metadata {
            trait_type: String::from("hair"),
            trait_value: String::from("red"),
            trait_rarity: String::from("common"),
        },
    };

    let allowed = mock_info(MINTER, &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), allowed, mint_msg2)
        .unwrap_err();
    assert_eq!(err, ContractError::Claimed {});

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id], tokens.tokens);
}

#[test]
fn test_update_minter() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let token_id = "1".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: String::from("medusa"),
        token_uri: None,
        extension: Metadata {
            trait_type: String::from("hair"),
            trait_value: String::from("red"),
            trait_rarity: String::from("common"),
        },
    };

    // Minter can mint
    let minter_info = mock_info(MINTER, &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), minter_info.clone(), mint_msg)
        .unwrap();

    // Update the owner to "random". The new owner should be able to
    // mint new tokens, the old one should not.
    contract
        .execute(
            deps.as_mut(),
            mock_env(),
            minter_info.clone(),
            ExecuteMsg::UpdateOwnership(cw_ownable::Action::TransferOwnership {
                new_owner: "random".to_string(),
                expiry: None,
            }),
        )
        .unwrap();

    // Minter does not change until ownership transfer completes.
    let minter: MinterResponse = from_binary(
        &contract
            .query(deps.as_ref(), mock_env(), QueryMsg::Minter {})
            .unwrap(),
    )
    .unwrap();
    assert_eq!(minter.minter, Some(MINTER.to_string()));

    // Pending ownership transfer should be discoverable via query.
    let ownership: cw_ownable::Ownership<Addr> = from_binary(
        &contract
            .query(deps.as_ref(), mock_env(), QueryMsg::Ownership {})
            .unwrap(),
    )
    .unwrap();

    assert_eq!(
        ownership,
        cw_ownable::Ownership::<Addr> {
            owner: Some(Addr::unchecked(MINTER)),
            pending_owner: Some(Addr::unchecked("random")),
            pending_expiry: None,
        }
    );

    // Accept the ownership transfer.
    let random_info = mock_info("random", &[]);
    contract
        .execute(
            deps.as_mut(),
            mock_env(),
            random_info.clone(),
            ExecuteMsg::UpdateOwnership(cw_ownable::Action::AcceptOwnership),
        )
        .unwrap();

    // Minter changes after ownership transfer is accepted.
    let minter: MinterResponse = from_binary(
        &contract
            .query(deps.as_ref(), mock_env(), QueryMsg::Minter {})
            .unwrap(),
    )
    .unwrap();
    assert_eq!(minter.minter, Some("random".to_string()));

    let mint_msg = ExecuteMsg::Mint {
        token_id: "randoms_token".to_string(),
        owner: String::from("medusa"),
        token_uri: None,
        extension: Metadata {
            trait_type: String::from("hair"),
            trait_value: String::from("red"),
            trait_rarity: String::from("common"),
        },
    };

    // Old owner can not mint.
    let err: ContractError = contract
        .execute(deps.as_mut(), mock_env(), minter_info, mint_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));

    // New owner can mint.
    let _ = contract
        .execute(deps.as_mut(), mock_env(), random_info, mint_msg)
        .unwrap();
}

#[test]
fn burning() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let token_id = "1".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: MINTER.to_string(),
        token_uri: None,
        extension: Metadata {
            trait_type: String::from("hair"),
            trait_value: String::from("red"),
            trait_rarity: String::from("common"),
        },
    };

    let burn_msg = ExecuteMsg::Burn { token_id };

    // mint some NFT
    let allowed = mock_info(MINTER, &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), allowed.clone(), mint_msg)
        .unwrap();

    // random not allowed to burn
    let random = mock_info("random", &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), random, burn_msg.clone())
        .unwrap_err();

    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));

    let _ = contract
        .execute(deps.as_mut(), mock_env(), allowed, burn_msg)
        .unwrap();

    // ensure num tokens decreases
    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(0, count.count);

    // trying to get nft returns error
    let _ = contract
        .nft_info(deps.as_ref(), "petrify".to_string())
        .unwrap_err();

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert!(tokens.tokens.is_empty());
}

#[test]
fn query_tokens_by_owner() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());
    let minter = mock_info(MINTER, &[]);

    // Mint a couple tokens (from the same owner)
    let token_id1 = "grow1".to_string();
    let demeter = String::from("demeter");
    let token_id2 = "grow2".to_string();
    let ceres = String::from("ceres");
    let token_id3 = "sing".to_string();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id1.clone(),
        owner: demeter.clone(),
        token_uri: None,
        extension: Metadata {
            trait_type: String::from("hair"),
            trait_value: String::from("red"),
            trait_rarity: String::from("common"),
        },
    };
    contract
        .execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg)
        .unwrap();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id2.clone(),
        owner: ceres.clone(),
        token_uri: None,
        extension: Metadata {
            trait_type: String::from("hair"),
            trait_value: String::from("red"),
            trait_rarity: String::from("common"),
        },
    };
    contract
        .execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg)
        .unwrap();

    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id3.clone(),
        owner: demeter.clone(),
        token_uri: None,
        extension: Metadata {
            trait_type: String::from("hair"),
            trait_value: String::from("red"),
            trait_rarity: String::from("common"),
        },
    };
    contract
        .execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    // get all tokens in order:
    let expected = vec![token_id1.clone(), token_id2.clone(), token_id3.clone()];
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(&expected, &tokens.tokens);
    // paginate
    let tokens = contract.all_tokens(deps.as_ref(), None, Some(2)).unwrap();
    assert_eq!(&expected[..2], &tokens.tokens[..]);
    let tokens = contract
        .all_tokens(deps.as_ref(), Some(expected[1].clone()), None)
        .unwrap();
    assert_eq!(&expected[2..], &tokens.tokens[..]);

    // get by owner
    let by_ceres = vec![token_id2];
    let by_demeter = vec![token_id1, token_id3];
    // all tokens by owner
    let tokens = contract
        .tokens(deps.as_ref(), demeter.clone(), None, None)
        .unwrap();
    assert_eq!(&by_demeter, &tokens.tokens);
    let tokens = contract.tokens(deps.as_ref(), ceres, None, None).unwrap();
    assert_eq!(&by_ceres, &tokens.tokens);

    // paginate for demeter
    let tokens = contract
        .tokens(deps.as_ref(), demeter.clone(), None, Some(1))
        .unwrap();
    assert_eq!(&by_demeter[..1], &tokens.tokens[..]);
    let tokens = contract
        .tokens(deps.as_ref(), demeter, Some(by_demeter[0].clone()), Some(3))
        .unwrap();
    assert_eq!(&by_demeter[1..], &tokens.tokens[..]);
}

#[test]
fn query_collection_info() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let collection_info: CollectionInfoResponse = from_binary(
        &contract
            .query(deps.as_ref(), mock_env(), QueryMsg::CollectionInfo {})
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        collection_info,
        CollectionInfoResponse {
            creator: "creator".into(),
            description: "description".into(),
            image: Some("https://example.com/image.png".into()),
            external_link: None,
            explicit_content: None,
            royalty_info: None,
        }
    );
}

#[test]
fn update_collection() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let new_collection_info: UpdateCollectionInfoMsg<RoyaltyInfoResponse> =
        UpdateCollectionInfoMsg {
            description: Some("description_new".into()),
            image: Some("https://example-new.com/image.png".into()),
            external_link: None,
            explicit_content: None,
            royalty_info: None,
        };

    let update_collection_info_msg = ExecuteMsg::UpdateCollectionInfo {
        collection_info: new_collection_info,
    };

    let allowed = mock_info("creator", &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            allowed.clone(),
            update_collection_info_msg.clone(),
        )
        .unwrap();

    let collection_info: CollectionInfoResponse = from_binary(
        &contract
            .query(deps.as_ref(), mock_env(), QueryMsg::CollectionInfo {})
            .unwrap(),
    )
    .unwrap();

    assert_eq!(
        collection_info,
        CollectionInfoResponse {
            creator: "creator".into(),
            description: "description_new".into(),
            image: Some("https://example-new.com/image.png".into()),
            external_link: None,
            explicit_content: None,
            royalty_info: None,
        }
    );

    let not_allowed = mock_info("not_creator", &[]);
    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            not_allowed.clone(),
            update_collection_info_msg,
        )
        .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized {})
}

#[test]
fn freeze_collection() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());
    let allowed = mock_info("creator", &[]);

    let freeze_msg = ExecuteMsg::FreezeCollectionInfo {};

    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            allowed.clone(),
            freeze_msg.clone(),
        )
        .unwrap();

    let new_collection_info: UpdateCollectionInfoMsg<RoyaltyInfoResponse> =
        UpdateCollectionInfoMsg {
            description: Some("description_new".into()),
            image: Some("https://example-new.com/image.png".into()),
            external_link: None,
            explicit_content: None,
            royalty_info: None,
        };

    let update_collection_info_msg = ExecuteMsg::UpdateCollectionInfo {
        collection_info: new_collection_info,
    };
    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            allowed,
            update_collection_info_msg,
        )
        .unwrap_err();

    assert_eq!(err, ContractError::CollectionInfoFrozen {})
}
