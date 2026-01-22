import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router";
import "./App.css";
import App from "./App";
import Layout from "./Layout";
import BrowserPage from "./features/browser/pages/BrowserPage";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />} >
          <Route index path="/" element={<App />} />
          <Route path="/browser" element={<BrowserPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  </React.StrictMode>,
);
