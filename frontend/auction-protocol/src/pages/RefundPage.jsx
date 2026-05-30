import { useState } from "react";
import Navbar from "../components/Navbar";

function RefundPage() {
  const [walletAddress, setWalletAddress] = useState("");

  const claimRefund = async () => {
    console.log(
      "Claiming refund for:",
      walletAddress
    );

    /*
      Later:

      await claimRefund(walletAddress);
    */

    alert("Refund request submitted");
  };

  return (
    <>
      <Navbar />

      <div className="container">
        <div className="card">

          <h2>Claim Refund</h2>

          <p>
            If you were outbid in an auction,
            you can reclaim your deposited funds.
          </p>

          <br />

          <input
            type="text"
            placeholder="Wallet Address"
            value={walletAddress}
            onChange={(e) =>
              setWalletAddress(e.target.value)
            }
          />

          <button onClick={claimRefund}>
            Claim Refund
          </button>

        </div>
      </div>
    </>
  );
}

export default RefundPage;