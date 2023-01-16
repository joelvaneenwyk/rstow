use std::clone::Clone;
use std::collections::HashMap;
use super::*;
use std::fs::{create_dir_all, File};
use std::panic;
use std::io::Result;
use std::env;
use std::sync::Mutex;
use once_cell::*;
use once_cell::sync::Lazy;

static TESTS_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| {
    return env::temp_dir().as_path().to_owned().join("rstow-tests").as_path().to_owned();
});

pub fn build_source_directory(name: &str) -> Result<PathBuf> {
    println!("Create test source directory");
    let source: PathBuf = TESTS_DIRECTORY.join(name).join("source");
    create_dir_all(source.as_path())?;
    Ok(source)
}

pub fn build_target_directory(name: &str) -> Result<PathBuf> {
    println!("Create test target directory");
    let target: PathBuf = TESTS_DIRECTORY.join(name).join("target");
    create_dir_all(target.as_path())?;
    Ok(target)
}

pub fn clear_directory(path: &Path) -> Result<()> {
    println!("Clean test directory {}", path.display());
    if path.exists() { remove_dir_all(path).unwrap() }
    Ok(())
}

pub fn add_file_to(name: &str, path: &Path) -> Result<PathBuf> {
    let file_path = path.join(name);
    println!("Add file {} in {} directory", file_path.display(), path.display());
    File::create(file_path.as_path());
    Ok(file_path)
}

pub fn add_directory_to(name: &str, path: &Path) -> Result<PathBuf> {
    let dir_path = path.join(name);
    println!("Add directory {} in {} directory", dir_path.display(), path.display());
    create_dir_all(dir_path.as_path());
    Ok(dir_path)
}

pub fn with_test_directories(name: &str, test: impl FnOnce(&PathBuf, &PathBuf) -> () + std::panic::UnwindSafe) -> Result<()>  {

    let test_dir = TESTS_DIRECTORY.join(name);
    let source: PathBuf = build_source_directory(name).unwrap();
    let target: PathBuf = build_target_directory(name).unwrap();

    let result = panic::catch_unwind(|| {
        test(&source, &target);
    });

    clear_directory(test_dir.as_path()).unwrap();
    assert!(result.is_ok());
    Ok(())
}
