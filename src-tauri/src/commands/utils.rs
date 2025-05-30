use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub start_position: usize,
    pub end_position: usize,
}

pub struct NovelReader {
    content: String,
    chapters: Vec<Chapter>,
    position: u64,
}

impl NovelReader {
    pub fn new(path: &str, last_read_position: u64) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let chapters = Self::parse_chapters(&content);
        Self {
            content,
            chapters,
            position: last_read_position,
        }
    }

    fn parse_chapters(content: &str) -> Vec<Chapter> {
        let mut chapters = Vec::new();
        // TODO: 实现章节解析逻辑
        // 这里只需要记录章节的起始和结束位置，不需要复制内容
        chapters
    }

    pub fn get_chapter_content(&self, chapter_index: usize) -> Option<&str> {
        self.chapters
            .get(chapter_index)
            .map(|chapter| &self.content[chapter.start_position..chapter.end_position])
    }
}
