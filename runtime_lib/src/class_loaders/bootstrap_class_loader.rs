use std::collections::VecDeque;
use std::fs;
use std::io::Read;
use std::iter::FromIterator;
use std::path::PathBuf;

use crate::class_loaders::class_loader::{get_java_home, ClassLoader};
use crate::class_loaders::class_loader_container::ClassLoaderContainer;
use crate::entities::class_struct::ClassStruct;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct BootStrapClassLoader {}

const RELATIVE_PATH: &str = "jre/lib";
const RT_JAR: &str = "rt.jar";

impl ClassLoader for BootStrapClassLoader {
	fn path_buf(&self) -> PathBuf {
		let java_home = get_java_home();
		let mut buf: PathBuf = PathBuf::new();
		buf.push(java_home);
		buf.push(RELATIVE_PATH);
		buf
	}

	fn parent(&self) -> ClassLoaderContainer { ClassLoaderContainer::None }

	fn load_class(&self, file: &str) -> ClassStruct {
		match self.load_rt_jar(file) {
			Some(c) => c,
			_ => ClassLoader::load_class(self, file)
		}
	}
}

impl BootStrapClassLoader {
	pub fn load_rt_jar(&self, file_name: &str) -> Option<ClassStruct> {
		let mut path: PathBuf = self.path_buf();
		path.push(RT_JAR);
		let buf: Vec<u8> = match fs::read(&path) {
			Ok(data_vec) => data_vec,
			Err(_err) => panic!("Could not load rt.jar from java home.")
		};

		let reader = std::io::Cursor::new(buf);
		let mut zip = match zip::ZipArchive::new(reader) {
			Ok(archive) => archive,
			Err(err) => panic!("{}", err)
		};

		for i in 0..zip.len() {
			let mut file = zip.by_index(i).unwrap();
			if file.name().eq(file_name) {
				let mut contents: Vec<u8> = Vec::new();
				match file.read_to_end(&mut contents) {
					Ok(_) => {}
					Err(e) => panic!("Could not read file: {} with err {}", file.name(), e)
				}
				let mut data: VecDeque<u8> = VecDeque::from_iter(contents);
				return Some(ClassStruct::new(&mut data));
			}
		}

		None
	}
}
