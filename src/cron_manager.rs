use core::fmt;
use std::{
    io::{BufRead, BufReader, Cursor, Write},
    process::Command,
};

use tempfile::NamedTempFile;

#[derive(Debug, Clone)]
pub struct CronJob {
    pub schedule: String,
    pub command: String,
    pub comment: Option<String>,
}

impl CronJob {
    fn new(schedule: &str, command: &str, comment: Option<&str>) -> Self {
        CronJob {
            schedule: schedule.to_string(),
            command: command.to_string(),
            comment: comment.map(|s| s.to_string()),
        }
    }
    fn to_string(&self) -> String {
        match &self.comment {
            Some(comment) => format!("{} {} # {}", self.schedule, self.command, comment),
            None => format!("{} {}", self.schedule, self.command),
        }
    }
}

impl fmt::Display for CronJob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.comment {
            Some(comment) => write!(f, "{} {} # {}", self.schedule, self.command, comment),
            None => write!(f, "{} {}", self.schedule, self.command),
        }
    }
}

pub struct CronManager {
    jobs: Vec<CronJob>,
}
impl CronManager {
    pub fn new() -> Self {
        let jobs = CronManager::load_crontab().unwrap_or_default();
        CronManager { jobs }
    }
    ///Load crontab for current user
    fn load_crontab() -> Result<Vec<CronJob>, std::io::Error> {
        let output = Command::new("crontab")
            .arg("-l")
            .output()
            .expect("should have output of user crontab");
        if output.status.success() {
            //take a slice of the entire vector because BufReader requires something that implements
            //the Read trait
            let cursor = Cursor::new(&output.stdout);
            let reader = BufReader::new(cursor);
            let mut jobs = Vec::new();
            for line in reader.lines() {
                let line = line?;
                if line.trim().is_empty() || line.starts_with('#') {
                    continue;
                }
                let (line, comment) = if let Some((code, comment)) = line.split_once('#') {
                    (code.trim(), Some(comment.trim().to_string()))
                } else {
                    (line.trim(), None)
                };

                let mut parts = line.split_whitespace();
                // take the schedule parts so all that is left is the command
                let schedule: String = parts.by_ref().take(5).collect::<Vec<&str>>().join(" ");
                let command: String = parts.collect::<Vec<&str>>().join(" ");
                jobs.push(CronJob::new(&schedule, &command, comment.as_deref()));
            }
            Ok(jobs)
        } else {
            Ok(Vec::new())
        }
    }

    ///saves crontab by creating a temporary file then calling `crontab <file_name>` which
    ///essentially overwrites the crontab
    fn save_crontab(&self) -> Result<(), std::io::Error> {
        let mut temp_file = NamedTempFile::new()?;
        for job in &self.jobs {
            writeln!(temp_file, "{}", job.to_string())?;
        }
        temp_file.flush()?;
        Command::new("crontab").arg(temp_file.path()).output()?;
        Ok(())
    }

    ///adds job and saves the crontab
    pub fn add_job(&mut self, job: CronJob) {
        self.jobs.push(job);
        self.save_crontab().unwrap();
    }

    pub fn list_jobs(&self) -> &Vec<CronJob> {
        &self.jobs
    }

    ///removes job at specific index and saves the crontab
    pub fn remove_job(&mut self, index: usize) {
        if index < self.jobs.len() {
            self.jobs.remove(index);
            self.save_crontab().unwrap();
        }
    }

    ///removes job based on comment
    ///
    ///Iterates over the jobs and creates a list of indexes  where the comment for the job matches
    ///the comment passed as a parameter.
    pub fn remove_job_by_comment(&mut self, comment: &str) {
        let mut to_remove = Vec::new();

        for (i, job) in self.jobs.iter().enumerate() {
            match &job.comment {
                Some(comm) => {
                    if *comm == comment.to_string() {
                        to_remove.push(i)
                    } else {
                        continue;
                    }
                }
                None => continue,
            }
        }
        // removing items may shift indexes, for example removing position 1 shifts pos 2 to 1 and
        // the indexes would be incorrect. If done in reverse, only the indexes that don't need to
        // be visited again will shift
        for index in to_remove.into_iter().rev() {
            self.jobs.remove(index);
        }
        self.save_crontab().unwrap();
    }
}
