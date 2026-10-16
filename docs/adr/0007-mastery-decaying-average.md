# 0007. Mastery is judged on a decaying average

Date: 2026-10-16. Status: accepted. Replaces "mean of the last three tries" (ADR-less M1 rule).

## Context
The G3 golden session showed a learner who scored 33%, 33%, then 100% after remediation. On
the mean of the last three tries that is 55.6%, so the child who had just understood still
read "needs support" and needed two more perfect tries to show mastery. That punishes the
recovery the remediation was meant to bring about.

## Decision
Mastery judges an outcome on a **decaying average**: each new try counts 65%, the running
average of everything before it 35%, in integer basis points, rounded half up at each step.
This is the default decaying average in standards-based gradebooks (Otus, Skyward), chosen
there because it rewards growth without letting one score decide alone. Thresholds are
unchanged: mastered at 80%, support below 60% after two tries.

## Consequences
- 33%, 33%, 100% → 76.7%: "in progress"; one more perfect try → 91.8%: mastered.
- A slip after mastery shows at once (100%, 0% → 35%), so the teacher sees it.
- Order matters by design: the same scores in reverse tell a falling story (100, 33, 33 → 41.5%).
- Sources: Otus, "Decaying Average Mastery Setting" (help.otus.com/en/articles/9174437);
  Skyward, "What is Decaying Average?" (skyward.com/blogs/ak12/2018/february). No independent
  empirical comparison of aggregation methods was found; the weight stays a setting.
