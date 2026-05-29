#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, Address, Env, String,
};

use crate::{
    auction::{AuctionProtocol, AuctionProtocolClient},
    error::ContractError,
    storage::AuctionStatus,
};

fn create_token_contract<'a>(
    env: &Env,
    admin: &Address,
) -> (Address, token::StellarAssetClient<'a>) {
    let contract_id = env.register_stellar_asset_contract_v2(admin.clone());

    (
        contract_id.address(),
        token::StellarAssetClient::new(env, &contract_id.address()),
    )
}

struct SetupResult<'a> {
    env: Env,
    client: AuctionProtocolClient<'a>,
    seller: Address,
    bidder_one: Address,
    bidder_two: Address,
    token_client: token::StellarAssetClient<'a>,
    token_address: Address,
}

fn setup<'a>() -> SetupResult<'a> {
    let env = Env::default();

    env.mock_all_auths();

    let admin = Address::generate(&env);

    let seller = Address::generate(&env);

    let bidder_one = Address::generate(&env);

    let bidder_two = Address::generate(&env);

    let (token_address, token_client) = create_token_contract(&env, &admin);

    let contract_id = env.register(AuctionProtocol, ());

    let client = AuctionProtocolClient::new(&env, &contract_id);

    client.initialize(
    &admin,
    &token_address,
);

    SetupResult {
        env,
        client,
        seller,
        bidder_one,
        bidder_two,
        token_client,
        token_address,
    }
}

#[test]
fn test_create_auction() {
    let setup_result = setup();

    let item_name = String::from_str(&setup_result.env, "MacBook Pro");

    let item_description =
        String::from_str(&setup_result.env, "M2 Chip 16GB RAM");

    let deadline = setup_result.env.ledger().timestamp() + 1000;

    let item_description =
    String::from_str(&setup_result.env, "Auction item");

let auction_id = setup_result.client.create_auction(
    &setup_result.seller,
    &item_name,
    &item_description,
    &1000_i128,
    &deadline,
);

    let auction = setup_result.client.get_auction(&auction_id);

    assert_eq!(auction.auction_id, 1);
    assert_eq!(auction.item_name, item_name);
    assert_eq!(auction.highest_bid, 1000);
    assert_eq!(auction.status, AuctionStatus::Active);
}

#[test]
fn test_place_bid() {
    let setup_result = setup();

    let item_name = String::from_str(&setup_result.env, "Gaming Laptop");

    let deadline = setup_result.env.ledger().timestamp() + 1000;

    let item_description =
    String::from_str(&setup_result.env, "Auction item");

let auction_id = setup_result.client.create_auction(
    &setup_result.seller,
    &item_name,
    &item_description,
    &1000_i128,
    &deadline,
);

    let bid_amount = 2000_i128;

    setup_result
        .token_client
        .mint(&setup_result.bidder_one, &bid_amount);

   setup_result.client.place_bid(
    &auction_id,
    &setup_result.bidder_one,
    &bid_amount,
);

    let auction = setup_result.client.get_auction(&auction_id);

    assert_eq!(auction.highest_bid, bid_amount);
    assert_eq!(auction.highest_bidder, setup_result.bidder_one);
}

#[test]
fn test_bid_too_low() {
    let setup_result = setup();

    let item_name = String::from_str(&setup_result.env, "iPhone");

    let deadline = setup_result.env.ledger().timestamp() + 1000;

    let item_description =
    String::from_str(&setup_result.env, "Auction item");

    let auction_id = setup_result.client.create_auction(
       &setup_result.seller,
       &item_name,
       &item_description,
       &1000_i128,
       &deadline,
);
    

    setup_result
        .token_client
        .mint(&setup_result.bidder_one, &1500_i128);

    let result = setup_result.client.try_place_bid(
        &auction_id,
        &setup_result.bidder_one,
        &500_i128,
    );

    assert_eq!(result, Err(Ok(ContractError::BidTooLow)));
}

