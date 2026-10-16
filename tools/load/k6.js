// Load on the two hot paths, against a release server pinned to 2 CPU cores (tools/load/run.sh):
// learners reading their view, and learners posting work (open answers, which also wake the
// assessment agent). Thresholds are the gate; the summary is recorded in docs/JOURNAL.md.
import http from "k6/http";
import { check } from "k6";
import crypto from "k6/crypto";

const BASE = __ENV.BASE || "http://127.0.0.1:18090";
const uuidOf = (text) => {
  const h = crypto.md5(text, "hex");
  return `${h.slice(0, 8)}-${h.slice(8, 12)}-${h.slice(12, 16)}-${h.slice(16, 20)}-${h.slice(20)}`;
};
// The school tools/load/run.sh seeds: 10 classes of 30 learners.
const PEOPLE = [];
for (let c = 1; c <= 10; c++) {
  for (let i = 1; i <= 30; i++) PEOPLE.push({ cohort: uuidOf(`load-class-${c}`), user: uuidOf(`load-learner-${c}-${i}`) });
}

export const options = {
  scenarios: {
    // Open-loop: a school of 300 learners each refreshing every 2 s, whatever the server does.
    // (A closed loop of N users only measures saturation; see capacity.js.)
    read: { executor: "constant-arrival-rate", rate: 150, timeUnit: "1s", duration: "30s", preAllocatedVUs: 60, exec: "read" },
    write: { executor: "constant-arrival-rate", rate: 50, timeUnit: "1s", duration: "30s", preAllocatedVUs: 20, exec: "write" },
  },
  thresholds: {
    "http_req_duration{scenario:read}": ["p(95)<200"],
    "http_req_duration{scenario:write}": ["p(95)<300"],
    http_req_failed: ["rate<0.01"],
    // A gate must have data: an empty scenario once "passed" with every write throwing.
    "http_reqs{scenario:write}": ["count>1000"],
    "http_reqs{scenario:read}": ["count>3000"],
  },
};

export function setup() {
  return PEOPLE.map(({ cohort, user }) => ({
    cohort,
    token: http.post(`${BASE}/api/login`, JSON.stringify({ user_id: user }), { headers: { "content-type": "application/json" } }).json("token"),
  }));
}

// A random v4 UUID (this k6 has no crypto.randomUUID): envelope ids only need to be unique.
const uuid = () =>
  "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, (c) => {
    const r = (Math.random() * 16) | 0;
    return (c === "x" ? r : (r & 0x3) | 0x8).toString(16);
  });

const anyone = (people) => people[Math.floor(Math.random() * people.length)];

const auth = (token) => ({ headers: { authorization: `Bearer ${token}`, "content-type": "application/json" } });

export function read(people) {
  const { cohort, token } = anyone(people);
  const res = http.get(`${BASE}/api/cohorts/${cohort}/learner-view?locale=ar`, auth(token));
  check(res, { "view 200": (r) => r.status === 200 });
}

export function write(people) {
  const { cohort, token } = anyone(people);
  const envelope = { id: uuid(), at: Date.now(), command: { type: "submit_answer", activity: "explain-f2", answer: "نضرب البسط والمقام في نفس العدد" } };
  const res = http.post(`${BASE}/api/cohorts/${cohort}/commands`, JSON.stringify([envelope]), auth(token));
  check(res, { "command applied": (r) => r.status === 200 && r.json("0.outcome.status") === "applied" });
}
