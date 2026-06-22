use crate::config::BPMF_EXT_SOURCE_ID;
use crate::phonetics::qstring_for_bpmf_sequence;
use crate::types::SourceRecord;
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const BASE_WEIGHT: f64 = -3.35;
const RANK_STEP: f64 = 0.000001;

pub fn parse_cin(
    path: &Path,
    existing_exact_keys: &HashSet<(String, String)>,
) -> Result<(Vec<SourceRecord>, usize, usize)> {
    let file = File::open(path).with_context(|| format!("read {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut keymap: HashMap<char, String> = HashMap::new();
    let mut ranks: HashMap<String, usize> = HashMap::new();
    let mut records = Vec::new();
    let mut seen = 0;
    let mut skipped = 0;
    let mut in_keyname = false;
    let mut in_chardef = false;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        match line {
            "%keyname  begin" | "%keyname begin" => {
                in_keyname = true;
                continue;
            }
            "%keyname  end" | "%keyname end" => {
                in_keyname = false;
                continue;
            }
            "%chardef  begin" | "%chardef begin" => {
                in_chardef = true;
                continue;
            }
            "%chardef  end" | "%chardef end" => break,
            _ => {}
        }

        if in_keyname {
            parse_keyname_line(line, &mut keymap);
            continue;
        }
        if !in_chardef {
            continue;
        }

        seen += 1;
        match parse_chardef_line(line, &keymap, &mut ranks, existing_exact_keys) {
            Some(record) => records.push(record),
            None => skipped += 1,
        }
    }

    Ok((records, seen, skipped))
}

fn parse_keyname_line(line: &str, keymap: &mut HashMap<char, String>) {
    let mut parts = line.split_whitespace();
    let Some(key) = parts.next().and_then(|text| text.chars().next()) else {
        return;
    };
    let Some(value) = parts.next() else {
        return;
    };
    keymap.insert(key, value.to_string());
}

fn parse_chardef_line(
    line: &str,
    keymap: &HashMap<char, String>,
    ranks: &mut HashMap<String, usize>,
    existing_exact_keys: &HashSet<(String, String)>,
) -> Option<SourceRecord> {
    let mut parts = line.split_whitespace();
    let key = parts.next()?;
    let phrase = parts.next()?.to_string();
    if phrase.chars().count() != 1 || !is_bmp_cjk(phrase.chars().next()?) {
        return None;
    }

    let bpmf = cin_key_to_bpmf(key, keymap)?;
    let (qstring, syllable_count) = qstring_for_bpmf_sequence(&bpmf)?;
    if syllable_count != 1 || existing_exact_keys.contains(&(qstring.clone(), phrase.clone())) {
        return None;
    }

    let rank = ranks.entry(qstring.clone()).or_default();
    let weight = BASE_WEIGHT - (*rank as f64 * RANK_STEP);
    *rank += 1;

    Some(SourceRecord {
        qstring,
        phrase,
        weight,
        source_id: BPMF_EXT_SOURCE_ID,
        tags: format!("unigram,{BPMF_EXT_SOURCE_ID},character-supplement"),
    })
}

fn cin_key_to_bpmf(key: &str, keymap: &HashMap<char, String>) -> Option<String> {
    let mut bpmf = String::new();
    for character in key.chars() {
        bpmf.push_str(keymap.get(&character)?);
    }
    Some(bpmf)
}

fn is_bmp_cjk(character: char) -> bool {
    let codepoint = character as u32;
    (0x3400..=0x4dbf).contains(&codepoint)
        || (0x4e00..=0x9fff).contains(&codepoint)
        || (0xf900..=0xfaff).contains(&codepoint)
}

#[cfg(test)]
mod tests {
    use super::is_bmp_cjk;

    #[test]
    fn keeps_bmp_cjk_but_excludes_non_bmp_and_private_use() {
        assert!(is_bmp_cjk('我'));
        assert!(is_bmp_cjk('䂺'));
        assert!(!is_bmp_cjk('𢬯'));
        assert!(!is_bmp_cjk('\u{e000}'));
    }
}
