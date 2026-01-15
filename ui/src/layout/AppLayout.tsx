import { Outlet, Navigate } from "react-router-dom";
import { useAuth } from "../auth";
import { Sidebar } from "./Sidebar";
import { Topbar } from "./Topbar";
import styles from "./AppLayout.module.css";
import config from "../config";

export function AppLayout() {
  const auth = useAuth();

  // In non-dev mode, require authentication
  if (!config.devAuthBypass && !auth.isAuthenticated && !auth.isLoading) {
    return <Navigate to="/callback" replace />;
  }

  // Show loading state while auth is being determined
  if (auth.isLoading) {
    return (
      <div className="appBg">
        <div className={styles.frame}>
          <div className={styles.loading}>Loading...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="appBg">
      <div className={styles.frame}>
        <aside className={styles.sidebar}>
          <Sidebar />
        </aside>

        <div className={styles.main}>
          <header className={styles.topbar}>
            <Topbar />
          </header>

          <main className={styles.content}>
            <Outlet />
          </main>
        </div>
      </div>
    </div>
  );
}
