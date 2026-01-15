import { Card } from "../ui/Card";
import { Pill } from "../ui/Pill";
import styles from "./OverviewScreen.module.css";

export function OverviewScreen() {
  return (
    <div className={styles.wrap}>
      <header className={styles.header}>
        <h1>Overview</h1>
        <p>Governance posture across agents, workflows, and decision gates.</p>
      </header>

      <section className={styles.metrics}>
        <Card title="Active Loops" right={<Pill>Live</Pill>}>
          <div className={styles.big}>4,025</div>
          <div className={styles.sub}>Running across environments</div>
        </Card>

        <Card title="Pending Approvals" right={<Pill tone="warning">Action</Pill>}>
          <div className={styles.big}>16</div>
          <div className={styles.sub}>Awaiting human authority</div>
        </Card>

        <Card title="Exceptions" right={<Pill tone="danger">Review</Pill>}>
          <div className={styles.big}>8</div>
          <div className={styles.sub}>Scoped deviations opened</div>
        </Card>
      </section>

      <section className={styles.grid2}>
        <Card title="Governance Alerts">
          <div className={styles.list}>
            <div className={styles.row}>
              <span>Protocol drift detected</span>
              <Pill tone="warning">Investigate</Pill>
            </div>
            <div className={styles.row}>
              <span>Document outdated: "SR-CONTRACT"</span>
              <Pill tone="warning">Review</Pill>
            </div>
            <div className={styles.row}>
              <span>Missing artifacts for "Compliance Review"</span>
              <Pill tone="danger">Blocked</Pill>
            </div>
          </div>
        </Card>

        <Card title="System Health">
          <div className={styles.list}>
            <div className={styles.row}>
              <span>API</span>
              <Pill tone="success">Online</Pill>
            </div>
            <div className={styles.row}>
              <span>Workers</span>
              <Pill tone="warning">Multiple errors</Pill>
            </div>
            <div className={styles.row}>
              <span>Event ingestion</span>
              <Pill tone="danger">Paused</Pill>
            </div>
          </div>
        </Card>
      </section>

      <Card title="Recent Activity">
        <div className={styles.tableWrap}>
          <table className={styles.table}>
            <thead>
              <tr>
                <th>Event</th><th>Actor</th><th>When</th>
              </tr>
            </thead>
            <tbody>
              <tr><td>Policy updated</td><td>Admin</td><td>2h ago</td></tr>
              <tr><td>Exception escalated</td><td>Agent Smith</td><td>1h ago</td></tr>
              <tr><td>Approval granted</td><td>Legal Counsel</td><td>45m ago</td></tr>
            </tbody>
          </table>
        </div>
      </Card>
    </div>
  );
}
