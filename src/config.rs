use crate::cli::Args;

pub struct AppConfig {
    pub log_files: Vec<String>,
    pub disable_preset_excludes: bool,
    pub exclude_words: Option<Vec<String>>,
    pub include_words: Option<Vec<String>>,
    pub debug: bool,
    pub use_cat_mode: bool,
}

impl From<Args> for AppConfig {
    fn from(args: Args) -> Self {
        Self {
            log_files: args.log_files,
            disable_preset_excludes: args.disable_preset_excludes,
            exclude_words: args.exclude_words,
            include_words: args.include_words,
            debug: args.debug,
            use_cat_mode: args.cat,
        }
    }
}
