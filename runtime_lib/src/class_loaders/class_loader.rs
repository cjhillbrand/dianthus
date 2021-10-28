use std::collections::VecDeque;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::string::String;
use std::{env, fs};

use crate::class_loaders::class_loader_container::ClassLoaderContainer;
use crate::entities::class_struct::ClassStruct;

pub trait ClassLoader {
	fn path_buf(&self) -> PathBuf;
	fn parent(&self) -> ClassLoaderContainer;
	fn load_class(&self, file: &str) -> ClassStruct {
		let mut path: PathBuf = self.path_buf();
		path.push(file);
		path.set_extension("class");
		// check if class file, check if .jar
		// if .jar use zip archive, and find correct file. May want to have
		// each class loader keep track of classes that it knows of.
		// or first go, we could see how long it takes to load them all? idk
		match fs::read(&path) {
			Ok(data_vec) => {
				let mut data: VecDeque<u8> = VecDeque::from_iter(data_vec);
				ClassStruct::new(&mut data)
			}
			Err(_err) => self.parent().load_class(file)
		}
	}
}

const JAVA_HOME: &str = "JAVA_HOME";
const CLASS_PATH: &str = "CLASSPATH";

pub fn get_java_home() -> String {
	match env::var(JAVA_HOME) {
		Ok(result) => result,
		Err(err) => panic!("Can not find \"{}\" environment variable. With err: {}", JAVA_HOME, err)
	}
}

pub fn get_class_path() -> String {
	match env::var(CLASS_PATH) {
		Ok(result) => result,
		Err(err) => panic!(
			"Can not find \"{}\" environment variable. With err: {}",
			CLASS_PATH, err
		)
	}
}

#[cfg(test)]
mod tests {
	use std::env;

	use crate::class_loaders::class_loader::{get_class_path, get_java_home};

	#[test]
	#[ignore]
	fn get_class_path_resolves_environment_variable() {
		let class_path: String = get_class_path();
		println!("Class path: {}", class_path);
	}

	#[test]
	#[ignore]
	fn get_java_home_resolves_environment_variable() {
		let java_home: String = get_java_home();
		println!("Java home: {}", java_home);
	}

	#[test]
	#[ignore]
	fn print_env_variables() {
		for (n, v) in env::vars() {
			println!("{}: {}", n, v);
		}
	}
}
