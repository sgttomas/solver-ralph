/**
 * SOLVER-Ralph UI Application (D-28)
 *
 * Portal UI for human review and governance decisions per SR-SPEC §2.
 * Provides:
 * - OIDC authentication with Zitadel
 * - Protected routes for authenticated users
 * - Views for loops, evidence, and approvals
 */

import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { Layout } from './components';
import { ProtectedRoute } from './auth';
import { Home, Callback, Loops, Evidence, Approvals } from './pages';

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
            path="/evidence"
            element={
              <ProtectedRoute>
                <Evidence />
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
        </Routes>
      </Layout>
    </BrowserRouter>
  );
}

export default App;
