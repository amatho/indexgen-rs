use std::fs;
use std::path;
use std::env;

extern crate handlebars;
#[macro_use]
extern crate serde_json;

struct File {
  file_type: String,
  path: String,
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let working_dir = if args.len() > 1 {
    &args[1]
  } else {
    "."
  };

  let paths = match fs::read_dir(working_dir) {
    Ok(p) => p,
    Err(_) => {
      println!("Could not read directory {}.", working_dir);
      return;
    }
  };

  println!("Working Directory: {}", working_dir);
  for path in paths {
    let path = path.unwrap().path();
    let path_type = match path.is_dir() {
      true => "dir",
      false => "file"
    };
    let filename = path::Path::new(path.file_name().unwrap()).to_str().unwrap();
    let first_char = &filename[0..1];

    if first_char == "." {
      continue;
    }

    let file = File {
      file_type: String::from(path_type),
      path: String::from(filename)
    };

    println!("type: {:?} - path: {:?}", file.file_type, file.path);
  }
}
