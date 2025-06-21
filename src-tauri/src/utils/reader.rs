use regex::Regex;
use serde::{Deserialize, Serialize, Serializer};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub index: usize,
    pub title: String,
    pub start_line: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NovelReader {
    pub novel_id: i64,
    pub novel_title: String,
    pub novel_path: String,
    pub chapters: Vec<Chapter>,
    pub read_position: usize,
    pub lines: Vec<String>,
}

impl Serialize for NovelReader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("NovelReader", 7)?;
        state.serialize_field("novel_id", &self.novel_id)?;
        state.serialize_field("novel_title", &self.novel_title)?;
        state.serialize_field("novel_path", &self.novel_path)?;
        state.serialize_field("chapters", &self.chapters)?;
        state.serialize_field("read_position", &self.read_position)?;
        state.serialize_field("current_chapter", &self.current_chapter())?;
        state.serialize_field("read_progress", &self.read_progress())?;
        state.end()
    }
}

impl NovelReader {
    pub fn new(
        novel_id: i64,
        novel_title: String,
        novel_path: String,
        read_position: usize,
        line_size: usize,
    ) -> Result<Self, String> {
        let (lines, chapters) = Self::read_lines(&novel_path, line_size)?;

        Ok(Self {
            novel_id,
            novel_title,
            novel_path,
            chapters,
            read_position,
            lines,
        })
    }

    pub fn read_lines(path: &str, line_size: usize) -> Result<(Vec<String>, Vec<Chapter>), String> {
        let file = File::open(&path).map_err(|e| e.to_string())?;
        let buf_reader = BufReader::new(file);

        let mut lines = Vec::new();
        let mut chapters = Vec::new();

        let chapter_re = Regex::new(r"^(第[零一二三四五六七八九十百千万1-9]+章.*)$")
            .map_err(|e| e.to_string())?;

        for (_, line) in buf_reader.lines().enumerate() {
            let line_string = line.map_err(|e| e.to_string())?;

            // 如果是章节，直接添加行
            let caps = chapter_re.captures(&line_string);
            if let Some(caps) = caps {
                let chapter = Chapter {
                    index: chapters.len(),
                    title: caps[1].to_string(),
                    start_line: lines.len(),
                };

                chapters.push(chapter);
                lines.push(line_string);
            } else {
                let mut temp_line = String::new();

                for ele in line_string.chars() {
                    temp_line.push(ele);
                    if temp_line.chars().count() >= line_size {
                        lines.push(temp_line);
                        temp_line = String::new();
                    }
                }

                if temp_line.chars().count() > 0 {
                    lines.push(temp_line);
                }
            }
        }

        Ok((lines, chapters))
    }

    pub fn current_chapter(&self) -> &Chapter {
        let mut current_chapter = &self.chapters[0];

        for chapter in self.chapters.iter() {
            if chapter.start_line <= self.read_position {
                current_chapter = chapter;
            }
        }

        current_chapter
    }

    pub fn read_progress(&self) -> f64 {
        self.read_position as f64 / self.lines.len() as f64 * 100.0
    }

    pub fn get_line(&self) -> Option<&str> {
        if self.read_position >= self.lines.len() {
            return None;
        }

        let line = &self.lines[self.read_position];

        Some(line)
    }

    pub fn set_read_position(&mut self, read_position: usize) -> Result<(), String> {
        if read_position >= self.lines.len() {
            return Err("Line number out of bounds".to_string());
        }

        self.read_position = read_position;

        Ok(())
    }

    pub fn next_line(&mut self) -> Result<(), String> {
        if self.read_position >= self.lines.len() {
            return Err("Line number out of bounds".to_string());
        }

        self.read_position += 1;

        Ok(())
    }

    pub fn prev_line(&mut self) -> Result<(), String> {
        if self.read_position == 0 {
            return Err("Line number out of bounds".to_string());
        }

        self.read_position -= 1;

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
            self.read_position = next_chapter.start_line;
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
            self.read_position = prev_chapter.start_line;
        }

        Ok(())
    }
}
