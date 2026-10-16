// Replays the golden session through the wasm build in Node and demands the same views the
// native kernel recorded. Run `scripts/build-wasm.sh` first (make bridge does both).
import { test } from "node:test";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";

const require = createRequire(import.meta.url);
const { Shell } = require("../../target/wasm-node/zidara_wasm.js");
const session = JSON.parse(
  readFileSync(new URL("../../crates/zidara-wasm/tests/golden/session.json", import.meta.url), "utf8"),
);

test("the wasm kernel reaches the same views as the native kernel", () => {
  const shell = new Shell(JSON.stringify(session.setup));
  session.steps.forEach((step, i) => {
    const result = JSON.parse(shell.run(JSON.stringify(step.actor), JSON.stringify(step.envelope)));
    assert.equal(result.status, "applied", `step ${i}`);
    assert.equal(result.events.length, step.events, `step ${i} event count`);
    const view = JSON.parse(shell.learnerView(session.learner, "ar", step.envelope.at));
    assert.deepEqual(view, session.views_ar[i], `step ${i} view`);
  });
  const { now, roster, view } = session.studio_fr;
  assert.deepEqual(JSON.parse(shell.studioView(JSON.stringify(roster), "fr", now)), view);
});

test("a replayed envelope is a no-op and bad input throws", () => {
  const shell = new Shell(JSON.stringify(session.setup));
  const [first] = session.steps;
  shell.run(JSON.stringify(first.actor), JSON.stringify(first.envelope));
  const again = JSON.parse(shell.run(JSON.stringify(first.actor), JSON.stringify(first.envelope)));
  assert.deepEqual(again, { status: "duplicate" });
  assert.throws(() => new Shell("{"), /setup: /);
  assert.throws(() => shell.learnerView("nope", "ar", 0), /user: /);
});
