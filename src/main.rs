use std::env;
use std::fs::File;
use std::path::Path;
use zip_extract::extract;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() {
   let args: Vec<String> = env::args().collect();

    let mut filename = None;

    for i in 1..args.len() {
        let arg = &args[i];

        if arg.starts_with("-f=") {
            filename = Some(arg.trim_start_matches("-f="));
        } else if arg == "-f" && i + 1 < args.len() {
            filename = Some(&args[i+1]);
        }
    }
    
    let filename: &str = filename
        .expect("Error: Please specify a file!");

    match get_file_type(filename) {
        Some("zip") => extract_zip(filename),
        Some("tar.gz") => extract_tarball(filename).expect("Failed to extract tar.gz file"),
        _ => {
            eprintln!("Error: Unsupported filetype. Aborting...");
            std::process::exit(1);
        }
    }
}

fn get_file_type(filename: &str) -> Option<&str> {
    if filename.ends_with(".tar.gz") {
        Some("tar.gz")
    } else if filename.ends_with(".zip") {
        Some("zip")
    } else {
        None
    }
}

fn extract_zip(filename: &str) {
    println!("Type 'zip', extracting...");

    let output_dir = Path::new("data");

    let zip_content = std::fs::read(filename);

    match zip_content {
        Ok(content) => {
            extract(std::io::Cursor::new(content), &output_dir, true).expect("Failed to extract zip file");
            println!("Successfully extracted {}, continuing...", filename);
        },
        Err(e) => {
            eprintln!("Error reading zip content: {}", e);
        }
    }
}

fn extract_tarball(filename: &str) -> Result<(), std::io::Error> {
    println!("Type 'tar.gz', extracting...");

    let tar_gz = File::open(filename).expect("Failed to read tar.gz file");
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack("data")?;
    
    println!("Successfully extracted {}, continuing...", filename);

    Ok(())
}
