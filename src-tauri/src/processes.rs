#[cfg(windows)]
use process_list;
#[cfg(not(windows))]
use procfs;

#[cfg(windows)]
pub fn list_all_processes() -> Vec<String> {
    // TODO: Implement Windows feature
}

#[cfg(not(windows))]
pub fn list_all_processes() -> Vec<String> {
    let mut process_list = Vec::new();
    for p in procfs::process::all_processes().unwrap() {
        process_list.push(
            match p {
                Ok(process) => {
                    match process.exe() {
                        Ok(exe) => String::from(exe.to_str().unwrap()),
                        Err(_) => continue,
                    }
                },
                Err(_) => continue,
            }
        );
    }
    process_list
}