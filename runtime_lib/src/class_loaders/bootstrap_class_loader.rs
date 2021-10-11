use std::path::PathBuf;

use crate::class_loaders::class_loader::{get_java_home, ClassLoader};
use crate::class_loaders::class_loader_container::ClassLoaderContainer;

pub struct BootStrapClassLoader {}

const RELATIVE_PATH: &str = "/jre/lib";

impl ClassLoader for BootStrapClassLoader {
	fn path_buf(&self) -> PathBuf {
		let java_home = get_java_home();
		let mut buf: PathBuf = PathBuf::new();
		buf.push(java_home);
		buf.push(RELATIVE_PATH);
		buf
	}

	fn parent(&self) -> ClassLoaderContainer { ClassLoaderContainer::None }
}
