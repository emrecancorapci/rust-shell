use std::os::unix::fs::PermissionsExt;
use std::{
    env,
    path::{Path, PathBuf},
};
pub trait ExecutionPath {
    fn get_exec_path(&self) -> Option<PathBuf>;
}

const IS_EXECUTABLE: u32 = 0o111;

impl ExecutionPath for &String {
    fn get_exec_path(&self) -> Option<PathBuf> {
        let path_str = env::var("PATH").unwrap_or_default();

        let path_arr = env::split_paths(&path_str).collect::<Vec<_>>();

        for path in path_arr {
            let cmd_path = Path::new(&path).join(self);

            if cmd_path.exists()
                && cmd_path.is_file()
                && cmd_path
                    .metadata()
                    .map(|m| m.permissions().mode() & IS_EXECUTABLE != 0)
                    .unwrap_or(false)
            {
                return Some(cmd_path);
            }
        }

        None
    }
}
