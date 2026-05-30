import { BrowserRouter, Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import CreateAuction from "./pages/CreateAuction";
import AuctionDetails from "./pages/AuctionDetails";
import RefundPage from "./pages/RefundPage";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/create" element={<CreateAuction />} />
        <Route path="/auction" element={<AuctionDetails />} />
        <Route path="/refund"element={<RefundPage />} />
  
  

      </Routes>
    </BrowserRouter>
  );
}

export default App;