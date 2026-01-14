import { NavLink } from "react-router-dom";
import styles from "./Sidebar.module.css";

const items = [
  { to: "/overview", label: "Overview" },
  { to: "/prompt-loop", label: "Prompt Loop" },
  { to: "/loops", label: "Loops" },
  { to: "/evidence", label: "Evidence" },
  { to: "/approvals", label: "Approvals" },
  { to: "/agents", label: "Agents" },
  { to: "/protocols", label: "Protocols" },
  { to: "/documents", label: "Documents" },
  { to: "/audit", label: "Audit Log" },
  { to: "/settings", label: "Settings" },
];

export function Sidebar() {
  return (
    <div className={styles.wrap}>
      <div className={styles.brand}>
        <div className={styles.mark} />
        <div>
          <div className={styles.name}>Chirality AI</div>
          <div className={styles.tag}>Governance console</div>
        </div>
      </div>

      <nav className={styles.nav}>
        {items.map((i) => (
          <NavLink
            key={i.to}
            to={i.to}
            className={({ isActive }) =>
              [styles.link, isActive ? styles.active : ""].join(" ")
            }
          >
            <span className={styles.dot} />
            {i.label}
          </NavLink>
        ))}
      </nav>
    </div>
  );
}
