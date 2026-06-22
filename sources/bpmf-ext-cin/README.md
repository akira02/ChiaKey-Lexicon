# Public Domain Extended BPMF Character Table

Source id: `bpmf-ext-cin`

This source vendors the public-domain `bpmf-ext.cin` character table from the KeyKey Boneyard tree:

```text
sources/bpmf-ext-cin/vendor/bpmf-ext.cin
```

The file header says it was revised from `opendesktop.org.tw`'s `phone.cin` to include CNS11643 and Unicode-compatible characters, with license marked as Public Domain.

The release builder uses this source only as a low-priority single-character reading supplement:

1. It imports CJK BMP characters only.
2. It excludes non-BMP and private-use characters.
3. It only adds missing exact `(reading, character)` pairs.
4. It does not override libchewing character frequencies.

This source fills character-level gaps such as the native/Yahoo `ㄨㄛˇ` candidate set:

```text
我 婐 捰 倭 䂺 婑 䰀 㦱
```

The source inventory is stored at:

```text
sources/bpmf-ext-cin/source-inventory.sha256
```
