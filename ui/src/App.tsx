/**
 * SOLVER-Ralph UI Application
 *
 * Portal UI for human review and governance decisions per SR-SPEC §2.
 * Provides views for:
 * - Loops, iterations, and candidates (D-29)
 * - Evidence bundles and manifests (D-29)
 * - Portal workflows: approvals, exceptions (D-30)
 */

import { useState, useEffect } from 'react'

interface ApiInfo {
  name: string
  version: string
  description: string
}

function App() {
  const [apiInfo, setApiInfo] = useState<ApiInfo | null>(null)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    fetch('/api/v1/info')
      .then(res => res.json())
      .then(data => setApiInfo(data))
      .catch(err => setError(`Failed to connect to API: ${err.message}`))
  }, [])

  return (
    <div style={{ fontFamily: 'system-ui, sans-serif', padding: '2rem' }}>
      <h1>SOLVER-Ralph</h1>
      <p>Governance-first platform for controlled agentic work</p>

      {error && (
        <div style={{ color: 'red', padding: '1rem', background: '#fee' }}>
          {error}
        </div>
      )}

      {apiInfo && (
        <div style={{ padding: '1rem', background: '#f0f0f0', borderRadius: '4px' }}>
          <h2>API Status</h2>
          <p><strong>Name:</strong> {apiInfo.name}</p>
          <p><strong>Version:</strong> {apiInfo.version}</p>
          <p><strong>Description:</strong> {apiInfo.description}</p>
        </div>
      )}

      <h2>Portal Views (Coming in D-29, D-30)</h2>
      <ul>
        <li>Loop/Iteration/Candidate Views</li>
        <li>Evidence Bundle Viewer</li>
        <li>Portal Workflows (Approvals, Exceptions)</li>
      </ul>
    </div>
  )
}

export default App
