use crate::config::{
    Config, BONEYARD_SOURCE_ID, BPMF_EXT_SOURCE_ID, BPMF_EXT_VENDOR_PATH, LIBCHEWING_SOURCE_ID,
    OVERLAY_SOURCE_ID, RIME_ESSAY_SOURCE_ID,
};
use std::path::PathBuf;

pub struct ReleasePaths {
    pub boneyard_source_dir: PathBuf,
    pub bpmf_ext_source_dir: PathBuf,
    pub libchewing_source_dir: PathBuf,
    pub rime_essay_source_dir: PathBuf,
    pub overlay_source_dir: PathBuf,
    pub overlay_phrases: PathBuf,
    pub boneyard_inventory: PathBuf,
    pub bpmf_ext_inventory: PathBuf,
    pub bpmf_ext_cin: PathBuf,
    pub libchewing_inventory: PathBuf,
    pub rime_essay_inventory: PathBuf,
    pub rime_essay_raw: PathBuf,
    pub db_filename: String,
    pub metadata_filename: String,
    pub db: PathBuf,
    pub metadata: PathBuf,
    pub checksum: PathBuf,
    pub dist_manifest: PathBuf,
}

impl ReleasePaths {
    pub fn new(cfg: &Config) -> Self {
        let boneyard_source_dir = cfg.root.join("sources").join(BONEYARD_SOURCE_ID);
        let bpmf_ext_source_dir = cfg.root.join("sources").join(BPMF_EXT_SOURCE_ID);
        let libchewing_source_dir = cfg.root.join("sources").join(LIBCHEWING_SOURCE_ID);
        let rime_essay_source_dir = cfg.root.join("sources").join(RIME_ESSAY_SOURCE_ID);
        let overlay_source_dir = cfg.root.join("sources").join(OVERLAY_SOURCE_ID);
        let db_filename = format!("KeyKeySource-{}.db", cfg.release_version);
        let metadata_filename = format!("KeyKeySource-{}.json", cfg.release_version);

        Self {
            overlay_phrases: overlay_source_dir.join("phrases.tsv"),
            boneyard_inventory: boneyard_source_dir.join("source-inventory.sha256"),
            bpmf_ext_inventory: bpmf_ext_source_dir.join("source-inventory.sha256"),
            bpmf_ext_cin: cfg.root.join(BPMF_EXT_VENDOR_PATH),
            libchewing_inventory: libchewing_source_dir.join("source-inventory.sha256"),
            rime_essay_inventory: rime_essay_source_dir.join("source-inventory.sha256"),
            rime_essay_raw: rime_essay_source_dir.join("raw/essay.txt"),
            db: cfg.dist_dir.join(&db_filename),
            metadata: cfg.dist_dir.join(&metadata_filename),
            checksum: cfg.dist_dir.join("SHA256SUMS"),
            dist_manifest: cfg.dist_dir.join("lexicon-manifest.json"),
            boneyard_source_dir,
            bpmf_ext_source_dir,
            libchewing_source_dir,
            rime_essay_source_dir,
            overlay_source_dir,
            db_filename,
            metadata_filename,
        }
    }
}
