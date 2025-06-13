use crate::utils::reader::NovelReader;

pub struct AppState {
    pub novel_reader: Option<NovelReader>,
    pub reading_mode: bool,
}
