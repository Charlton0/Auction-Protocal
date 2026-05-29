use soroban_sdk::{contractevent, Address, String};


#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuctionCreatedEvent{
    pub auction_id: u64,
    pub seller: Address,
    pub item_name: String,
    pub starting_price: i128,
    pub deadline: u64, 

}


#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BidPlacedEvent{
    pub auction_id: u64,
    pub bidder: Address,
    pub amount: i128,

}


#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefundClaimedEvent{
    pub bidder: Address,
    pub amount: i128,
    pub timestamp: u64,

}


#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuctionCancelledEvent{
    pub auction_id: u64,
    pub seller: Address,

}


#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuctionFinalizedEvent{
    pub auction_id: u64,
    pub winner: Address,
    pub amount: i128,
   
}

