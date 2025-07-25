use std::{
    fs::File,
    io::{BufReader, Read},
};

use charset_normalizer_rs::{from_bytes, utils::decode};
use encoding::types::DecoderTrap;
use epub::doc::EpubDoc;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub index: usize,
    pub title: String,
    pub start_line: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NovelReader {
    pub novel_id: i64,
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
        state.serialize_field("novel_path", &self.novel_path)?;
        state.serialize_field("chapters", &self.chapters)?;
        state.serialize_field("read_position", &self.read_position)?;
        state.serialize_field("total_lines", &self.lines.len())?;
        state.serialize_field("current_chapter", &self.current_chapter())?;
        state.serialize_field("read_progress", &self.read_progress())?;
        state.end()
    }
}

impl NovelReader {
    pub fn new(
        novel_id: i64,
        novel_path: String,
        read_position: usize,
        line_size: usize,
    ) -> Result<Self, String> {
        let (lines, chapters) = Self::read_lines(&novel_path, line_size)?;

        Ok(Self {
            novel_id,
            novel_path,
            chapters,
            read_position,
            lines,
        })
    }

    pub fn read_lines(path: &str, line_size: usize) -> Result<(Vec<String>, Vec<Chapter>), String> {
        let extension = path.split('.').last().unwrap_or_default();

        let lines = match extension {
            "txt" => TxtReader::read_lines(path, line_size)?,
            "epub" => EpubReader::read_lines(path, line_size)?,
            _ => return Err("不支持的文件类型".to_string()),
        };

        let chapters = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.is_chapter)
            .map(|(index, line)| Chapter {
                index,
                title: line.content.clone(),
                start_line: index,
            })
            .collect();

        let lines = lines.iter().map(|line| line.content.clone()).collect();

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
            return Err("行号超出范围".to_string());
        }

        self.read_position = read_position;

        Ok(())
    }

    pub fn next_line(&mut self) -> Result<(), String> {
        if self.read_position >= self.lines.len() {
            return Err("行号超出范围".to_string());
        }

        self.read_position += 1;

        Ok(())
    }

    pub fn prev_line(&mut self) -> Result<(), String> {
        if self.read_position == 0 {
            return Err("行号超出范围".to_string());
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
                return Err("没有下一章节".to_string());
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
                return Err("没有上一章节".to_string());
            }

            let prev_chapter = &self.chapters[prev_chapter_index - 1];
            self.read_position = prev_chapter.start_line;
        }

        Ok(())
    }
}

struct Line {
    pub is_chapter: bool,
    pub content: String,
}

trait FileReader {
    fn read_lines(path: &str, line_size: usize) -> Result<Vec<Line>, String>;
}

struct TxtReader;

impl FileReader for TxtReader {
    fn read_lines(path: &str, line_size: usize) -> Result<Vec<Line>, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mut buf_reader = BufReader::new(file);
        let mut buffer = Vec::new();

        buf_reader
            .read_to_end(&mut buffer)
            .map_err(|e| e.to_string())?;

        let result = from_bytes(&buffer, None);
        let encoding = result.get_best().ok_or("无法检测文件编码")?.encoding();

        let decoded_string = decode(&buffer, encoding, DecoderTrap::Replace, false, false)?;

        let mut lines: Vec<Line> = Vec::new();

        let chapter_regex = Regex::new(r"^(第[零一二三四五六七八九十百千万1-9]+章.*)$").unwrap();

        for line in decoded_string.lines() {
            let line = line.trim();
            // 如果是章节，直接添加行
            let captures = chapter_regex.captures(line);

            // 是章节
            if let Some(captures) = captures {
                lines.push(Line {
                    is_chapter: true,
                    content: captures[1].to_string(),
                });
            } else {
                line.chars()
                    .collect::<Vec<_>>()
                    .chunks(line_size)
                    .for_each(|chunk| {
                        lines.push(Line {
                            is_chapter: false,
                            content: chunk.iter().collect(),
                        });
                    });
            }
        }

        Ok(lines)
    }
}

struct EpubReader;

impl FileReader for EpubReader {
    fn read_lines(path: &str, line_size: usize) -> Result<Vec<Line>, String> {
        let mut doc = EpubDoc::new(path).map_err(|e| e.to_string())?;

        let mut lines: Vec<Line> = Vec::new();

        let spine_idrefs: Vec<String> = doc.spine.iter().map(|spine| spine.idref.clone()).collect();

        for spine_idref in spine_idrefs {
            if let Some(page) = doc.resources.get(&spine_idref) {
                let page_path = page.0.clone();
                let page_content = doc.get_resource_str_by_path(page_path).unwrap();
                let page_html = Html::parse_document(page_content.as_str());

                let chapter_title_selector = Selector::parse("head > title").unwrap();

                let chapter_title = page_html
                    .select(&chapter_title_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()
                    .join("");

                lines.push(Line {
                    is_chapter: true,
                    content: chapter_title,
                });

                let line_selector = Selector::parse("p").unwrap();

                for line in page_html.select(&line_selector) {
                    let line_string = line.text().collect::<Vec<_>>().join("");
                    let line_str = line_string.trim();

                    line_str
                        .chars()
                        .collect::<Vec<_>>()
                        .chunks(line_size)
                        .for_each(|chunk| {
                            lines.push(Line {
                                is_chapter: false,
                                content: chunk.iter().collect(),
                            });
                        });
                }
            }
        }

        Ok(lines)
    }
}
