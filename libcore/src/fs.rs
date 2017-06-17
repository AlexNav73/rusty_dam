
use std::fs;
use std::path::{Path, PathBuf};

use Entity;
use models::file::File as RFile;

pub(crate) struct FileService {
    work_dir: PathBuf
}

impl FileService {
    pub(crate) fn new<S: AsRef<Path>>(path: S) -> Self {
        if path.as_ref().is_dir() {
            FileService { work_dir: path.as_ref().to_path_buf() }
        } else {
            // TODO: Return result, do not panic
            panic!("Working dir not found!")
        }
    }

    pub(crate) fn move_file(&self, file: &RFile) -> Result<String, ()> {
        let file_id = file.id().to_string();
        match fs::create_dir(&file_id) {
            Ok(_) => {
                let new_file = self.work_dir.as_path()
                    .join(file_id)
                    .join(file.name());
                fs::copy(file.path(), &new_file).map_err(|_| ())?;
                Ok(new_file.into_os_string().into_string().unwrap())
            },
            Err(e) => panic!("Can't create directory for file: {}", e)
        }
    }
}

