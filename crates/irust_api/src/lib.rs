use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Continue,
    DeleteNextWord,
    Multiple(Vec<Command>),
    SetThinCursor,
    SetWideCursor,
    HandleCharacter(char),
    HandleEnter(bool),
    HandleAltEnter,
    HandleTab,
    HandleBackTab,
    HandleRight,
    HandleLeft,
    HandleBackSpace,
    HandleDelete,
    HandleCtrlC,
    HandleCtrlD,
    HandleCtrlE,
    HandleCtrlL,
    HandleCtrlR,
    HandleCtrlZ,
    HandleUp,
    HandleDown,
    HandleCtrlRight,
    HandleCtrlLeft,
    HandleHome,
    HandleEnd,
    RemoveRacerSugesstionsAndReprint,
    Exit,
}

#[derive(Serialize, Deserialize)]
pub struct GlobalVariables {
    current_working_dir: PathBuf,
    previous_working_dir: PathBuf,
    last_loaded_code_path: Option<PathBuf>,
    /// last successful output
    last_output: Option<String>,
    pub operation_number: usize,

    pub prompt_position: (usize, usize), // (row, col)
    pub prompt_len: usize,
}

impl Default for GlobalVariables {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalVariables {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().expect("Error getting current working directory");

        Self {
            current_working_dir: cwd.clone(),
            previous_working_dir: cwd,
            last_loaded_code_path: None,
            last_output: None,
            operation_number: 1,
            prompt_position: (0, 0), // (row, col)
            prompt_len: 0,
        }
    }

    pub fn update_cwd(&mut self, cwd: PathBuf) {
        self.previous_working_dir = self.current_working_dir.clone();
        self.current_working_dir = cwd;
    }

    pub fn get_cwd(&self) -> PathBuf {
        self.current_working_dir.clone()
    }

    pub fn get_pwd(&self) -> PathBuf {
        self.previous_working_dir.clone()
    }

    pub fn set_last_loaded_coded_path(&mut self, path: PathBuf) {
        self.last_loaded_code_path = Some(path);
    }

    pub fn get_last_loaded_coded_path(&self) -> Option<PathBuf> {
        self.last_loaded_code_path.clone()
    }

    pub fn get_last_output(&self) -> Option<&String> {
        self.last_output.as_ref()
    }

    pub fn set_last_output(&mut self, out: String) {
        self.last_output = Some(out);
    }
}
