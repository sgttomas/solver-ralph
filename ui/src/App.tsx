/**
 * SOLVER-Ralph UI Application (D-29)
 *
 * Portal UI for human review and governance decisions per SR-SPEC §2.
 * Provides:
 * - OIDC authentication with Zitadel
 * - Protected routes for authenticated users
 * - Views for loops, iterations, candidates, evidence, and approvals
 */

import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { Layout } from './components';
import { ProtectedRoute } from './auth';
import {
  Home,
  Callback,
  Loops,
  LoopDetail,
  IterationDetail,
  CandidateDetail,
  Evidence,
  EvidenceDetail,
  Approvals,
  PromptLoop,
} from './pages';

function App(): JSX.Element {
  return (
    <BrowserRouter>
      <Layout>
        <Routes>
          {/* Public routes */}
          <Route path="/" element={<Home />} />
          <Route path="/callback" element={<Callback />} />

          {/* Protected routes - require authentication */}
          <Route
            path="/loops"
            element={
              <ProtectedRoute>
                <Loops />
              </ProtectedRoute>
            }
          />
          <Route
            path="/loops/:loopId"
            element={
              <ProtectedRoute>
                <LoopDetail />
              </ProtectedRoute>
            }
          />
          <Route
            path="/iterations/:iterationId"
            element={
              <ProtectedRoute>
                <IterationDetail />
              </ProtectedRoute>
            }
          />
          <Route
            path="/candidates/:candidateId"
            element={
              <ProtectedRoute>
                <CandidateDetail />
              </ProtectedRoute>
            }
          />
          <Route
            path="/evidence"
            element={
              <ProtectedRoute>
                <Evidence />
              </ProtectedRoute>
            }
          />
          <Route
            path="/evidence/:contentHash"
            element={
              <ProtectedRoute>
                <EvidenceDetail />
              </ProtectedRoute>
            }
          />
          <Route
            path="/approvals"
            element={
              <ProtectedRoute>
                <Approvals />
              </ProtectedRoute>
            }
          />
          <Route
            path="/prompt"
            element={
              <ProtectedRoute>
                <PromptLoop />
              </ProtectedRoute>
            }
          />
        </Routes>
      </Layout>
    </BrowserRouter>
  );
}

export default App;
