import { useState } from "react";
import Navbar from "../components/Navbar";
import { createAuction } from "../services/soroban";

function CreateAuction() {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [startingBid, setStartingBid] = useState("");
  const [deadline, setDeadline] = useState("");

  const handleSubmit = async (e) => {
    e.preventDefault();

    const auctionData = {
      title,
      description,
      startingBid,
      deadline,
    };

    console.log("Creating Auction:", auctionData);

    await createAuction(auctionData);

    alert("Auction submitted");
  };

  return (
    <>
      <Navbar />

      <div className="container">
        <div className="card">
          <h2>Create New Auction</h2>

          <form onSubmit={handleSubmit} className="form">

            <label>Item Name</label>
            <input
              type="text"
              placeholder="Gaming Laptop"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              required
            />

            <label>Description</label>
            <textarea
              rows="5"
              placeholder="Describe your item..."
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              required
            />

            <label>Starting Price</label>
            <input
              type="number"
              placeholder="100"
              value={startingBid}
              onChange={(e) => setStartingBid(e.target.value)}
              required
            />

            <label>Auction Deadline</label>
            <input
              type="datetime-local"
              value={deadline}
              onChange={(e) => setDeadline(e.target.value)}
              required
            />

            <button type="submit">
              Create Auction
            </button>

          </form>
        </div>
      </div>
    </>
  );
}

export default CreateAuction;