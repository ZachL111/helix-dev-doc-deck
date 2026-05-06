# helix-dev-doc-deck

`helix-dev-doc-deck` explores developer tools with a small Rust codebase and local fixtures. The technical goal is to build a Rust toolkit that studies doc behavior through safe and unsafe fixtures, with remediation hints and no network dependency.

## Why It Exists

The project exists to keep a narrow engineering decision visible and testable. For this repo, that decision is how change width and review cost should influence a review result.

## Helix Dev Doc Deck Review Notes

Start with `safe rewrite` and `change width`. Those cases create the widest score spread in this repo, so they are the best quick check when the model changes.

## Features

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/helix-dev-doc-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `safe rewrite` and `change width`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Architecture Notes

The fixture data drives the tests. The code stays thin, while `metadata/domain-review.json` and `config/review-profile.json` explain what each case is meant to protect.

The added Rust path is deliberately direct, with fixtures doing most of the explaining.

## Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Tests

The same command runs the local verification path. The highest-scoring domain case is `recovery` at 221, which lands in `ship`. The most cautious case is `stale` at 124, which lands in `watch`.

## Limitations And Roadmap

The fixture set is small enough to audit by hand. The next useful expansion is malformed input coverage, not extra surface area.
