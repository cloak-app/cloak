use crate::db::model::Novel;
use regex::Regex;
use serde::{Deserialize, Serialize, Serializer};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub title: String,
    pub start_line: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NovelReader {
    pub novel: Novel,
    pub chapters: Vec<Chapter>,
    pub line_num: usize,
    lines: Vec<String>,
}

impl Serialize for NovelReader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("NovelReader", 4)?;
        state.serialize_field("novel", &self.novel)?;
        state.serialize_field("chapters", &self.chapters)?;
        state.serialize_field("line_num", &self.line_num)?;
        state.serialize_field("current_chapter", &self.current_chapter())?;
        state.end()
    }
}

impl NovelReader {
    pub fn new(novel: Novel) -> Result<Self, String> {
        let file = File::open(&novel.path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut chapters = Vec::new();

        let chapter_re = Regex::new(r"^(第[零一二三四五六七八九十百千万1-9]+章.*)$")
            .map_err(|e| e.to_string())?;

        for (start_line, line) in reader.lines().enumerate() {
            let line_string = line.map_err(|e| e.to_string())?;

            chapter_re.captures(&line_string).map(|caps| {
                let chapter = Chapter {
                    title: caps[1].to_string(),
                    start_line,
                };

                chapters.push(chapter);
            });

            lines.push(line_string);
        }

        let line_num = novel.last_read_position as usize;

        Ok(Self {
            chapters,
            line_num,
            novel,
            lines,
        })
    }

    pub fn current_chapter(&self) -> Chapter {
        let mut current_chapter = None::<Chapter>;

        for chapter in self.chapters.iter() {
            if chapter.start_line <= self.line_num {
                current_chapter = Some(chapter.clone());
            }
        }

        current_chapter.unwrap()
    }

    pub fn get_line(&self) -> Option<&str> {
        if self.line_num >= self.lines.len() {
            return None;
        }

        let line = &self.lines[self.line_num];

        Some(line)
    }

    pub fn set_line_num(&mut self, line_num: usize) -> Result<(), String> {
        if line_num >= self.lines.len() {
            return Err("Line number out of bounds".to_string());
        }

        self.line_num = line_num;

        Ok(())
    }

    pub fn next_line(&mut self) -> Result<(), String> {
        if self.line_num >= self.lines.len() {
            return Err("Line number out of bounds".to_string());
        }

        self.line_num += 1;

        Ok(())
    }

    pub fn prev_line(&mut self) -> Result<(), String> {
        if self.line_num == 0 {
            return Err("Line number out of bounds".to_string());
        }

        self.line_num -= 1;

        Ok(())
    }

    pub fn next_chapter(&mut self) -> Result<(), String> {
        let current_chapter = self.current_chapter();

        let next_chapter_index = self
            .chapters
            .iter()
            .position(|chapter| chapter.start_line == current_chapter.start_line);

        if let Some(next_chapter_index) = next_chapter_index {
            if next_chapter_index + 1 >= self.chapters.len() {
                return Err("No next chapter".to_string());
            }

            let next_chapter = &self.chapters[next_chapter_index + 1];
            self.line_num = next_chapter.start_line;
        }

        Ok(())
    }

    pub fn prev_chapter(&mut self) -> Result<(), String> {
        let current_chapter = self.current_chapter();

        let prev_chapter_index = self
            .chapters
            .iter()
            .position(|chapter| chapter.start_line == current_chapter.start_line);

        if let Some(prev_chapter_index) = prev_chapter_index {
            if prev_chapter_index == 0 {
                return Err("No previous chapter".to_string());
            }

            let prev_chapter = &self.chapters[prev_chapter_index - 1];
            self.line_num = prev_chapter.start_line;
        }

        Ok(())
    }
}
