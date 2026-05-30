const CONTRACT_ID =
  "CBYWDUQ6CW3MI5EPQ54KKY5PBGFJZKG25SFQPGAV3MVI4XE7WWI2SDX5";

const NETWORK = "TESTNET";

export async function createAuction(data) {
  console.log("Create Auction Request");

  console.log("Item:", data.title);
  console.log("Description:", data.description);
  console.log("Starting Bid:", data.startingBid);
  console.log("Deadline:", data.deadline);

  return true;
}

export async function getAuction(id) {
  console.log("Get Auction", id);
}

export async function placeBid(id, amount) {
  console.log("Place Bid", id, amount);
}

export async function finalizeAuction(id) {
  console.log("Finalize Auction", id);
}

export async function cancelAuction(id) {
  console.log("Cancel Auction", id);
}

export async function claimRefund(walletAddress) {
  console.log(
    "Claim Refund:",
    walletAddress
  );

  return true;
}

export { CONTRACT_ID, NETWORK };