use soroban_sdk::{
    contract, contractimpl, token, Address, Env, String,
};

use crate::{
    error::ContractError,
    events::{
        AuctionCancelledEvent, AuctionCreatedEvent,
        AuctionFinalizedEvent, BidPlacedEvent,
        RefundClaimedEvent,
    },
    storage::{Auction, AuctionStatus, DataKey},
};

#[contract]
pub struct AuctionProtocol;

#[contractimpl]
impl AuctionProtocol {

    pub fn initialize(
        env: Env,
        admin: Address,
        token: Address,
    ) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::AuctionCount, &0u64);
    }

    
    // function to create an auction
    
    pub fn create_auction(
        env: Env,
        seller: Address,
        item_name: String,
        item_description: String,
        starting_price: i128,
        deadline: u64,
    ) -> Result<u64, ContractError> {

        seller.require_auth();

        if deadline <= env.ledger().timestamp() {
            return Err(ContractError::InvalidDeadline);
        }

        if starting_price <= 0 {
            return Err(ContractError::InvalidBidAmount);
        }

        let mut auction_count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::AuctionCount)
            .unwrap_or(0);

        auction_count += 1;

        let auction = Auction {
            auction_id: auction_count,
            seller: seller.clone(),
            item_name: item_name.clone(),
            item_description,
            starting_price,
            highest_bid: starting_price,
            highest_bidder: seller.clone(),
            deadline,
            active: true,
            status: AuctionStatus::Active,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Auction(auction_count), &auction);

        env.storage()
            .instance()
            .set(&DataKey::AuctionCount, &auction_count);

        AuctionCreatedEvent {
            auction_id: auction_count,
            seller,
            item_name,
            starting_price,
            deadline,
        }
        .publish(&env);

        Ok(auction_count)
    }

    
    // function to fetch an auction
    
    pub fn get_auction(
        env: Env,
        auction_id: u64,
    ) -> Auction {

        env.storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .unwrap()
    }


    // function to place bid goes here
    
    pub fn place_bid(
        env: Env,
        auction_id: u64,
        bidder: Address,
        amount: i128,
    ) -> Result<(), ContractError> {

        bidder.require_auth();

        let mut auction: Auction = Self::get_auction(
            env.clone(),
            auction_id,
        );

        if !auction.active {
            return Err(ContractError::AuctionEnded);
        }

        if env.ledger().timestamp() >= auction.deadline {
            return Err(ContractError::AuctionEnded);
        }

        if amount <= auction.highest_bid {
            return Err(ContractError::BidTooLow);
        }

        if bidder == auction.seller {
            return Err(ContractError::SellerCannotBid);
        }

        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .expect("TOKEN NOT INITIALIZED"); // testing 

        let token_client = token::Client::new(
            &env,
            &token_address,
        );

        // Transfer bid amount into contract
        token_client.transfer(
            &bidder,
            &env.current_contract_address(),
            &amount,
        );

        // Save refund for previous bidder
        if auction.highest_bidder != auction.seller {

            let refund_key = DataKey::Refund(
                auction.highest_bidder.clone(),
            );

            let existing_refund: i128 = env
                .storage()
                .persistent()
                .get(&refund_key)
                .unwrap_or(0);

            env.storage()
                .persistent()
                .set(
                    &refund_key,
                    &(existing_refund + auction.highest_bid),
                );
        }

        auction.highest_bid = amount;
        auction.highest_bidder = bidder.clone();

        env.storage()
            .persistent()
            .set(
                &DataKey::Auction(auction_id),
                &auction,
            );

        BidPlacedEvent {
            auction_id,
            bidder,
            amount,
        }
        .publish(&env);

        Ok(())
    }

    
    // fuction for claiming refund
    
    pub fn claim_refund(
        env: Env,
        bidder: Address,
    ) -> Result<(), ContractError> {

        bidder.require_auth();

        let refund_key = DataKey::Refund(
            bidder.clone(),
        );

        let refund_amount: i128 = env
            .storage()
            .persistent()
            .get(&refund_key)
            .unwrap_or(0);

        if refund_amount <= 0 {
            return Err(ContractError::NoRefundAvailable);
        }

        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .unwrap();

        let token_client = token::Client::new(
            &env,
            &token_address,
        );

        token_client.transfer(
            &env.current_contract_address(),
            &bidder,
            &refund_amount,
        );

        env.storage()
            .persistent()
            .set(&refund_key, &0i128);

    
let timestamp = env.ledger().timestamp();

        RefundClaimedEvent {
            bidder,
            amount: refund_amount,
            timestamp,
        }
        .publish(&env);

        Ok(())
    }

    
    // finalize auction function
    
    pub fn finalize_auction(
        env: Env,
        auction_id: u64,
    ) -> Result<(), ContractError> {

        let mut auction: Auction = Self::get_auction(
            env.clone(),
            auction_id,
        );

        if !auction.active {
            return Err(
                ContractError::AuctionAlreadyFinalized
            );
        }

        if env.ledger().timestamp() < auction.deadline {
            return Err(ContractError::AuctionStillActive);
        }

        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .unwrap();

        let token_client = token::Client::new(
            &env,
            &token_address,
        );

        // Transfer winning bid to seller
        token_client.transfer(
            &env.current_contract_address(),
            &auction.seller,
            &auction.highest_bid,
        );

        auction.active = false;
        auction.status = AuctionStatus::Finalized;

        env.storage()
            .persistent()
            .set(
                &DataKey::Auction(auction_id),
                &auction,
            );

        AuctionFinalizedEvent {
            auction_id,
            winner: auction.highest_bidder,
            amount: auction.highest_bid,
        }
        .publish(&env);

        Ok(())
    }

    
    // function to cancel auction, only when no bid has been placed
    
    pub fn cancel_auction(
        env: Env,
        auction_id: u64,
        seller: Address,
    ) -> Result<(), ContractError> {

        seller.require_auth();

        let mut auction: Auction = Self::get_auction(
            env.clone(),
            auction_id,
        );

        if seller != auction.seller {
            return Err(ContractError::Unauthorized);
        }

        if auction.highest_bidder != auction.seller {
            return Err(ContractError::CannotCancelAuction);
        }

        auction.active = false;
        auction.status = AuctionStatus::Cancelled;

        env.storage()
            .persistent()
            .set(
                &DataKey::Auction(auction_id),
                &auction,
            );

        AuctionCancelledEvent {
            auction_id,
            seller,
        }
        .publish(&env);

        Ok(())
    }
}

