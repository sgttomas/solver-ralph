import { Card } from "../ui/Card";
import { Pill } from "../ui/Pill";
import { Button } from "../ui/Button";
import styles from "./ApprovalsScreen.module.css";

export function ApprovalsScreen() {
  return (
    <div className={styles.grid}>
      <div className={styles.left}>
        <header className={styles.header}>
          <h1>Approvals</h1>
          <p>Review packages, diffs, evidence, and impact.</p>
        </header>

        <Card title="Queue" right={<Pill tone="warning">16 pending</Pill>}>
          <div className={styles.queue}>
            {["Compliance Exception Review", "Protocol Update", "Document Revision"].map((t) => (
              <button key={t} className={styles.item}>
                <div className={styles.itemTop}>
                  <div className={styles.strong}>{t}</div>
                  <Pill tone="warning">Needs Counsel</Pill>
                </div>
                <div className={styles.meta}>AGNT-00042 / LOOP-456</div>
              </button>
            ))}
          </div>
        </Card>
      </div>

      <div className={styles.right}>
        <Card title="Approval Package" right={<Pill tone="warning">Env: Production</Pill>}>
          <div className={styles.pkg}>
            <div className={styles.block}>
              <div className={styles.meta}>Decision Gate</div>
              <div className={styles.strong}>Compliance Review</div>
              <div className={styles.muted}>Requested by Agent Smith • LOOP-456</div>
            </div>

            <div className={styles.block}>
              <div className={styles.meta}>Settings diff</div>
              <div className={styles.diff}>
                <div className={styles.diffCol}>
                  <div className={styles.meta}>Before</div>
                  <div>Domains: General</div>
                </div>
                <div className={styles.diffCol}>
                  <div className={styles.meta}>After</div>
                  <div>Domains: Finance</div>
                </div>
              </div>
            </div>

            <div className={styles.block}>
              <div className={styles.meta}>Impact preview</div>
              <ul className={styles.ul}>
                <li>Affects 5 agents</li>
                <li>12 loops currently active</li>
                <li>Requires updated DOC-01234 citation</li>
              </ul>
            </div>

            <div className={styles.block}>
              <div className={styles.meta}>Evidence</div>
              <div className={styles.row}>
                <span>DOC-01234</span>
                <Pill tone="warning">Outdated</Pill>
              </div>
              <div className={styles.row}>
                <span>Supporting log excerpt</span>
                <Pill tone="success">OK</Pill>
              </div>
            </div>

            <div className={styles.actions}>
              <Button>Approve</Button>
              <Button variant="secondary">Request Changes</Button>
              <Button variant="ghost">Escalate</Button>
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}
