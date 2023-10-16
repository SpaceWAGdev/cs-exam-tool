use procfs::{self, process::ProcessesIter};

pub fn list_all_processes() -> ProcessesIter{
    procfs::process::all_processes().unwrap()
}