# Review Journal

The review surface for `helix-dev-doc-deck` is deliberately narrow: one fixture, one scoring rule, and one local check.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 177, lane `ship`
- `stress`: `diagnostic quality`, score 185, lane `ship`
- `edge`: `review cost`, score 133, lane `watch`
- `recovery`: `safe rewrite`, score 221, lane `ship`
- `stale`: `change width`, score 124, lane `watch`

## Note

A future change should add new cases before it changes the scoring rule.
