use fnv::FnvFile;
use std::io::Cursor;

pub(crate) fn info(contents: &[u8]) -> String {
    format!(
        "Smash Ultimate 'Volume by Fighter Count' File\n{}",
        serde_yaml::to_string(
            &FnvFile::read(&mut Cursor::new(contents)).unwrap()
        ).unwrap_or_else(|_| "Invalid fnv file".to_owned())
    )
}
