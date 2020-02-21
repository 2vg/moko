use rand::Rng;
use rand::distributions::Alphanumeric;
use std::path::Path;

pub fn generate_random_string(size: usize) -> String {
    rand::thread_rng().sample_iter(Alphanumeric).take(size).collect::<String>()
}

pub fn get_file_ext(file_name: &str) -> String {
    use std::ffi::OsStr;
    Path::new(file_name).extension().unwrap_or(OsStr::new("")).to_string_lossy().into_owned()
}
