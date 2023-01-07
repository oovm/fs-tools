use std::env::current_exe;

use super::*;

/// find_directory
///
/// # Arguments
///
/// * `start`: The starting directory for searching
/// * `name`: Target directory name
///
/// # Examples
///
/// ```
/// use find-target::find_directory;
/// ```
pub fn find_directory(start: &Path, name: &str) -> Result<PathBuf> {
    let normed = ensure_file(start, name)?;
    let mut here = normed.as_path();
    while let Some(dir) = here.parent() {
        let path = here.join(name);
        if path.exists() {
            return match path.is_dir() {
                true => Ok(path),
                false => Err(Error::from_raw_os_error(22)),
            };
        } else {
            here = dir;
        }
    }
    Err(Error::from_raw_os_error(20))
}

/// find_directory_or_create
///
/// # Arguments
///
/// * `start`: The starting directory for searching
/// * `name`: Target directory name
///
/// # Examples
///
/// ```
/// find-target::find_directory_or_create;
/// ```
pub fn find_directory_or_create(start: &Path, name: &str) -> Result<PathBuf> {
    match find_directory(start, name) {
        Ok(o) => return Ok(o),
        Err(_) => {}
    }
    let dir = ensure_directory(start)?.join(name);
    create_dir_all(&dir)?;
    Ok(dir)
}


/// Get the folder where the executable file is located
///
/// # Examples
///
/// ```
/// print!("{}", find_target::this_directory().unwrap().display());
/// ```
pub fn this_directory() -> Result<PathBuf> {
    match current_exe()?.canonicalize()?.parent() {
        Some(s) => { Ok(s.to_path_buf()) }
        None => { Err(Error::from_raw_os_error(10006)) }
    }
}