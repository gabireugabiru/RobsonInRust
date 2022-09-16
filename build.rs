use robson_compiler::CompilerInfra;
use serde::Deserialize;
use std::{
  env::{self, current_dir},
  fs::{self, File},
  io::{ErrorKind, Write},
  process::exit,
};
#[derive(Deserialize)]
struct Config {
  include: Vec<String>,
}
#[derive(Clone, Copy)]
pub struct BuildInfra {}
impl CompilerInfra for BuildInfra {
  fn clone_self(&mut self) -> Box<dyn CompilerInfra> {
    Box::new(*self)
  }
  fn color_print(&mut self, _: String, _: u64) {}
  fn println(&mut self, _: String) {}
}

fn main() {
  match env::current_exe() {
    Ok(mut current_exe) => {
      current_exe.pop();
      current_exe.pop();
      current_exe.pop();
      current_exe.pop();
      current_exe.pop();
      match env::set_current_dir(&current_exe) {
        Ok(_) => {}
        Err(err) => {
          eprintln!(
            "Configuration error\n===> Couldnt set current dir to {:?}\n",
            &current_exe
          );
          eprintln!("{}", err);
          exit(1);
        }
      };
    }
    Err(_) => {}
  }

  let config_dir = env::current_dir().unwrap().join("robson.toml");
  let config_dir = config_dir.as_os_str();
  let a = match fs::read_to_string("robson.toml") {
    Ok(a) => a,
    Err(err) => {
      eprintln!(
        "Configuration error\n===> Couldnt open {:?}\n",
        &config_dir
      );
      eprintln!("{}", err);
      exit(1);
    }
  };
  let a = match toml::from_str::<Config>(&a) {
    Ok(a) => a,
    Err(err) => {
      eprintln!(
        "Configuration error\n===> Couldnt parse {:?}\n",
        &config_dir
      );
      eprintln!("{}", err);
      exit(1);
    }
  };

  let out_dir = current_dir().unwrap().as_path().join("src").join("rbsns");

  {
    let result = std::fs::create_dir("src/rbsns");
    if let Err(err) = result {
      if err.kind() != ErrorKind::AlreadyExists {
        eprintln!(
          "IO Error\n===> Couldnt create {:?}\n",
          out_dir.as_os_str()
        );
        eprintln!("{}", err);
        exit(1)
      }
    }
  }

  for path in &a.include {
    let mut compiler = match robson_compiler::compiler::Compiler::new(
      path.clone(),
      Box::new(BuildInfra {}),
    ) {
      Ok(a) => a,
      Err(err) => {
        eprintln!(
          "Compiling error\n===> Couldnt stanciate compiler {:?}\n",
          current_dir().unwrap().as_path().join(&path).as_os_str()
        );
        eprintln!("{}", err);
        exit(1)
      }
    };
    let file_path = current_dir().unwrap().as_path().join(&path);

    let file_name = {
      let mut file_path = file_path.clone();
      file_path.set_extension("rbsn");
      let file_name = file_path.file_name().unwrap();
      file_name.to_owned()
    };
    let buffer = match compiler.compile() {
      Ok(a) => a,
      Err(err) => {
        eprintln!(
          "Compiling error\n===> Couldnt compile {:?}\n",
          file_path.as_os_str()
        );
        eprintln!("{}", err);
        exit(1)
      }
    };
    let file_path = out_dir.join(file_name);

    let mut file = match File::create(&file_path) {
      Ok(a) => a,
      Err(err) => {
        eprintln!(
          "IO error\n===> Couldnt create {:?}\n",
          file_path.as_os_str()
        );
        eprintln!("{}", err);
        exit(1)
      }
    };
    // eprintln!("{}")
    if let Err(err) = file.write(&buffer) {
      eprintln!(
        "IO error\n===> Couldnt write {:?}\n",
        file_path.as_os_str()
      );
      eprintln!("{}", err);
      exit(1)
    }
  }

  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed={}", config_dir.to_string_lossy());
  for i in a.include {
    let path = current_dir().unwrap().join(i);
    println!("cargo:rerun-if-changed={}", path.to_string_lossy());
  }
}
