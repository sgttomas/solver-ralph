import { Card } from "../ui/Card";

export function PlaceholderScreen({ title }: { title: string }) {
  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "var(--space5)" }}>
      <header>
        <h1 style={{ margin: 0, fontSize: 26, letterSpacing: "-0.03em" }}>{title}</h1>
        <p style={{ margin: "6px 0 0", color: "var(--muted)", fontSize: 14 }}>
          This screen is stubbed. Drop your real UI here.
        </p>
      </header>
      <Card title="Coming soon">
        <div style={{ color: "var(--muted)", fontSize: 14 }}>
          When you connect your backend, this will become the {title} view.
        </div>
      </Card>
    </div>
  );
}
