⏺ UI Codebase Summary

  Tech Stack: React 18 + React Router 6 + Vite + TypeScript + OIDC (Zitadel)

  Directory Structure

  ui/src/
  ├── auth/          # OIDC authentication (AuthProvider, ProtectedRoute)
  ├── components/    # Reusable components
  ├── layout/        # AppLayout, Sidebar, Topbar
  ├── pages/         # Functional pages with API integration
  ├── screens/       # Placeholder/wireframe screens
  ├── styles/        # Global CSS + theme variables
  ├── ui/            # UI primitives (Button, Card, Pill)
  ├── routes.tsx     # Router config
  └── config.ts      # Environment config

  Page Implementation Status
  ┌─────────────────┬─────────────────┬─────────────────────────────┐
  │      Route      │      Page       │           Status            │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /overview       │ OverviewScreen  │ ✅ Dashboard with metrics   │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /loops          │ Loops           │ ✅ Lists work units         │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /loops/:id      │ LoopDetail      │ ✅ Loop + iterations        │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /iterations/:id │ IterationDetail │ ✅ Context refs, candidates │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /candidates/:id │ CandidateDetail │ ✅ Runs, evidence, freeze   │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /evidence       │ Evidence        │ ✅ Lists artifacts          │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /evidence/:hash │ EvidenceDetail  │ ✅ Oracle results           │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /approvals      │ Approvals       │ ✅ Portal workflows         │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /prompt-loop    │ PromptLoop      │ ✅ SSE streaming            │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /agents         │ Placeholder     │ 🔶 Future                   │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /protocols      │ Placeholder     │ 🔶 Future                   │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /documents      │ Placeholder     │ 🔶 Future                   │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /audit          │ Placeholder     │ 🔶 Future                   │
  ├─────────────────┼─────────────────┼─────────────────────────────┤
  │ /settings       │ Placeholder     │ 🔶 Future                   │
  └─────────────────┴─────────────────┴─────────────────────────────┘
  Key Observations

  1. Styling inconsistency: Functional pages use inline styles; there's a nicer component library (Button, Card, Pill) in ui/src/ui/ that isn't being used
  2. No global state: Each page manages its own state with useState/useEffect
  3. Direct fetch calls: No API client abstraction - each page calls fetch() directly
  4. Design system exists: CSS variables theme in styles/theme.css with warm beige palette

  Previous Session Notes

  From SR-README, the last session noted:
  "The existing functional pages use inline styles; they work but don't match the new Chirality design aesthetic. Future work: Port existing pages to use Card/Pill/Button primitives."

  ---
  What would you like me to focus on? Options include:
  - A) Port existing pages to use the Card/Pill/Button component library
  - B) Implement one of the placeholder screens (Agents, Protocols, etc.)
  - C) Add new features to existing pages
  - D) Fix specific bugs or issues
  - E) Something else
