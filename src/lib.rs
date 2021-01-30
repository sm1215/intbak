use std::path::{PathBuf};
use std::{fs, io};

// #[derive(Debug)]
// enum ListDebug {
//     Enabled(bool),
//     Pipe,
// }

// /// Wheter to print verbosely 
// impl Default for ListDebug {
//     fn default() -> Self {
//         ListDebug::Enabled(false)
//     }
//     // fn new(is_enabled: bool) -> Self {
//     //     ListDebug::Enabled(is_enabled)
//     // }
//     // fn new(is_enabled: bool) -> Self {
//     //     return Self {
//     //         is_enabled
//     //     }
//     // }
// }

// impl ListDebug {
//     fn pipe(message: String) {
//         match ListDebug::Enabled {
//             Some(bool) => println!("{:?}", message),
//             None => None,
//         }
//     }
// }

pub fn create_nested_directory(path: &PathBuf) -> io::Result<()> {
    // if ListDebug::is_enabled() {
    //     println!("creating directory {:#?}", path);
    // }
    // ListDebug::pipe(format!("creating directory {:#?}", path));
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn copy_file(source: &PathBuf, destination: &PathBuf) -> io::Result<()> {
    println!("copying {:?} to {:?}", source, destination);
    let mut destination_dir = destination.clone();
    destination_dir.pop();
    if !destination_dir.exists() {
        create_nested_directory(&destination_dir)?;
    }
    let copy_file_result = fs::copy(source, destination);
    println!("copy result: {:?}", copy_file_result);
    Ok(())
}

pub fn set_write_perms(path: &PathBuf) {
    let mut perms = fs::metadata(&path)
        .expect("error getting permissions")
        .permissions();
    perms.set_readonly(false);
    fs::set_permissions(&path, perms.clone())
        .expect("error setting permissions");
}

pub fn copy_directory_contents(source: &PathBuf, destination: &PathBuf) -> io::Result<()> {
    println!("\nentering source {:#?}", source);

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // need to pull tail from path and append to destination
            let nested_path = match path.iter().last() {
                Some(nested_path) => nested_path,
                None => {
                    println!("no nested_path");
                    break;
                }
            };
            let mut destination = destination.clone();
            destination.push(&nested_path);
            copy_directory_contents(&path, &destination)?;
        } else {
            let mut destination = destination.clone();
            let filename = match path.file_name() {
                Some(filename) => filename,
                None => {
                    println!("no filename");
                    break;
                }
            };
            destination.push(&filename);
            copy_file(&path, &destination)?;
        }
    }
    
    Ok(())
}

pub fn run_backup(source: PathBuf, destination: PathBuf, targets: Vec<&str>) {
    for entry in targets {
        let mut source = source.clone();
            source.push(entry);
        let mut destination = destination.clone();
            destination.push(entry);
        let _r = copy_directory_contents(&source, &destination);
    }
}