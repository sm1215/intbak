use std::path::{PathBuf};
use std::{fs, io};

// struct PrintDebug {
//     enabled: bool,
// }

// impl PrintDebug {
//     pub fn new(enabled: bool) -> Self {
//         Self {
//             enabled
//         }
//     }
//     pub fn pipe(&self, message: String) {
//         if self.enabled {
//             println!("{:?}", message)
//         }
//     }
// }

pub fn create_nested_directory(path: &PathBuf) -> io::Result<()> {
    // PrintDebug::pipe(format!("creating directory {:#?}", path));
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn copy_file(source: &PathBuf, destination: &PathBuf) -> io::Result<()> {
    // list_debug(
    //     format!("copying {:?} to {:?}", source, destination),
    //     debug,
    // );
    let mut destination_dir = destination.clone();
    destination_dir.pop();
    if !destination_dir.exists() {
        create_nested_directory(&destination_dir)?;
    }
    let copy_file_result = fs::copy(source, destination);
    // list_debug(
    //     format!("copy result: {:?}", copy_file_result),
    //     debug,
    // );
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
    // if debug {
    //     println!("\nentering source {:#?}", source);
    // }

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

pub fn run_backup(source: PathBuf, destination: PathBuf, targets: Vec<&str>, debug: bool) {
    // let debugger = PrintDebug::new(debug);
    for entry in targets {
        let mut source = source.clone();
            source.push(entry);
        let mut destination = destination.clone();
            destination.push(entry);
        let _r = copy_directory_contents(&source, &destination);
    }
}