#[test]
fn test_finalize_auction() {
    let setup_result = setup();

    let item_name = String::from_str(&setup_result.env, "PlayStation 5");

    let deadline = setup_result.env.ledger().timestamp() + 1000;

    let item_description =
    String::from_str(&setup_result.env, "Auction item");

    let auction_id = setup_result.client.create_auction(
      &setup_result.seller,
      &item_name,
      &item_description,
      &1000_i128,
      &deadline,
);

    let bid_amount = 3000_i128;

    setup_result
        .token_client
        .mint(&setup_result.bidder_one, &bid_amount);

    setup_result.client.place_bid(
        &auction_id,
        &setup_result.bidder_one,
        &bid_amount,
    );

    setup_result.env.ledger().set_timestamp(deadline + 1);

    setup_result.client.finalize_auction(&auction_id);

    let auction = setup_result.client.get_auction(&auction_id);

    assert_eq!(auction.status, AuctionStatus::Finalized);
}

#[test]
fn test_cancel_auction_without_bids() {
    let setup_result = setup();

    let item_name = String::from_str(&setup_result.env, "Camera");

    let deadline = setup_result.env.ledger().timestamp() + 1000;

    let item_description =
    String::from_str(&setup_result.env, "Auction item");

    let auction_id = setup_result.client.create_auction(
       &setup_result.seller,
       &item_name,
       &item_description,
       &1000_i128,
       &deadline,
);

   setup_result
    .client
    .cancel_auction(&auction_id, &setup_result.seller);

    let auction = setup_result.client.get_auction(&auction_id);

    assert_eq!(auction.status, AuctionStatus::Cancelled);
}

#[test]
fn test_manual_claim_refund() {
    let setup_result = setup();

    let item_name = String::from_str(&setup_result.env, "Smart TV");

    let deadline = setup_result.env.ledger().timestamp() + 1000;

    let item_description =
    String::from_str(&setup_result.env, "Auction item");

    let auction_id = setup_result.client.create_auction(
      &setup_result.seller,
      &item_name,
      &item_description,
      &1000_i128,
      &deadline,
);

    setup_result
        .token_client
        .mint(&setup_result.bidder_one, &5000_i128);

    setup_result
        .token_client
        .mint(&setup_result.bidder_two, &7000_i128);

    setup_result.client.place_bid(
        &auction_id,
        &setup_result.bidder_one,
        &5000_i128,
    );

    setup_result.client.place_bid(
        &auction_id,
        &setup_result.bidder_two,
        &7000_i128,
    );

    let refund_before = setup_result
        .token_client
        .balance(&setup_result.bidder_one);

    setup_result
        .client
        .claim_refund(&setup_result.bidder_one);

    let refund_after = setup_result
        .token_client
        .balance(&setup_result.bidder_one);

    assert!(refund_after > refund_before);
}

#[test]
fn test_cannot_bid_after_deadline() {
    let setup_result = setup();

    let item_name = String::from_str(&setup_result.env, "Bicycle");

    let deadline = setup_result.env.ledger().timestamp() + 100;

    let item_description =
    String::from_str(&setup_result.env, "Auction item");

    let  auction_id = setup_result.client.create_auction(
         &setup_result.seller,
         &item_name,
         &item_description,
         &1000_i128,
         &deadline,
);

    setup_result.env.ledger().set_timestamp(deadline + 1);

    let result = setup_result.client.try_place_bid(
        &auction_id,
        &setup_result.bidder_one,
        &2000_i128,
    );

    assert_eq!(result, Err(Ok(ContractError::AuctionEnded)));
}

// #[test]
// fn test_remove_completed_auction() {
//     let setup_result = setup();

//     let item_name = String::from_str(&setup_result.env, "Printer");

//     let deadline = setup_result.env.ledger().timestamp() + 100;

//     let item_description =
//     String::from_str(&setup_result.env, "Auction item");

//     let auction_id = setup_result.client.create_auction(
//        &setup_result.seller,
//        &item_name,
//        &item_description,
//        &1000_i128,
//        &deadline,
// );

//     setup_result.env.ledger().set_timestamp(deadline + 1);

//     setup_result.client.finalize_auction(&auction_id);

//     setup_result.client.remove_auction(&auction_id);

//     let exists = setup_result
//         .env
//         .storage()
//         .persistent()
//         .has(&DataKey::Auction(auction_id));

//     assert!(!exists);
// }
