use memmap2::Mmap;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::db::model::Novel;

struct Chapter {
    title: String,
    start_line: usize,
}

pub struct NovelReader {
    pub chapters: Vec<Chapter>,
    line_offsets: Vec<usize>, // 每行起始字节偏移量
    mmap: Mmap,
}

impl NovelReader {
    pub fn new(novel: Novel) -> Result<Self, String> {
        let file = File::open(novel.path).map_err(|e| e.to_string())?;
        let mmap = unsafe { Mmap::map(&file).map_err(|e| e.to_string()) }?;
        let mut reader = BufReader::new(file);
        let mut line_offsets = Vec::new();
        let mut current_offset = 0;

        // 预读取行偏移量
        let mut buffer = String::new();
        while reader.read_line(&mut buffer).map_err(|e| e.to_string())? > 0 {
            line_offsets.push(current_offset);
            current_offset += buffer.len();
            buffer.clear();
        }

        let chapter_re = Regex::new(r"^(第[零一二三四五六七八九十百千万]+章)").unwrap();

        let chapters: Vec<Chapter> = reader
            .lines()
            .enumerate()
            .filter_map(|(line_num, line)| {
                line.ok().and_then(|line_string| {
                    chapter_re.captures(&line_string).map(|caps| Chapter {
                        title: caps[1].to_string(),
                        start_line: line_num,
                    })
                })
            })
            .collect();

        Ok(Self {
            chapters,
            line_offsets,
            mmap,
        })
    }

    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        if line_num >= self.line_offsets.len() {
            return None;
        }

        let start = self.line_offsets[line_num] as usize;
        let end = if line_num + 1 < self.line_offsets.len() {
            self.line_offsets[line_num + 1] as usize
        } else {
            self.mmap.len()
        };

        std::str::from_utf8(&self.mmap[start..end]).ok()
    }
}
