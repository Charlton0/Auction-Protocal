use soroban_sdk::{contracttype, Address, String};

#[derive(Clone)]
#[contracttype]
pub struct Auction {
   pub auction_id: u64,
   pub seller: Address,
   pub item_name: String,
   pub item_description: String,
   pub starting_price: i128,
   pub highest_bid: i128,
   pub highest_bidder: Address,
   pub deadline: u64,
   pub active: bool,
   pub status: AuctionStatus,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey { // stores the data variantsSS
    Admin,
    Token,
    AuctionCount, // auction ID counter
    Auction(u64), // store auction by ID
    AuctionBids(u64), // stores all bids for an auction 
    Refund(Address), // refund claim 
    SellerAuctions(Address) // addresses for actions placed by the seller

}

#[derive(Clone)]
#[contracttype]
pub struct Bid {  // this struct stores bid details/history 
    pub bidder: Address, 
    pub amount: i128, 
    pub time_stamp: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub enum AuctionStatus { //this  will store the current state of the auction
    Active, 
    Cancelled,
    Finalized,
    
}