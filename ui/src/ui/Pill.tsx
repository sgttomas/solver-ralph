import styles from "./Pill.module.css";

export type PillTone = "neutral" | "success" | "warning" | "danger" | "info";

export function Pill({
  tone = "neutral",
  children,
}: {
  tone?: PillTone;
  children: React.ReactNode;
}) {
  return <span className={`${styles.pill} ${styles[tone]}`}>{children}</span>;
}
