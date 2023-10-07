mod action;
mod file;

pub use action::Action;
pub use action::Error;
pub use file::File;

pub struct FileAction {
    file: File,
    action: Action,
}

impl FileAction {
    pub fn new(file: File, action: Action) -> FileAction {
        FileAction { file, action }
    }

    pub fn perform(&self) -> Result<(), Error> {
        self.action.perform(self.file.filename())
    }
}
