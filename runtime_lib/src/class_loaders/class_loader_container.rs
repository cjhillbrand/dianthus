use std::path::PathBuf;

use crate::class_loaders::bootstrap_class_loader::BootStrapClassLoader;
use crate::class_loaders::class_loader::ClassLoader;
use crate::class_loaders::extension_class_loader::ExtensionClassLoader;
use crate::class_loaders::system_class_loader::SystemClassLoader;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum ClassLoaderContainer {
	Bootstrap(BootStrapClassLoader),
	Extension(ExtensionClassLoader),
	System(SystemClassLoader),
	None
}

impl ClassLoader for ClassLoaderContainer {
	fn path_buf(&self) -> PathBuf {
		match self {
			ClassLoaderContainer::Bootstrap(loader) => loader.path_buf(),
			ClassLoaderContainer::Extension(loader) => loader.path_buf(),
			ClassLoaderContainer::System(loader) => loader.path_buf(),
			ClassLoaderContainer::None => {
				panic!("Could not load class.")
			}
		}
	}

	fn parent(&self) -> ClassLoaderContainer {
		match self {
			ClassLoaderContainer::Bootstrap(loader) => loader.parent(),
			ClassLoaderContainer::Extension(loader) => loader.parent(),
			ClassLoaderContainer::System(loader) => loader.parent(),
			ClassLoaderContainer::None => {
				panic!("Could not load class.")
			}
		}
	}
}
