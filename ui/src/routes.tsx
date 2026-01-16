import { createBrowserRouter, Navigate } from "react-router-dom";
import { AppLayout } from "./layout/AppLayout";

// New wireframe screens
import { OverviewScreen } from "./screens/OverviewScreen";
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
import { AgentDetail } from "./pages/AgentDetail";
import { Protocols } from "./pages/Protocols";
import { ProtocolDetail } from "./pages/ProtocolDetail";
import { Oracles } from "./pages/Oracles";
import { OracleSuiteDetail } from "./pages/OracleSuiteDetail";
import { VerificationProfileDetail } from "./pages/VerificationProfileDetail";
import { Templates } from "./pages/Templates";
import { TemplateDetail } from "./pages/TemplateDetail";
import { Workflows } from "./pages/Workflows";
import { Intakes } from "./pages/Intakes";
import { IntakeCreate } from "./pages/IntakeCreate";
import { IntakeDetail } from "./pages/IntakeDetail";
import { IntakeEdit } from "./pages/IntakeEdit";
import { References } from "./pages/References";
import { ReferenceDocumentDetail } from "./pages/ReferenceDocumentDetail";
import { ReferenceBundleDetail } from "./pages/ReferenceBundleDetail";
import { GovernedArtifactDetail } from "./pages/GovernedArtifactDetail";
import { WorkSurfaces } from "./pages/WorkSurfaces";
import { WorkSurfaceCompose } from "./pages/WorkSurfaceCompose";
import { WorkSurfaceDetail } from "./pages/WorkSurfaceDetail";
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
      { path: "/artifacts", element: <Evidence /> },
      { path: "/artifacts/:contentHash", element: <EvidenceDetail /> },
      { path: "/approvals", element: <Approvals /> },
      { path: "/prompts", element: <PromptLoop /> },

      // Agents
      { path: "/agents", element: <Agents /> },
      { path: "/agents/:agentId", element: <AgentDetail /> },

      // Protocols
      { path: "/protocols", element: <Protocols /> },
      { path: "/protocols/:templateId", element: <ProtocolDetail /> },

      // Oracles
      { path: "/oracles", element: <Oracles /> },
      { path: "/oracles/suites/:suiteId", element: <OracleSuiteDetail /> },
      { path: "/oracles/profiles/:profileId", element: <VerificationProfileDetail /> },

      // Templates
      { path: "/templates", element: <Templates /> },
      { path: "/templates/:category/:templateId", element: <TemplateDetail /> },

      // Workflows
      { path: "/workflows", element: <Workflows /> },

      // Intakes
      { path: "/intakes", element: <Intakes /> },
      { path: "/intakes/new", element: <IntakeCreate /> },
      { path: "/intakes/:intakeId", element: <IntakeDetail /> },
      { path: "/intakes/:intakeId/edit", element: <IntakeEdit /> },

      // Work Surfaces
      { path: "/work-surfaces", element: <WorkSurfaces /> },
      { path: "/work-surfaces/new", element: <WorkSurfaceCompose /> },
      { path: "/work-surfaces/:workSurfaceId", element: <WorkSurfaceDetail /> },

      // References (renamed from Context)
      { path: "/references", element: <References /> },
      { path: "/references/documents/:documentId", element: <ReferenceDocumentDetail /> },
      { path: "/references/bundles/:bundleId", element: <ReferenceBundleDetail /> },
      { path: "/references/governed-artifacts/:artifactId", element: <GovernedArtifactDetail /> },

      // Audit Log
      { path: "/audit", element: <Audit /> },

      // Settings
      { path: "/settings", element: <Settings /> },

      { path: "*", element: <NotFoundScreen /> },
    ],
  },
]);
