use log::info;
use std::fs::File;
use std::path::PathBuf;
use fs2::FileExt;

const FFT_LOCK_NAME: &str = "FFT.lock";
fn tmp_path(filename: &str) -> PathBuf {
    let mut p = PathBuf::new();
    p.push("/var/");
    p.push(filename);
    p
}

/// `FFTLock` prevents two kernel objects to be instantiated simultaneously.
#[derive(Debug)]
pub struct FFTLock(File);
impl FFTLock {
    pub fn lock() -> FFTLock {
        info!("Acquiring FFT lock...");
        let f = File::create(tmp_path(FFT_LOCK_NAME)).unwrap();
        f.lock_exclusive().unwrap();
        info!("seal FFT lock acquired!");
        FFTLock(f)
    }
}
impl Drop for FFTLock {
    fn drop(&mut self) {
        info!("seal FFT lock released!");
    }
}