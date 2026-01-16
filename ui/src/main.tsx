/**
 * SOLVER-Ralph UI Entry Point
 *
 * Bootstraps the React application with the Chirality AI governance console.
 */

import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router-dom";
import { AuthProvider } from "./auth";
import { ToastProvider } from "./components/ToastContext";
import { router } from "./routes";

import "./styles/theme.css";
import "./styles/app.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <AuthProvider>
      <ToastProvider>
        <RouterProvider router={router} />
      </ToastProvider>
    </AuthProvider>
  </React.StrictMode>
);
