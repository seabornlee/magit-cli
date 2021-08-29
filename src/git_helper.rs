use std::error::Error;
use git2::{Repository, StatusOptions};

pub(crate) fn get_unstaged() -> Result<Vec<String>, Box<dyn Error>> {
    let repo = Repository::open(".").expect("failed to open");
    let mut options = StatusOptions::default();
    options.include_untracked(true);
    let mut result = Vec::new();
    let statuses = repo.statuses(Some(&mut options))?;
    for e in statuses.iter() {
        let status = e.status();
        if status.is_wt_new() || status.is_wt_modified() {
            let x = e.path_bytes();
            let path = String::from_utf8(x.to_vec())?;
            result.push(path)
        }
    }
    Ok(result)
}

pub(crate) fn get_staged() -> Result<Vec<String>, Box<dyn Error>> {
    let repo = Repository::open(".").expect("failed to open");
    let mut options = StatusOptions::default();
    options.include_untracked(true);
    let mut result = Vec::new();
    let statuses = repo.statuses(Some(&mut options))?;
    for e in statuses.iter() {
        let status = e.status();
        if status.is_index_new() || status.is_index_modified() {
            let x = e.path_bytes();
            let path = String::from_utf8(x.to_vec())?;
            result.push(path)
        }
    }
    Ok(result)
}