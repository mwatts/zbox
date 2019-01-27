//! base module document
//!

pub mod crypto;
pub mod lru;
mod refcnt;
mod time;
pub mod utils;
mod version;

pub use self::refcnt::RefCnt;
pub use self::time::Time;
pub use self::version::Version;

use std::sync::{Arc, Once, RwLock, ONCE_INIT};

#[cfg(target_os = "android")]
use log::Level;

#[cfg(target_os = "android")]
use android_logger::{self, Filter};

#[cfg(target_arch = "wasm32")]
use log::Level;

#[cfg(target_arch = "wasm32")]
use wasm_logger;

#[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
use env_logger;

static INIT: Once = ONCE_INIT;

/// Initialise ZboxFS environment.
///
/// This function should be called before any other functions provided by ZboxFS.
/// This function can be called more than one time.
pub fn init_env() {
    // only call the initialisation code once globally
    INIT.call_once(|| {
        #[cfg(target_os = "android")]
        {
            android_logger::init_once(
                Filter::default()
                    .with_min_level(Level::Trace)
                    .with_allowed_module_path("zbox::fs::fs")
                    .with_allowed_module_path("zbox::trans::txmgr"),
                Some("zboxfs"),
            );
        }
        #[cfg(target_arch = "wasm32")]
        {
            wasm_logger::init(wasm_logger::Config::new(Level::Trace));
        }
        #[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
        {
            env_logger::try_init().ok();
        }
        crypto::Crypto::init().expect("Initialise crypto failed");
    });
}

/// Wrap type into reference type Arc<RwLock<T>>
pub trait IntoRef: Sized {
    fn into_ref(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}
