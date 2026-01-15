import styles from "./Button.module.css";

export type ButtonVariant = "primary" | "secondary" | "ghost";

export function Button({
  variant = "primary",
  className,
  ...props
}: React.ButtonHTMLAttributes<HTMLButtonElement> & { variant?: ButtonVariant }) {
  return (
    <button
      {...props}
      className={[styles.btn, styles[variant], className].filter(Boolean).join(" ")}
    />
  );
}
