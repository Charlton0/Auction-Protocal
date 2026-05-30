function AuctionCard({ title, bid }) {
  return (
    <div className="auction-card">
      <h3>{title}</h3>

      <p>Highest Bid: {bid} XLM</p>

      <button>View Auction</button>
    </div>
  );
}

export default AuctionCard;