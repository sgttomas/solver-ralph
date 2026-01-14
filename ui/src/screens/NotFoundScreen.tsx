import { Link } from "react-router-dom";
import { Card } from "../ui/Card";
import { Button } from "../ui/Button";

export function NotFoundScreen() {
  return (
    <Card title="Not found">
      <p style={{ marginTop: 0, color: "var(--muted)" }}>
        That route doesn't exist.
      </p>
      <Link to="/overview">
        <Button>Back to Overview</Button>
      </Link>
    </Card>
  );
}
