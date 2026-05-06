# Helix Dev Doc Deck Walkthrough

This walk-through keeps the domain vocabulary close to the data instead of burying it in prose.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 177 | ship |
| stress | diagnostic quality | 185 | ship |
| edge | review cost | 133 | watch |
| recovery | safe rewrite | 221 | ship |
| stale | change width | 124 | watch |

Start with `recovery` and `stale`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

`recovery` is the optimistic case; use it to make sure the scoring path still rewards strong signal.
