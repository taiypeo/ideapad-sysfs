mod action;
mod file;

use action::{Action, Error};
use file::File;

pub struct FileAction {
    file: File,
    action: Action,
}

impl FileAction {
    fn new(file: File, action: Action) -> FileAction {
        FileAction { file, action }
    }

    fn perform(&self) -> Result<(), Error> {
        self.action.perform(self.file.filename())
    }
}
