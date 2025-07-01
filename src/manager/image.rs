use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use crate::manager::ai::OpenAI;

#[derive(Debug, Clone)]
pub struct SSManager {
    ai: OpenAI,
}

impl SSManager {
    pub fn new(ai: OpenAI) -> Self {
        Self { ai }
    }

    fn modify_ss_path(&self, path: &PathBuf) -> PathBuf {
        // initially the path of the file starts with .<file_name>
        // we need to remove the . from the file name
        let filename = path.file_name().unwrap().to_str().unwrap()[1..].to_string();
        let parent = path.parent().unwrap_or(Path::new("."));
        parent.join(filename)
    }

    pub fn is_screenshot_file(&self, path: &PathBuf) -> bool {
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

    fn delete_file(&self, path: &PathBuf) -> Result<(), anyhow::Error> {
        if let Err(e) = fs::remove_file(path) {
            return Err(anyhow::anyhow!(
                "Failed to delete file: {:?}, Error: {}",
                path,
                e
            ));
        }
        Ok(())
    }

    async fn process_ss(&self, path: &PathBuf) -> Result<(), anyhow::Error> {
        // create new filename
        let mut new_filename = self.ai.get_name(&path).await;
        new_filename += ".png";

        // create new path
        let parent = path.parent().unwrap_or(Path::new("."));
        let new_path = parent.join(new_filename);

        // copy file to new path
        if let Err(e) = fs::copy(path.clone(), &new_path) {
            return Err(anyhow::anyhow!(
                "Failed to copy file: {:?} -> {:?}, Error: {}",
                path.clone(),
                new_path,
                e
            ));
        }

        // delete old file
        self.delete_file(&path)
    }

    pub async fn process_new_ss(&self, path: &PathBuf) -> Result<(), anyhow::Error> {
        if !self.is_screenshot_file(path) {
            return Err(anyhow::anyhow!(
                "file is not screenshot or not recent: {:?}",
                path
            ));
        }

        let path = self.modify_ss_path(path);

        if !self.is_recent(&path, Duration::from_secs(30)) {
            return Err(anyhow::anyhow!("Skipping old file: {:?}", path));
        }

        self.process_ss(&path).await
    }

    pub async fn process_random_image(&self, path: &PathBuf) -> Result<(), anyhow::Error> {
        let file_type = match path.extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => return Err(anyhow::anyhow!("Failed to get file extension")),
        };

        let parent = path.parent().unwrap_or(Path::new("."));

        let mut new_filename: String = self.ai.get_name(&path).await;
        new_filename += &format!(".{}", file_type);
        dbg!(&new_filename);

        let new_path = parent.join(new_filename);

        if let Err(e) = fs::copy(path.clone(), &new_path) {
            return Err(anyhow::anyhow!(
                "Failed to copy file: {:?} -> {:?}, Error: {}",
                path,
                new_path,
                e
            ));
        }

        self.delete_file(&path)
            .map_err(|e| anyhow::anyhow!("Failed to delete file: {:?}, Error: {}", path, e))
    }
}
