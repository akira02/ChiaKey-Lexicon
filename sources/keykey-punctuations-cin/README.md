# KeyKey BPMF Punctuation Table

Source id: `keykey-punctuations-cin`

This source vendors the original KeyKey BPMF punctuation CIN table:

```text
sources/keykey-punctuations-cin/vendor/bpmf-punctuations.cin
```

The upstream source path is:

```text
YahooKeyKey-Source-1.1.2528/DataTables/bpmf-punctuations.cin
```

The release builder imports only rows inside `%chardef` whose keys start with `_punctuation_` or `_ctrl_`. These rows are required by Smart Mandarin runtime punctuation lookup, for example Shift+, resolves `_punctuation_<` and expects `，`.

The source inventory is stored at:

```text
sources/keykey-punctuations-cin/source-inventory.sha256
```
