import { Link } from "react-router-dom";

function Navbar() {
  return (
    <nav className="navbar">
      <h2>No-Loss Auction</h2>

      <div style={{ display: "flex", gap: "10px" }}>
        <Link to="/">Home</Link>
        <Link to="/create">Create Auction</Link>
        <Link to="/auction">Auction Details</Link>
        <Link to="/refund">Refund</Link>
        <button>Connect Wallet</button>
      </div>
    </nav>
  );
}

export default Navbar;