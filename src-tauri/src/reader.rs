use crate::db::model::Novel;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub title: String,
    pub start_line: usize,
}

pub struct NovelReader {
    pub novel: Novel,
    pub chapters: Vec<Chapter>,
    pub line_num: usize,
    lines: Vec<String>,
}

impl NovelReader {
    pub fn new(novel: Novel) -> Result<Self, String> {
        let file = File::open(&novel.path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut chapters = Vec::new();

        let chapter_re = Regex::new(r"^(第[零一二三四五六七八九十百千万1-9]+章).*").unwrap();

        for (line_num, line) in reader.lines().enumerate() {
            let line_string = line.unwrap();

            chapter_re.captures(&line_string).map(|caps| {
                let chapter = Chapter {
                    title: caps[1].to_string(),
                    start_line: line_num,
                };

                chapters.push(chapter);
            });

            lines.push(line_string);
        }

        // 添加安全检查
        if novel.last_read_position > usize::MAX as u64 {
            return Err("Position value too large for this platform".to_string());
        }

        Ok(Self {
            chapters,
            line_num: novel.last_read_position as usize,
            novel,
            lines,
        })
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
}
