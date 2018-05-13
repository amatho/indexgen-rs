extern crate handlebars;
#[macro_use]
extern crate serde_derive;

extern crate serde;

mod template;

use std::fs;
use std::path;
use std::env;

#[derive(Serialize)]
struct FileData {
  file_type: String,
  file_name: String,
}

#[derive(Serialize)]
struct Data {
  files: Vec<FileData>,
  dir: String,
}

fn main() {
  let args: Vec<_> = env::args().collect();
  let working_dir = get_wd(&args);
  let current_dir: String = env::current_dir().unwrap().file_name().unwrap().to_str().unwrap().into();

  let paths = match fs::read_dir(&working_dir) {
    Ok(p) => p,
    Err(_) => {
      println!("Could not read directory {}.", &working_dir);
      return;
    }
  };

  println!("Current Directory: {:?}", current_dir);
  let mut file_vec: Vec<FileData> = vec![];

  for path in paths {
    let path = path.unwrap().path();

    if is_hidden(&path) {
      continue;
    }

    let file_data = build_file_data(path);
    file_vec.push(file_data);
  }

  let data = Data { files: file_vec, dir: current_dir };
  write_index(data);
}

fn get_wd(args: &Vec<String>) -> String {
  let wd = if args.len() > 1 {
    &args[1]
  } else {
    "."
  };

  String::from(wd)
}

fn build_file_data(path: path::PathBuf) -> FileData {
  let file_type = match path.is_dir() {
    true => String::from("dir"),
    false => String::from("file")
  };

  let file_name = String::from(path.file_name().unwrap().to_str().unwrap());

  FileData {
    file_type,
    file_name,
  }
}

fn is_hidden(path: &path::PathBuf) -> bool {
  let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
  let first_char = &file_name[0..1];

  first_char == "."
}

fn write_index(data: Data) {
  let handlebars = handlebars::Handlebars::new();
  let temp = handlebars.render_template(template::TEMPLATE, &data).expect("Could not render template.");
  fs::write("index.html", temp).expect("Could not write index.html.");
}
