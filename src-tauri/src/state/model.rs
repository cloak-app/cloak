use crate::utils::{reader::NovelReader, update::UpdateChecker};

pub struct AppState {
    pub novel_reader: Option<NovelReader>,
    pub reading_mode: bool,
    pub update_checker: UpdateChecker,
}
