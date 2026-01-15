import styles from "./Card.module.css";

export function Card(props: {
  title?: string;
  right?: React.ReactNode;
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <section className={[styles.card, props.className].filter(Boolean).join(" ")}>
      {(props.title || props.right) && (
        <div className={styles.header}>
          <h3 className={styles.title}>{props.title}</h3>
          <div className={styles.right}>{props.right}</div>
        </div>
      )}
      {props.children}
    </section>
  );
}
