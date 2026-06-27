# ChiaKey Auto Hotwords Overlay

This source contains a small, automatically refreshed hotwords overlay.

Google Trends is used only as a discovery signal. Daily collection queries the
24-hour, 48-hour, and 7-day trending windows, then stores minimal normalized
observations as GitHub Actions artifacts. The weekly refresh job aggregates
those observations and writes only the resulting ChiaKey-owned overlay rows into
this directory.

`phrases.tsv` format:

```text
phrase<TAB>weight<TAB>tags
```

The release builder infers qstrings from existing single-character readings.
Rows that cannot be inferred are skipped during release preparation.

Automation policy:

- Keep only Han-only terms after normalization.
- Drop ASCII letters and digits.
- Keep only 2-4 codepoint terms.
- Drop query-like terms such as weather, stock-price, target-price, ranking, and
  index queries.
- Drop terms already present in the base lexicon.
- Drop terms that are already typeable as top-ranked existing segments.
- Keep 7-day-only terms as weak signal; they do not enter the overlay on their
  own.
- Retain active terms for up to 30 days after last observation.

Weights are intentionally conservative:

- `-2.4` for terms that pass the multi-window signal threshold.
- `-2.1` after stronger repeated signal in the last 14 days.
- `-1.9` after sustained repeated signal in the last 30 days.
- `-2.6` while decaying after 14 days without observation.

Signal scoring:

- `24h`: 1 point
- `48h`: 2 points
- `7d`: 3 points

A term is emitted only when it has enough short-window corroboration, such as
appearing in both `24h` and `48h`, appearing in a short window plus `7d`, or
appearing across multiple collection days. A single `7d` observation is treated
as background context rather than a reason to add a word.

This layer is expected to change over time and should not be treated as a
manual review source.
