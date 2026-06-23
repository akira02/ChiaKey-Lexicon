# ChiaKey Supplemental Symbol List

This source contains project-owned supplemental symbols for the Smart Mandarin
punctuation list.

Yahoo KeyKey's original `bpmf-punctuations.cin` remains the compatibility base.
This overlay only appends missing symbols to `_punctuation_list`; it does not
change direct punctuation key mappings such as `_punctuation_<`.

`symbols.tsv` format:

```text
symbol<TAB>tags
```

The release builder writes every accepted symbol as:

```text
_punctuation_list<TAB>symbol
```

Existing Yahoo punctuation-list symbols are skipped so their ordering is not
changed.

License: CC0-1.0.
