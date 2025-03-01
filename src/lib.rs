#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
use linux as sys;
#[cfg(target_os = "macos")]
use macos as sys;
#[cfg(target_os = "windows")]
use windows as sys;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MountInfo {
    /// Mount path
    pub path: String,
    /// Available bytes to current user
    pub avail: Option<u64>,
    /// Free bytes
    pub free: Option<u64>,
    /// Size in bytes
    pub size: Option<u64>,
    /// Name
    pub name: Option<String>,
    /// Format (NTFS, FAT, ext4, ...)
    pub format: Option<String>,
    /// Read only
    pub readonly: Option<bool>,
    /// True if this mount point is likely to not be important
    pub dummy: bool,
}

pub use sys::{mountinfos, mountpaths, Error};
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOT: &str = if cfg!(target_os = "windows") {
        "C:\\"
    } else {
        "/"
    };

    #[test]
    fn mountpaths_works() {
        let paths = mountpaths().unwrap();
        assert!(paths.len() > 0);
        assert!(paths.iter().any(|p| p.as_str() == ROOT));

        for mountpath in &paths {
            eprintln!("{}", mountpath);
        }
    }
    #[test]
    fn mountinfosworks() {
        let infos = mountinfos().unwrap();
        assert!(infos.len() > 0);
        assert!(infos.iter().any(|i| if i.path.as_str() == ROOT {
            assert!(i.size.unwrap_or_default() > 1024 * 1024); // > 1Mb
            assert!(i.avail.unwrap_or_default() < i.size.unwrap_or_default());
            assert!(i.free.unwrap_or_default() < i.size.unwrap_or_default());
            true
        } else {
            false
        }));
        for mountinfo in &infos {
            eprintln!("{:?}", mountinfo);
        }
    }
}
