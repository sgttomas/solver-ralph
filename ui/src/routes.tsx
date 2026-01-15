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
import { Agents } from "./pages/Agents";
import { Protocols } from "./pages/Protocols";
import { Context } from "./pages/Context";
import { Audit } from "./pages/Audit";
import { Settings } from "./pages/Settings";

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

      // Agents
      { path: "/agents", element: <Agents /> },
      { path: "/agents/:agentId", element: <PlaceholderScreen title="Agent Detail" /> },

      // Protocols
      { path: "/protocols", element: <Protocols /> },
      { path: "/protocols/:templateId", element: <PlaceholderScreen title="Protocol Detail" /> },

      // Context (documents/intakes/bundles)
      { path: "/documents", element: <Context /> },
      { path: "/context/:documentId", element: <PlaceholderScreen title="Document Detail" /> },
      { path: "/context/intakes/:intakeId", element: <PlaceholderScreen title="Intake Detail" /> },
      { path: "/context/bundles/:bundleId", element: <PlaceholderScreen title="Bundle Detail" /> },

      // Audit Log
      { path: "/audit", element: <Audit /> },

      // Settings
      { path: "/settings", element: <Settings /> },

      { path: "*", element: <NotFoundScreen /> },
    ],
  },
]);
