//! omacut - Video and audio trimming engine for SigmaOS

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct TimeCode {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
}

impl TimeCode {
    pub fn new(h: u32, m: u32, s: u32, ms: u32) -> Self {
        Self {
            hours: h,
            minutes: m,
            seconds: s,
            milliseconds: ms,
        }
    }

    pub fn to_milliseconds(&self) -> u64 {
        (self.hours as u64 * 3600 + self.minutes as u64 * 60 + self.seconds as u64) * 1000
            + self.milliseconds as u64
    }

    pub fn from_milliseconds(ms: u64) -> Self {
        let total_secs = ms / 1000;
        let rem_ms = (ms % 1000) as u32;
        let hours = (total_secs / 3600) as u32;
        let minutes = ((total_secs % 3600) / 60) as u32;
        let seconds = (total_secs % 60) as u32;
        Self::new(hours, minutes, seconds, rem_ms)
    }
}

#[derive(Debug, Clone)]
pub struct TrimJob {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub start: TimeCode,
    pub end: TimeCode,
}

impl TrimJob {
    pub fn new(input: &str, output: &str, start: TimeCode, end: TimeCode) -> Result<Self, String> {
        if input.is_empty() || output.is_empty() {
            return Err("Input and output paths cannot be empty".to_string());
        }
        if start.to_milliseconds() >= end.to_milliseconds() {
            return Err("Start time must be strictly before end time".to_string());
        }
        Ok(Self {
            input_path: PathBuf::from(input),
            output_path: PathBuf::from(output),
            start,
            end,
        })
    }

    pub fn duration_ms(&self) -> u64 {
        self.end
            .to_milliseconds()
            .saturating_sub(self.start.to_milliseconds())
    }
}

pub struct TrimQueue {
    jobs: Vec<TrimJob>,
}

impl TrimQueue {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, job: TrimJob) {
        self.jobs.push(job);
    }

    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }

    pub fn total_duration_ms(&self) -> u64 {
        self.jobs.iter().map(|j| j.duration_ms()).sum()
    }
}

impl Default for TrimQueue {
    fn default() -> Self {
        Self::new()
    }
}
