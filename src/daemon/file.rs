use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use crate::daemon::ai::OpenAI;

#[derive(Debug, Clone)]
pub struct SSController {
    ai: OpenAI,
}

impl SSController {
    pub fn new(key: String, prompt: String) -> Self {
        let ai = OpenAI::new(key, prompt);
        Self { ai }
    }

    fn modify_path(&self, path: &PathBuf) -> PathBuf {
        let filename = path.file_name().unwrap().to_str().unwrap()[1..].to_string();
        let parent = path.parent().unwrap_or(Path::new("."));
        parent.join(filename)
    }

    fn is_screenshot_file(&self, path: &PathBuf) -> bool {
        if let Some(mut filename) = path.file_name().and_then(|n| n.to_str()) {
            filename = &filename[1..];
            let lowercase = filename.to_lowercase();
            return (lowercase.starts_with("screenshot") || lowercase.contains("screen shot"))
                && !filename.ends_with("-ss")
                && path.extension().map_or(false, |ext| ext == "png");
        }
        false
    }

    fn is_recent(&self, path: &PathBuf, max_age: Duration) -> bool {
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(created) = metadata.created().or_else(|_| metadata.modified()) {
                return SystemTime::now()
                    .duration_since(created)
                    .unwrap_or(Duration::MAX)
                    < max_age;
            }
        }
        false
    }

    fn delete_file(&self, path: &PathBuf) -> Result<(), String> {
        if let Err(e) = fs::remove_file(path) {
            return Err(format!("Failed to delete file: {:?}, Error: {}", path, e));
        }
        Ok(())
    }

    pub async fn process_file(&self, path: &PathBuf) -> Result<(), String> {
        if !self.is_screenshot_file(path) {
            return Err(format!("file is not screenshot or not recent: {:?}", path));
        }

        let path = self.modify_path(path);

        if !self.is_recent(&path, Duration::from_secs(30)) {
            return Err(format!("Skipping old file: {:?}", path));
        }

        let mut new_filename = self.ai.get_name(&path).await;
        new_filename += ".png";

        let parent = path.parent().unwrap_or(Path::new("."));
        let new_path = parent.join(new_filename);

        if let Err(e) = fs::copy(path.clone(), &new_path) {
            return Err(format!(
                "Failed to copy file: {:?} -> {:?}, Error: {}",
                path.clone(),
                new_path,
                e
            ));
        }

        self.delete_file(&path)
    }
}
