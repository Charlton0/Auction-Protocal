use soroban_sdk:: contracterror;

#[repr(u32)]
#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractError { // defines the rules/ what users should not do
    AuctionEnded = 1,
    BidTooLow = 2,
    Unauthorized = 3,
    AuctionNotFound = 4,
    AuctionAlreadyFinalized = 5,
    NoRefundAvailable = 6,
    AuctionCancelled = 7,
    CannotCancelAuction = 8,
    InvalidDeadline = 9,
    SellerCannotBid = 10,
    AuctionStillActive = 11,
    InvalidBidAmount = 12,

}