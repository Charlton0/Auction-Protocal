import { useState } from "react";
import Navbar from "../components/Navbar";

function CreateAuction() {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [startingBid, setStartingBid] = useState("");

  const handleSubmit = (e) => {
    e.preventDefault();

    const auctionData = {
      title,
      description,
      startingBid,
    };

    console.log("Auction Created:", auctionData);

    alert("Auction created (frontend only for now)");
  };

  return (
    <>
      <Navbar />

      <div className="container">
        <h2>Create Auction</h2>

        <form onSubmit={handleSubmit} className="form">
          <input
            type="text"
            placeholder="Item title"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
          />

          <textarea
            placeholder="Description"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
          />

          <input
            type="number"
            placeholder="Starting bid (XLM)"
            value={startingBid}
            onChange={(e) => setStartingBid(e.target.value)}
          />

          <button type="submit">Create Auction</button>
        </form>
      </div>
    </>
  );
}

export default CreateAuction;