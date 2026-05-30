import { Link } from "react-router-dom";

function Home() {
  return (
    <div style={{ padding: "2rem" }}>
      <h1>No Loss Auction Protocol</h1>

      <p>
        Create auctions, place bids and claim refunds on Stellar Soroban.
      </p>

      <div style={{ marginTop: "20px" }}>
        <Link to="/create">
          <button>Create Auction</button>
        </Link>
      </div>

      <div style={{ marginTop: "20px" }}>
        <Link to="/auction">
          <button>View Auction</button>
        </Link>
      </div>
    </div>
  );
}

export default Home;