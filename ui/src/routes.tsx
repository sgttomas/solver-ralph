import { createBrowserRouter, Navigate } from "react-router-dom";
import { AppLayout } from "./layout/AppLayout";

// New wireframe screens
import { OverviewScreen } from "./screens/OverviewScreen";
import { PlaceholderScreen } from "./screens/PlaceholderScreen";
import { NotFoundScreen } from "./screens/NotFoundScreen";

// Existing functional pages
import { Callback } from "./pages/Callback";
import { Loops } from "./pages/Loops";
import { LoopDetail } from "./pages/LoopDetail";
import { IterationDetail } from "./pages/IterationDetail";
import { CandidateDetail } from "./pages/CandidateDetail";
import { Evidence } from "./pages/Evidence";
import { EvidenceDetail } from "./pages/EvidenceDetail";
import { Approvals } from "./pages/Approvals";
import { PromptLoop } from "./pages/PromptLoop";

export const router = createBrowserRouter([
  // Callback outside layout (OIDC redirect)
  { path: "/callback", element: <Callback /> },

  {
    element: <AppLayout />,
    children: [
      { path: "/", element: <Navigate to="/overview" replace /> },

      // New wireframe
      { path: "/overview", element: <OverviewScreen /> },

      // Functional pages
      { path: "/loops", element: <Loops /> },
      { path: "/loops/:loopId", element: <LoopDetail /> },
      { path: "/iterations/:iterationId", element: <IterationDetail /> },
      { path: "/candidates/:candidateId", element: <CandidateDetail /> },
      { path: "/evidence", element: <Evidence /> },
      { path: "/evidence/:contentHash", element: <EvidenceDetail /> },
      { path: "/approvals", element: <Approvals /> },
      { path: "/prompt-loop", element: <PromptLoop /> },

      // Placeholders for future
      { path: "/agents", element: <PlaceholderScreen title="Agents" /> },
      { path: "/protocols", element: <PlaceholderScreen title="Protocols" /> },
      { path: "/documents", element: <PlaceholderScreen title="Context" /> },
      { path: "/audit", element: <PlaceholderScreen title="Audit Log" /> },
      { path: "/settings", element: <PlaceholderScreen title="Settings" /> },

      { path: "*", element: <NotFoundScreen /> },
    ],
  },
]);
