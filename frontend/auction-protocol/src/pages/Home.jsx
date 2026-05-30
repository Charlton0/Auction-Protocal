import Navbar from "../components/Navbar";
import AuctionCard from "../components/AuctionCard";

function Home() {
  return (
    <>
      <Navbar />

      <div className="container">
        <header className="hero">
          <h1>No-Loss Auction Protocol</h1>

          <p>
            Bid on items without losing your funds.
          </p>
        </header>

        <section className="auctions">
          <h2>Featured Auctions</h2>

          <AuctionCard
            title="Gaming Laptop"
            bid="100"
          />

          <AuctionCard
            title="iPhone 15"
            bid="75"
          />

          <AuctionCard
            title="Smart TV"
            bid="120"
          />
        </section>
      </div>
    </>
  );
}

export default Home;