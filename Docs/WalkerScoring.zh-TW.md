# Walker 計分算法與權重驗證

本文件說明 ChiaKey 輸入引擎（Manjusri / Gramambular 系）的 walker 如何用詞庫中的
unigram、bigram 機率計分，並據此整理出詞庫資料的權重驗證規則。

## 分數單位

所有機率皆為 **log10 機率**（負值，越接近 0 代表越可能）。

- 因為是 log 空間，分數相加 = 機率相乘；分數比大小 = 機率比大小。
- 找不到的詞用 log(0) 地板值 `-99.0`

## 計分流程

每個節點（節點 = 一段讀音對應的候選詞集合）在 walk 時，會帶著前一個詞 `previous`
呼叫 `findHighestScorePair(previous)`：

1. **override 優先**：若該節點被使用者覆寫選字，直接回傳 `c_defaultOverrideScore`，
   結束。
2. **查 bigram**：在 `m_bigramMap[previous]` 找此 context 的 bigram。資料已依機率
   排序，取 `at(0)`（最高者）作為 `bigramResult`。可能不存在。
3. **取 backoff weight**：查 `m_unigramPreviousBackoffs[previous]`，得到 `previous`
   這個 context 的 backoff weight（即 BOW(previous)）；查不到則用預設
   `c_defaultUNKBackoff`。
4. **算 unigram path**：取最高分 unigram `m_unigramCurrents[0]`，分數**加上** backoff
   weight：`result = unigram_logP + backoff(previous)`。若此節點完全沒有 unigram，
   則用 `backoff(previous) + c_defaultUNKProbability`。
5. **比大小取 max**：若 bigram 存在且 `bigramResult > result`，回傳 bigram；
   否則回傳 unigram path 的結果。

對應的部份原始碼：

```cpp
// Node.h:304-317
StringScorePair result;
if (m_unigramCurrents.size()) {
  result = m_unigramCurrents[0];
  result.second += backoffWeight;            // unigram + backoff
} else {
  result.first = "";
  result.second = backoffWeight + Node::c_defaultUNKProbability;
}

if (hasBigramResult)
  if (bigramResult.second > result.second)   // 取 max
    return bigramResult;
return result;
```

這是標準的 Katz back-off，bigram 直接用條件機率，unigram 退避時補上
context 的 backoff weight，兩條路徑取較高分者。

## 關於詞庫權重驗證

因為是「取 max」而非「取代」或「相加」，所以一筆 bigram 要真正生效，它的 log 機率
必須贏過對應的 unigram path。據此整理出以下可機械化檢查的規則。

### 1. 數值範圍

- unigram、bigram `probability` 必須為 log10 機率，皆 `<= 0`（地板約 `-99.0`）。
- backoff weight 通常 `<= 0`（log 空間），不應為過大正值。
- 偵測明顯單位錯誤：若出現 `0 < p <= 1`（疑似填了原始機率而非 log10），應警示。

### 2. bigram 有效性

一筆 bigram `(previous, current)` 只有在下式成立時才會被 walker 選中：

```
bigram_logP(current | previous)  >  unigram_logP(current) + backoff(previous)
```

驗證時可分級：

- **死權重（dead weight）**：`bigram_logP <= unigram_logP(current) + backoff(previous)`。
  這筆 bigram 永遠不會贏過 unigram，等於沒作用，應標記（可能是權重算錯或可刪）。
- **退化（degenerate）**：`bigram_logP <= unigram_logP(current)`（即使不算 backoff 也輸）。
  幾乎必為錯誤資料。
- backoff(previous) 取不到時，驗證需用預設 `c_defaultUNKBackoff` 代入，與 runtime 一致。

### 3. 排序前提

walker 取 `m_bigramMap[previous].at(0)` 與 `m_unigramCurrents[0]`，**假設資料已依
probability 由高到低排序**。詞庫 build 出的同一 `qstring`／同一 `previous` 群組，
必須維持 `probability DESC`；否則 walker 會誤取到非最高分者。

### 4. 一致性

- 每筆 bigram 的 `current` 應在對應 unigram 表中存在（否則 unigram path 缺基準，
  back-off 比較失真）。
- 同一 `(previous, current)` 不應有重複或矛盾的多筆 bigram。
