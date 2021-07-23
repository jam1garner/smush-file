#[cfg(feature = "arc")]
use {
    smash_arc::{ArcFile, ArcLookup},
    std::path::Path
};

pub mod nus3audio;
pub mod nutexb;
pub mod ssbh;
pub mod prc;
pub mod sli;
pub mod csb;
pub mod sqb;
pub mod svt;
pub mod fnv;

mod c_api;

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Nutexb,
    Ssbh,
    Prc,
    Nus3audio,
    Sli,
    Csb,
    Sqb,
    Fnv,
    Svt,
    Unsupported,
}

impl Default for FileType {
    fn default() -> Self {
        FileType::Unsupported
    }
}

impl FileType {
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "nutexb" => Self::Nutexb,
            "nuhlpb" | "numatb" | "numdlb" | "nusrcmdlb" | "numshb" | "nusktb" | "nuanmb"
            | "nurpdb" | "nufxlb" | "nushdb" => Self::Ssbh,
            "prc" | "stdat" | "stprm" => Self::Prc,
            "nus3audio" => Self::Nus3audio,
            "sli" => Self::Sli,
            "csb" => Self::Csb,
            "sqb" => Self::Sqb,
            "fnv" => Self::Fnv,
            "svt" => Self::Svt,
            _ => Self::Unsupported,
        }
    }

    pub fn from_magic(contents: &[u8]) -> Self {
        match &contents[..4] {
            b"NUS3" => Some(Self::Nus3audio),
            b"SSBH" => Some(Self::Ssbh),
            b"CSB\0" => Some(Self::Csb),
            b"SLI\0" => Some(Self::Sli),
            b"FNV\0" => Some(Self::Fnv),
            b"SVT\0" => Some(Self::Svt),
            b"SQB\0" => Some(Self::Sqb),
            _ => None
        }.unwrap_or_else(|| match &contents[..8] {
            b"paracobn" => Some(Self::Prc),
            _ => None,
        }.unwrap_or_else(|| match &contents[contents.len() - 8..contents.len() - 4] {
            b" XET" => Some(Self::Nutexb),
            _ => None,
        }.unwrap_or_default()))
    }
}

#[cfg(feature = "arc")]
pub fn get_arc(arc: &ArcFile, path: &str) -> String {
    let extension = Path::new(path)
        .extension()
        .map(|x| x.to_string_lossy().into_owned());

    if let Some(ext) = extension {
        use FileType::*;

        let file_type = FileType::from_extension(&ext);
        if let Unsupported = file_type {
            format!("No info")
        } else {
            let contents = match arc.get_file_contents(path, smash_arc::Region::UsEnglish) {
                Ok(x) => x,
                Err(_) => return String::from("Could not open file"),
            };
            get_from_file_type(&contents, file_type)
        }
    } else {
        format!("Folder")
    }
}

pub fn get_from_extension(contents: &[u8], extension: impl AsRef<str>) -> String {
    get_from_file_type(contents, FileType::from_extension(extension.as_ref()))
}

pub fn get_from_magic(contents: &[u8]) -> String {
    get_from_file_type(contents, FileType::from_magic(contents))
}

pub fn get_from_file_type(contents: &[u8], file_type: FileType) -> String {
    use FileType::*;
    match file_type {
        Nutexb => nutexb::info(contents),
        Ssbh => ssbh::info(contents),
        Prc => prc::info(contents),
        Nus3audio => nus3audio::info(contents),
        Sli => sli::info(contents),
        Csb => csb::info(contents),
        Sqb => sqb::info(contents),
        Fnv => fnv::info(contents),
        Svt => svt::info(contents),
        _ => format!("No info"),
    }
}
