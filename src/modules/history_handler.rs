use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Error, ErrorKind, Read, Write},
    path::Path,
    time::{Duration, SystemTime},
};

use crate::shell::core::ShellHistoryHandler;

pub struct HistoryHandler {
    current_index: usize,
    history: Vec<HistoryEntry>,
    path: std::path::PathBuf,
}

impl ShellHistoryHandler for HistoryHandler {
    fn new() -> Self
    where
        Self: Sized,
    {
        let env_dir = std::env::var("RSHELL_HOME");

        if env_dir.is_err() {
            let home_dir = std::env::var("HOME").unwrap_or_else(|err| {
                panic!("Failed to get HOME directory: {}", err);
            });

            let file_path = Path::new(&home_dir).join(".rshell_history");

            let result = HistoryHandler::create_history_from_path(&file_path);

            if result.is_err() {
                let mut file: File = File::create(&file_path).unwrap_or_else(|err| {
                    panic!("Failed to create history file: {}", err);
                });

                let result = HistoryHandler::create_history_from_file(&mut file);

                if result.is_err() {
                    return HistoryHandler {
                        current_index: 0,
                        history: Vec::new(),
                        path: "".into(),
                    };
                }

                let history = result.unwrap();

                return HistoryHandler {
                    current_index: history.len(),
                    history: history,
                    path: file_path,
                };
            } else {
                let history = result.unwrap();

                return HistoryHandler {
                    current_index: history.len(),
                    history: history,
                    path: file_path,
                };
            }
        }

        let history_path = Path::new(&env_dir.unwrap()).join(".rshell_history");

        let result = HistoryHandler::create_history_from_path(&history_path);

        if result.is_err() {
            return HistoryHandler {
                current_index: 0,
                history: Vec::new(),
                path: "".into(),
            };
        }

        let history = result.unwrap();

        return HistoryHandler {
            current_index: history.len(),
            history: history,
            path: history_path,
        };
    }

    fn add_entry(&mut self, entry: &str) {
        self.history.push(HistoryEntry::now(entry.to_string()));
        self.current_index = self.history.len();
    }

    fn remove_entry(&mut self, index: usize) -> Result<(), Error> {
        if index >= self.history.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Index out of bounds"));
        }

        self.history.remove(index);

        Ok(())
    }

    fn get_previous(&mut self) -> Option<String> {
        if self.current_index == 0 {
            return None;
        }

        self.current_index -= 1;

        Some(self.history[self.current_index].command.clone())
    }

    fn get_next(&mut self) -> Option<String> {
        if self.history.len() == 0 || self.current_index >= self.history.len() - 1 {
            return None;
        } else {
            self.current_index += 1;

            return Some(self.history[self.current_index].command.clone());
        }
    }

    fn get_nth(&self, n: usize) -> Option<String> {
        if n >= self.history.len() {
            return None;
        }

        Some(self.history[n].command.clone())
    }

    fn get_all(&self) -> Vec<String> {
        return self
            .history
            .iter()
            .map(|entry| entry.command.clone())
            .collect::<Vec<String>>();
    }

    fn load(&mut self) -> Result<(), Error> {
        if !self.path.exists() || self.path.to_str().unwrap_or("").is_empty() {
            return Ok(());
        }

        let file = File::open(&self.path)?;

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;

            let entry = HistoryEntry::from_line(&line)?;

            self.history.push(entry);
        }

        Ok(())
    }

    fn save(&self) -> Result<(), Error> {
        if !self.path.exists() || self.path.to_str().unwrap_or("").is_empty() {
            return Ok(());
        }

        let temp_path = Path::new("/tmp/.rshell_history");

        let mut file = File::create(&temp_path)?;

        file.lock()?;

        let content = self
            .history
            .iter()
            .map(|entry| {
                format!(
                    "{} {}",
                    entry
                        .timestamp
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_else(|_| Duration::from_secs(0))
                        .as_secs(),
                    entry.command
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        file.write(content.as_bytes())?;

        file.unlock().map_err(|err| {
            fs::copy(temp_path, &self.path).unwrap_or_else(|copy_err| {
                panic!(
                    "Failed to copy temporary history file back to original location: {}",
                    copy_err
                );
            });
            err
        })?;

        fs::copy(&temp_path, &self.path)?;

        fs::remove_file(&temp_path)?;

        Ok(())
    }

    fn clear(&mut self) -> Result<(), Error> {
        self.history.clear();
        fs::remove_file(&self.path)?;
        File::create(&self.path)?;

        Ok(())
    }

    fn set_default_path(&mut self, path: String) -> Result<(), Error> {
        let previous_path = self.path.clone();

        self.path = std::path::PathBuf::from(path);

        if !self.path.exists() {
            std::fs::File::create(&self.path)?;
        }

        let content = previous_path.to_string_lossy().into_owned();

        std::fs::write(&self.path, content)?;

        Ok(())
    }
}

impl HistoryHandler {
    fn create_history_from_path(file_path: &Path) -> Result<Vec<HistoryEntry>, std::io::Error> {
        if !file_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "History file not found",
            ));
        }

        let mut file = File::open(&file_path).unwrap_or_else(|err| {
            panic!("Failed to open history file: {}", err);
        });

        return HistoryHandler::create_history_from_file(&mut file);
    }

    fn create_history_from_file(mut file: &mut File) -> Result<Vec<HistoryEntry>, std::io::Error> {
        let mut content_buf = String::new();

        File::read_to_string(&mut file, &mut content_buf).map_err(|err| {
            Error::new(
                ErrorKind::Other,
                format!("History file couldn't read: {}", err),
            )
        })?;

        let history = content_buf
            .lines()
            .map(|line| HistoryEntry::from_line(line))
            .filter(|he| he.is_ok())
            .map(|he| he.unwrap())
            .collect::<Vec<HistoryEntry>>();

        Ok(history)
    }
}

#[derive(Debug)]
struct HistoryEntry {
    timestamp: SystemTime,
    command: String,
}

impl HistoryEntry {
    fn new(command: String, timestamp: SystemTime) -> Self {
        HistoryEntry { timestamp, command }
    }
    fn now(command: String) -> Self {
        HistoryEntry {
            timestamp: SystemTime::now(),
            command,
        }
    }

    fn from_line(line: &str) -> Result<Self, std::io::Error> {
        let (time, command) = line
            .split_once(" ")
            .ok_or_else(|| std::io::Error::new(ErrorKind::InvalidData, "split failed"))?;

        let timestamp = time
            .parse::<u64>()
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err.to_string()))?;

        Ok(HistoryEntry {
            timestamp: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp),
            command: command.to_string(),
        })
    }
}
