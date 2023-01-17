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
use tempfile::{tempdir, tempfile};

pub fn _create_test_directory(path1: &str, path2: &str) -> Result<PathBuf> {
    let temp_dir: PathBuf = tempdir().unwrap().path().to_owned();
    let source: PathBuf = temp_dir.join(path1).join(path2);
    Ok(source)
}

pub fn build_source_directory(name: &str) -> Result<PathBuf> {
    println!("Create test source directory");
    let source: PathBuf = _create_test_directory(name, "source")?;
    create_dir_all(source.as_path())?;
    Ok(source)
}

pub fn build_target_directory(name: &str) -> Result<PathBuf> {
    println!("Create test target directory");
    let target: PathBuf = _create_test_directory(name, "target")?;
    create_dir_all(target.as_path())?;
    Ok(target)
}

pub fn clear_directory(path: &Path) -> Result<()> {
    println!("Clean test directory {}", path.display());
    if path.exists() {
        remove_dir_all(path).unwrap()
    }
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

pub fn with_test_directories(name: &str, test: impl FnOnce(&PathBuf, &PathBuf) -> () + std::panic::UnwindSafe) -> Result<()> {
    let source: PathBuf = build_source_directory(name).unwrap();
    let target: PathBuf = build_target_directory(name).unwrap();

    let result = panic::catch_unwind(|| {
        test(&source, &target);
    });

    clear_directory(source.as_path()).unwrap();
    clear_directory(target.as_path()).unwrap();

    assert!(result.is_ok(), "Test failed. {}", name);

    Ok(())
}
