use std::path::PathBuf;

use crate::class_loaders::bootstrap_class_loader::BootStrapClassLoader;
use crate::class_loaders::class_loader::{get_java_home, ClassLoader};
use crate::class_loaders::class_loader_container::ClassLoaderContainer;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionClassLoader {}

const RELATIVE_PATH: &str = "/lib/ext";

impl ClassLoader for ExtensionClassLoader {
	fn path_buf(&self) -> PathBuf {
		let java_home = get_java_home();
		let mut buf: PathBuf = PathBuf::new();
		buf.push(java_home);
		buf.push(RELATIVE_PATH);
		buf
	}

	fn parent(&self) -> ClassLoaderContainer { ClassLoaderContainer::Bootstrap(BootStrapClassLoader {}) }
}
