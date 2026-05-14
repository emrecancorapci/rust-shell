use std::{
    fs::{self, File, OpenOptions},
    io::{Error, ErrorKind, Read, Write},
    path::{Path, PathBuf},
};

use crate::shell::core::ShellHistoryHandler;

pub struct HistoryHandler {
    current_index: usize,
    appended_index: usize,
    history: Vec<String>,
    path: PathBuf,
}

impl ShellHistoryHandler for HistoryHandler {
    fn new() -> Self
    where
        Self: Sized,
    {
        HistoryHandler {
            current_index: 0,
            appended_index: 0,
            history: Vec::new(),
            path: "".into(),
        }
    }

    fn add_entry(&mut self, entry: &str) {
        self.history.push(entry.to_string());
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

        Some(self.history[self.current_index].to_string())
    }

    fn get_next(&mut self) -> Option<String> {
        if self.history.len() == 0 || self.current_index >= self.history.len() - 1 {
            return None;
        } else {
            self.current_index += 1;

            return Some(self.history[self.current_index].to_string());
        }
    }

    fn get_nth(&self, n: usize) -> Option<String> {
        if n >= self.history.len() {
            return None;
        }

        Some(self.history[n].to_string())
    }

    fn get_all(&self) -> &Vec<String> {
        return &self.history;
    }

    fn load(&mut self) -> Result<(), Error> {
        let file_path = if let Ok(file_path) = std::env::var("HISTFILE") {
            Path::new(&file_path).to_path_buf()
        } else {
            let env_dir = std::env::var("HOME").unwrap_or_else(|err| {
                panic!("Failed to get HOME directory: {}", err);
            });

            Path::new(&env_dir).join(".rshell_history")
        };

        let mut result = HistoryHandler::create_history_from_path(&file_path);

        if result.is_err() {
            let mut file: File = File::create(&file_path).unwrap_or_else(|err| {
                panic!("Failed to create history file: {}", err);
            });

            result = HistoryHandler::create_history_from_file(&mut file);

            if result.is_err() {
                self.current_index = 0;
                self.history = Vec::new();
                self.path = "".into();

                return Ok(());
            }
        }

        self.history = result.unwrap();
        self.current_index = self.history.len();
        self.path = file_path;

        return Ok(());
    }

    fn load_from(&mut self, file_path: PathBuf) -> Result<(), Error> {
        let loaded_history = HistoryHandler::create_history_from_path(&file_path)?;
        self.history.extend(loaded_history);
        self.current_index = self.history.len();
        self.path = file_path.into();

        return Ok(());
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
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("\n")
            + "\n";

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

    fn save_to(&self, file_path: PathBuf) -> Result<(), Error> {
        let mut file = File::create(&file_path)?;

        let content = self
            .history
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<String>>()
            .join("\n")
            + "\n";

        file.write(content.as_bytes())?;

        return Ok(());
    }

    fn append_to(&mut self, file_path: PathBuf) -> Result<(), Error> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&file_path);

        if file.is_err() {
            let err = file.unwrap_err();

            return Err(Error::new(
                ErrorKind::Other,
                format!("File read error: {}", err.to_string()),
            ));
        }

        let mut file = file.unwrap();

        match file.lock() {
            Ok(_) => {}
            Err(err) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("File lock error: {}", err.to_string()),
                ))
            }
        }

        let content = self
            .history
            .iter()
            .map(|entry| entry.to_string())
            .skip(self.appended_index)
            .collect::<Vec<String>>()
            .join("\n")
            + "\n";

        match file.write(content.as_bytes()) {
            Ok(_) => {}
            Err(err) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("File write error: {}", err.to_string()),
                ))
            }
        }

        match file.unlock() {
            Ok(_) => {}
            Err(err) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("File unlock error: {}", err.to_string()),
                ))
            }
        }

        self.appended_index = self.history.len();

        return Ok(());
    }

    fn reset_index(&mut self) {
        self.current_index = self.history.len();
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
    fn create_history_from_path(file_path: &Path) -> Result<Vec<String>, std::io::Error> {
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

    fn create_history_from_file(mut file: &mut File) -> Result<Vec<String>, std::io::Error> {
        let mut content_buf = String::new();

        File::read_to_string(&mut file, &mut content_buf).map_err(|err| {
            Error::new(
                ErrorKind::Other,
                format!("History file couldn't read: {}", err),
            )
        })?;

        let history = content_buf
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        Ok(history)
    }
}
