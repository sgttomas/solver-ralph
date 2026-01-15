import { useAuth } from "../auth";
import config from "../config";
import styles from "./Topbar.module.css";

export function Topbar() {
  const auth = useAuth();

  const handleLogout = () => {
    if (!config.devAuthBypass) {
      auth.signoutRedirect();
    }
  };

  const userName = auth.user?.profile?.name || auth.user?.profile?.email || "User";

  return (
    <div className={styles.bar}>
      <div className={styles.left}>
        <span className={styles.label}>Workspace</span>
        <select className={styles.select}>
          <option>Chirality</option>
        </select>
      </div>

      <div className={styles.searchWrap}>
        <input
          className={styles.search}
          placeholder="Search workflows, protocols, docs…  (⌘K)"
        />
      </div>

      <div className={styles.right}>
        <span className={styles.pill}>Env: Production</span>
        {auth.isAuthenticated && (
          <>
            <span className={styles.pill}>{userName}</span>
            {!config.devAuthBypass && (
              <button className={styles.logoutBtn} onClick={handleLogout}>
                Logout
              </button>
            )}
          </>
        )}
      </div>
    </div>
  );
}
