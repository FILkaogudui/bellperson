use log::info;
use std::fs::File;
use std::path::PathBuf;
use fs2::FileExt;

const C2_MEM_LOCK_NAME: &str = "c2_mem.lock";
fn tmp_path(filename: &str) -> PathBuf {
    let mut p = PathBuf::new();
    p.push("/var/");
    p.push(filename);
    p
}

/// `C2MemLock` prevents two kernel objects to be instantiated simultaneously.
#[derive(Debug)]
pub struct C2MemLock(File);
impl C2MemLock {
    pub fn lock() -> C2MemLock {
        info!("Acquiring C2 Mem lock...");
        let f = File::create(tmp_path(C2_MEM_LOCK_NAME)).unwrap();
        f.lock_exclusive().unwrap();
        info!("seal C2 Mem lock acquired!");
        C2MemLock(f)
    }
}
impl Drop for C2MemLock {
    fn drop(&mut self) {
        info!("seal FFT lock released!");
    }
}