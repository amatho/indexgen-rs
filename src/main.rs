mod template;

use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::env;

struct FileData {
  file_type: String,
  file_name: String,
}

pub struct Data {
  files: Vec<FileData>,
  dir: String,
}

fn main() {
  let args: Vec<_> = env::args().collect();
  let working_dir = get_wd(&args);
  let current_dir = get_current_dir(&working_dir);

  let paths = match fs::read_dir(&working_dir) {
    Ok(p) => p,
    Err(_) => {
      println!("Could not read directory {}.", str_expect(&working_dir));
      return;
    }
  };

  println!("Current Directory: {:?}", current_dir);
  let mut file_vec: Vec<FileData> = vec![];

  for entry in paths {
    let path = entry.unwrap().path();

    if should_ignore(&path) {
      continue;
    }

    let file_data = build_file_data(path);
    file_vec.push(file_data);
  }

  let data = Data { files: file_vec, dir: current_dir };
  write_index(data, working_dir);
}

fn get_wd(args: &Vec<String>) -> PathBuf {
  let wd = if args.len() > 1 {
    &args[1]
  } else {
    "."
  };

  PathBuf::from(wd)
}

fn get_current_dir(working_dir: &Path) -> String {
  let path_buf = match str_expect(working_dir) {
    "." => {
      env::current_dir().unwrap()
    },
    _ => Path::new(working_dir).to_path_buf()
  };

  get_file_name(&path_buf)
}

fn build_file_data(path: PathBuf) -> FileData {
  let file_type = match path.is_dir() {
    true => String::from("dir"),
    false => String::from("file")
  };

  let file_name = get_file_name(&path);

  FileData {
    file_type,
    file_name,
  }
}

fn should_ignore(path: &PathBuf) -> bool {
  let file_name = get_file_name(path);
  let first_char = &file_name[0..1];

  first_char == "." || &file_name[..] == "index.html"
}

fn write_index(data: Data, working_dir: PathBuf) {
  let contents = template::generate(data);
  let write_path = Path::new(&working_dir).join("index.html");
  fs::write(write_path, contents).expect("Could not write index.html.");
}

fn get_file_name(path_buf: &PathBuf) -> String {
  OsString::from(path_buf.file_name().expect("Could not find filename component of path."))
    .into_string().expect("Path does not contain valid UTF-8.")
}

fn str_expect(p: &Path) -> &str {
  p.to_str().expect("Not valid unicode.")
}
