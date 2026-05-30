import { useState } from "react";
import Navbar from "../components/Navbar";

function AuctionDetails() {
  const [auctionId, setAuctionId] = useState("");
  const [bidAmount, setBidAmount] = useState("");

  const [auction, setAuction] = useState(null);

  const loadAuction = async () => {
    console.log("Loading auction:", auctionId);

    /*
      Later:
      const result = await getAuction(auctionId);
      setAuction(result);
    */

    // Temporary mock data
    setAuction({
      auction_id: auctionId,
      item_name: "Gaming Laptop",
      item_description: "16GB RAM, RTX Graphics",
      seller: "GABCD123...",
      starting_price: 100,
      highest_bid: 150,
      highest_bidder: "GXYZ789...",
      deadline: "2026-06-10",
      active: true,
      status: "Active",
    });
  };

  const placeBid = async () => {
    console.log(
      "Bid placed:",
      bidAmount,
      "on auction",
      auctionId
    );

    /*
      Later:
      await placeBid(
        auctionId,
        bidAmount
      );
    */
  };

  const finalizeAuction = async () => {
    console.log(
      "Finalize auction",
      auctionId
    );
  };

  const cancelAuction = async () => {
    console.log(
      "Cancel auction",
      auctionId
    );
  };

  return (
    <>
      <Navbar />

      <div className="container">

        <div className="card">

          <h2>Auction Details</h2>

          <input
            type="number"
            placeholder="Auction ID"
            value={auctionId}
            onChange={(e) =>
              setAuctionId(e.target.value)
            }
          />

          <button
            onClick={loadAuction}
            style={{ marginTop: "10px" }}
          >
            Load Auction
          </button>

        </div>

        {auction && (

          <div
            className="card"
            style={{ marginTop: "20px" }}
          >

            <h2>{auction.item_name}</h2>

            <p>
              {auction.item_description}
            </p>

            <br />

            <p>
              <strong>Seller:</strong>{" "}
              {auction.seller}
            </p>

            <p>
              <strong>Starting Price:</strong>{" "}
              {auction.starting_price}
            </p>

            <p>
              <strong>Highest Bid:</strong>{" "}
              {auction.highest_bid}
            </p>

            <p>
              <strong>Highest Bidder:</strong>{" "}
              {auction.highest_bidder}
            </p>

            <p>
              <strong>Deadline:</strong>{" "}
              {auction.deadline}
            </p>

            <p>
              <strong>Status:</strong>{" "}
              {auction.status}
            </p>

            <br />

            <input
              type="number"
              placeholder="Enter bid amount"
              value={bidAmount}
              onChange={(e) =>
                setBidAmount(e.target.value)
              }
            />

            <button onClick={placeBid}>
              Place Bid
            </button>

            <button
              onClick={finalizeAuction}
              style={{
                marginLeft: "10px",
              }}
            >
              Finalize
            </button>

            <button
              onClick={cancelAuction}
              style={{
                marginLeft: "10px",
              }}
            >
              Cancel
            </button>

          </div>

        )}
      </div>
    </>
  );
}

export default AuctionDetails;