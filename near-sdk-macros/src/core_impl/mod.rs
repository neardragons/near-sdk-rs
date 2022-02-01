mod code_generator;
mod info_extractor;
mod metadata;
pub use code_generator::*;
pub use info_extractor::*;
pub use metadata::metadata_visitor::MetadataVisitor;


const WITGEN_ENABLED: &str = "WITGEN_ENABLED";

pub(crate) fn is_witgen() -> bool {
  std::env::var(WITGEN_ENABLED).map_or(false, |s|s == "true")
}