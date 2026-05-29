use soroban_sdk::{contractevent, Address};

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuctionCreatedEvent{

}

pub struct BidPlacedEvent{

}

pub struct RefundClaimedEvent{

}

pub struct AuctionCancelledEvent{

}

pub struct AuctionFinalizedEvent{
    
}