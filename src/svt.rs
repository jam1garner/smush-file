use svt::SvtFile;
use std::io::Cursor;

pub(crate) fn info(contents: &[u8]) -> String {
    format!(
        "Smash Ultimate Sound Volume Table File\n{}",
        serde_yaml::to_string(
            &SvtFile::read(&mut Cursor::new(contents)).unwrap()
        ).unwrap_or_else(|_| "Invalid svt file".to_owned())
    )
}
