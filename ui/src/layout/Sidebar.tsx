import { NavLink } from "react-router-dom";
import styles from "./Sidebar.module.css";

const items = [
  { to: "/work", label: "Work" },
  { to: "/work-surfaces/new", label: "New Work" },
  { to: "/settings", label: "Settings" },
  { to: "/audit", label: "Audit" },
];

export function Sidebar() {
  return (
    <div className={styles.wrap}>
      <div className={styles.brand}>
        <img src="/logo.png" alt="Chirality AI" className={styles.mark} />
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
