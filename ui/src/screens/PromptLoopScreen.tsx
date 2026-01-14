import { useParams } from "react-router-dom";
import { Card } from "../ui/Card";
import { Pill } from "../ui/Pill";
import { Button } from "../ui/Button";
import styles from "./PromptLoopScreen.module.css";

export function PromptLoopScreen() {
  const { loopId } = useParams();
  const id = loopId ?? "LOOP-456";

  return (
    <div className={styles.grid}>
      <div className={styles.left}>
        <header className={styles.header}>
          <h1>Prompt Loop</h1>
          <p>Governance-aware console with preflight checks and decision gates.</p>
        </header>

        <Card title="Run Thread" right={<Pill>{id}</Pill>}>
          <div className={styles.thread}>
            <div className={styles.bubble}>
              <div className={styles.meta}>Agent</div>
              <div className={styles.strong}>Agent Smith</div>
            </div>
            <div className={styles.bubble}>
              <div className={styles.meta}>Assistant output</div>
              <div className={styles.muted}>Proposed next action requires compliance evidence.</div>
            </div>
            <div className={styles.bubble}>
              <div className={styles.meta}>User</div>
              <div>Proceed with the remediation plan.</div>
            </div>
          </div>
        </Card>

        <Card title="Propose Action">
          <textarea className={styles.textarea} placeholder="Describe the next action…" />
          <div className={styles.actions}>
            <div className={styles.hint}>Preflight validates evidence + authority gates.</div>
            <Button>Submit for Preflight</Button>
          </div>
        </Card>
      </div>

      <div className={styles.right}>
        <Card title="Protocol Step" right={<Pill tone="warning">Gate ahead</Pill>}>
          <div className={styles.block}>
            <div className={styles.strong}>Compliance Review</div>
            <div className={styles.muted}>Requires Legal Counsel + DOC-01234</div>
          </div>
        </Card>

        <Card title="Required Evidence">
          <div className={styles.list}>
            <div className={styles.row}>
              <span>DOC-01234</span>
              <Pill tone="warning">Missing</Pill>
            </div>
            <div className={styles.row}>
              <span>Supporting log excerpt</span>
              <Pill tone="success">Attached</Pill>
            </div>
          </div>
        </Card>

        <Card title="Next Decision Gate">
          <div className={styles.gate}>
            <div className={styles.strong}>Decision Gate: Compliance Review</div>
            <div className={styles.muted}>
              This action cannot execute until evidence is complete and counsel approves.
            </div>
            <div className={styles.gateActions}>
              <Button variant="secondary">View Gate</Button>
              <Button>Request Approval</Button>
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}
