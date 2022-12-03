use std::path::Path;

mod version;

fn main() {
    let path = Path::new("/Users/jdkato/Desktop");
    let resp = version::install(path, "latest", "macOS_64-bit");

    let _ = match resp {
        Ok(()) => println!("Installed Vale to {:#?}", path),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
