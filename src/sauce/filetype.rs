use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::sauce::{SauceError, DataType};
pub use std::str::FromStr;

#[derive(Deserialize, Serialize)]
pub enum FileType {
    ASCII,
    ANSI,
    ANSImation,
    RIPScript,
    PCBoard,
    Avatar,
    HTML,
    Source,
    TundraDraw,
    GIF,
    PCX,
    LBMOrIFF,
    TGA,
    FLI,
    FLC,
    BMP,
    GL,
    DL,
    WPGBitmap,
    PNG,
    JPG,
    MPG,
    AVI,
    DXF,
    DWG,
    WPGVector,
    Studio3DS,
    MOD,
    Renaissance669,
    STM,
    S3M,
    MTM,
    FAR,
    ULT,
    AMF,
    DMF,
    OKT,
    ROL,
    CMF,
    MID,
    SADT,
    VOC,
    WAV,
    SMP8,
    SMP8S,
    SMP16,
    SMP16S,
    PATCH8,
    PATCH16,
    XM,
    HSC,
    IT,
    Variable(u8),
    XBin,
    ZIP,
    ARJ,
    LZH,
    ARC,
    TAR,
    ZOO,
    RAR,
    UC2,
    PAK,
    SQZ,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileType::ASCII => write!(f, "ASCII"),
            FileType::ANSI => write!(f, "ANSI"),
            FileType::ANSImation => write!(f, "ANSImation"),
            FileType::RIPScript => write!(f, "RIPScript"),
            FileType::PCBoard => write!(f, "PCBoard"),
            FileType::Avatar => write!(f, "Avatar"),
            FileType::HTML => write!(f, "HTML"),
            FileType::Source => write!(f, "Source"),
            FileType::TundraDraw => write!(f, "TundraDraw"),
            FileType::GIF => write!(f, "GIF"),
            FileType::PCX => write!(f, "PCX"),
            FileType::LBMOrIFF => write!(f, "LBM/IFF"),
            FileType::TGA => write!(f, "TGA"),
            FileType::FLI => write!(f, "FLI"),
            FileType::FLC => write!(f, "FLC"),
            FileType::BMP => write!(f, "BMP"),
            FileType::GL => write!(f, "GL"),
            FileType::DL => write!(f, "DL"),
            FileType::WPGBitmap => write!(f, "WPG Bitmap"),
            FileType::PNG => write!(f, "PNG"),
            FileType::JPG => write!(f, "JPG"),
            FileType::MPG => write!(f, "MPG"),
            FileType::AVI => write!(f, "AVI"),
            FileType::DXF => write!(f, "DXF"),
            FileType::DWG => write!(f, "DWG"),
            FileType::WPGVector => write!(f, "WPG Vector"),
            FileType::Studio3DS => write!(f, "3DS"),
            FileType::MOD => write!(f, "MOD"),
            FileType::Renaissance669 => write!(f, "669"),
            FileType::STM => write!(f, "STM"),
            FileType::S3M => write!(f, "S3M"),
            FileType::MTM => write!(f, "MTM"),
            FileType::FAR => write!(f, "FAR"),
            FileType::ULT => write!(f, "ULT"),
            FileType::AMF => write!(f, "AMF"),
            FileType::DMF => write!(f, "DMF"),
            FileType::OKT => write!(f, "OKT"),
            FileType::ROL => write!(f, "ROL"),
            FileType::CMF => write!(f, "CMF"),
            FileType::MID => write!(f, "MID"),
            FileType::SADT => write!(f, "SADT"),
            FileType::VOC => write!(f, "VOC"),
            FileType::WAV => write!(f, "WAV"),
            FileType::SMP8 => write!(f, "SMP8"),
            FileType::SMP8S => write!(f, "SMP8S"),
            FileType::SMP16 => write!(f, "SMP16"),
            FileType::SMP16S => write!(f, "SMP16S"),
            FileType::PATCH8 => write!(f, "PATCH8"),
            FileType::PATCH16 => write!(f, "PATCH16"),
            FileType::XM => write!(f, "XM"),
            FileType::HSC => write!(f, "HSC"),
            FileType::IT => write!(f, "IT"),
            FileType::Variable(value) => write!(f, "Variable({})", value),
            FileType::XBin => write!(f, "XBin"),
            FileType::ZIP => write!(f, "ZIP"),
            FileType::ARJ => write!(f, "ARJ"),
            FileType::LZH => write!(f, "LZH"),
            FileType::ARC => write!(f, "ARC"),
            FileType::TAR => write!(f, "TAR"),
            FileType::ZOO => write!(f, "ZOO"),
            FileType::RAR => write!(f, "RAR"),
            FileType::UC2 => write!(f, "UC2"),
            FileType::PAK => write!(f, "PAK"),
            FileType::SQZ => write!(f, "SQZ"),
        }
    }
}

impl FileType {
    pub fn as_u8(&self) -> u8 {
        match self {
            FileType::ASCII => 0,
            FileType::GIF => 0,
            FileType::DXF => 0,
            FileType::MOD => 0,
            FileType::ZIP => 0,
            FileType::ANSI => 1,
            FileType::PCX => 1,
            FileType::DWG => 1,
            FileType::Renaissance669 => 1,
            FileType::ARJ => 1,
            FileType::ANSImation => 2,
            FileType::LBMOrIFF => 2,
            FileType::WPGVector => 2,
            FileType::STM => 2,
            FileType::LZH => 2,
            FileType::RIPScript => 3,
            FileType::TGA => 3,
            FileType::Studio3DS => 3,
            FileType::S3M => 3,
            FileType::ARC => 3,
            FileType::PCBoard => 4,
            FileType::FLI => 4,
            FileType::MTM => 4,
            FileType::TAR => 4,
            FileType::Avatar => 5,
            FileType::FLC => 5,
            FileType::FAR => 5,
            FileType::ZOO => 5,
            FileType::HTML => 6,
            FileType::BMP => 6,
            FileType::ULT => 6,
            FileType::RAR => 6,
            FileType::Source => 7,
            FileType::GL => 7,
            FileType::AMF => 7,
            FileType::UC2 => 7,
            FileType::TundraDraw => 8,
            FileType::DL => 8,
            FileType::DMF => 8,
            FileType::PAK => 8,
            FileType::WPGBitmap => 9,
            FileType::OKT => 9,
            FileType::SQZ => 9,
            FileType::PNG => 10,
            FileType::ROL => 10,
            FileType::JPG => 11,
            FileType::CMF => 11,
            FileType::MPG => 12,
            FileType::MID => 12,
            FileType::AVI => 13,
            FileType::SADT => 13,
            FileType::VOC => 14,
            FileType::WAV => 15,
            FileType::SMP8 => 16,
            FileType::SMP8S => 17,
            FileType::SMP16 => 18,
            FileType::SMP16S => 19,
            FileType::PATCH8 => 20,
            FileType::PATCH16 => 21,
            FileType::XM => 22,
            FileType::HSC => 23,
            FileType::IT => 24,
            FileType::Variable(value) => *value,
            FileType::XBin => 0,
        }
    }
}

impl FromStr for FileType {
    type Err = Box<SauceError>;
    fn from_str(string: &str) -> Result<FileType, Box<SauceError>> {
        match string {
            "ASCII" => Ok(FileType::ASCII),
            "ANSI" => Ok(FileType::ANSI),
            "ANSImation" => Ok(FileType::ANSImation),
            "RIPScript" => Ok(FileType::RIPScript),
            "PCBoard" => Ok(FileType::PCBoard),
            "Avatar" => Ok(FileType::Avatar),
            "HTML" => Ok(FileType::HTML),
            "Source" => Ok(FileType::Source),
            "TundraDraw" => Ok(FileType::TundraDraw),
            "GIF" => Ok(FileType::GIF),
            "PCX" => Ok(FileType::PCX),
            "LBM/IFF" => Ok(FileType::LBMOrIFF),
            "TGA" => Ok(FileType::TGA),
            "FLI" => Ok(FileType::FLI),
            "FLC" => Ok(FileType::FLC),
            "BMP" => Ok(FileType::BMP),
            "GL" => Ok(FileType::GL),
            "DL" => Ok(FileType::DL),
            "WPG Bitmap" => Ok(FileType::WPGBitmap),
            "PNG" => Ok(FileType::PNG),
            "JPG" => Ok(FileType::JPG),
            "MPG" => Ok(FileType::MPG),
            "AVI" => Ok(FileType::AVI),
            "DXF" => Ok(FileType::DXF),
            "DWG" => Ok(FileType::DWG),
            "WPG Vector" => Ok(FileType::WPGVector),
            "3DS" => Ok(FileType::Studio3DS),
            "MOD" => Ok(FileType::MOD),
            "669" => Ok(FileType::Renaissance669),
            "STM" => Ok(FileType::STM),
            "S3M" => Ok(FileType::S3M),
            "MTM" => Ok(FileType::MTM),
            "FAR" => Ok(FileType::FAR),
            "ULT" => Ok(FileType::ULT),
            "AMF" => Ok(FileType::AMF),
            "DMF" => Ok(FileType::DMF),
            "OKT" => Ok(FileType::OKT),
            "ROL" => Ok(FileType::ROL),
            "CMF" => Ok(FileType::CMF),
            "MID" => Ok(FileType::MID),
            "SADT" => Ok(FileType::SADT),
            "VOC" => Ok(FileType::VOC),
            "WAV" => Ok(FileType::WAV),
            "SMP8" => Ok(FileType::SMP8),
            "SMP8S" => Ok(FileType::SMP8S),
            "SMP16" => Ok(FileType::SMP16),
            "SMP16S" => Ok(FileType::SMP16S),
            "PATCH8" => Ok(FileType::PATCH8),
            "PATCH16" => Ok(FileType::PATCH16),
            "XM" => Ok(FileType::XM),
            "HSC" => Ok(FileType::HSC),
            "IT" => Ok(FileType::IT),
            "XBin" => Ok(FileType::XBin),
            "ZIP" => Ok(FileType::ZIP),
            "ARJ" => Ok(FileType::ARJ),
            "LZH" => Ok(FileType::LZH),
            "ARC" => Ok(FileType::ARC),
            "TAR" => Ok(FileType::TAR),
            "ZOO" => Ok(FileType::ZOO),
            "RAR" => Ok(FileType::RAR),
            "UC2" => Ok(FileType::UC2),
            "PAK" => Ok(FileType::PAK),
            "SQZ" => Ok(FileType::SQZ),
            _ => {
                let regex = Regex::new(r"^Variable\((\d+)\)$").expect("illegal regex");
                if let Some(captures) = regex.captures(string) {
                    if let Some(capture) = captures.get(1) {
                        let value_string = capture.as_str();
                        if let Ok(value) = value_string.parse::<u8>() {
                            return Ok(FileType::Variable(value));
                        }
                    }
                }
                Err(Box::new(SauceError::InvalidFileType))
            },
        }
    }
}

pub trait AsFileType {
    fn as_filetype(&self, datatype: &Option<DataType>) -> Result<Option<FileType>, Box<SauceError>>;
}

impl AsFileType for u8 {
    fn as_filetype(&self, datatype: &Option<DataType>) -> Result<Option<FileType>, Box<SauceError>> {
        match datatype {
            Some(DataType::Character) => {
                match self {
                    0 => Ok(Some(FileType::ASCII)),
                    1 => Ok(Some(FileType::ANSI)),
                    2 => Ok(Some(FileType::ANSImation)),
                    3 => Ok(Some(FileType::RIPScript)),
                    4 => Ok(Some(FileType::PCBoard)),
                    5 => Ok(Some(FileType::Avatar)),
                    6 => Ok(Some(FileType::HTML)),
                    7 => Ok(Some(FileType::Source)),
                    8 => Ok(Some(FileType::TundraDraw)),
                    _ => Err(Box::new(SauceError::InvalidFileType)),
                }
            },
            Some(DataType::Bitmap) => {
                match self {
                    0 => Ok(Some(FileType::GIF)),
                    1 => Ok(Some(FileType::PCX)),
                    2 => Ok(Some(FileType::LBMOrIFF)),
                    3 => Ok(Some(FileType::TGA)),
                    4 => Ok(Some(FileType::FLI)),
                    5 => Ok(Some(FileType::FLC)),
                    6 => Ok(Some(FileType::BMP)),
                    7 => Ok(Some(FileType::GL)),
                    8 => Ok(Some(FileType::DL)),
                    9 => Ok(Some(FileType::WPGBitmap)),
                    10 => Ok(Some(FileType::PNG)),
                    11 => Ok(Some(FileType::JPG)),
                    12 => Ok(Some(FileType::MPG)),
                    13 => Ok(Some(FileType::AVI)),
                    _ => Err(Box::new(SauceError::InvalidFileType)),
                }
            },
            Some(DataType::Vector) => {
                match self {
                    0 => Ok(Some(FileType::DXF)),
                    1 => Ok(Some(FileType::DWG)),
                    2 => Ok(Some(FileType::WPGVector)),
                    3 => Ok(Some(FileType::Studio3DS)),
                    _ => Err(Box::new(SauceError::InvalidFileType)),
                }
            },
            Some(DataType::Audio) => {
                match self {
                    0 => Ok(Some(FileType::MOD)),
                    1 => Ok(Some(FileType::Renaissance669)),
                    2 => Ok(Some(FileType::STM)),
                    3 => Ok(Some(FileType::S3M)),
                    4 => Ok(Some(FileType::MTM)),
                    5 => Ok(Some(FileType::FAR)),
                    6 => Ok(Some(FileType::ULT)),
                    7 => Ok(Some(FileType::AMF)),
                    8 => Ok(Some(FileType::DMF)),
                    9 => Ok(Some(FileType::OKT)),
                    10 => Ok(Some(FileType::ROL)),
                    11 => Ok(Some(FileType::CMF)),
                    12 => Ok(Some(FileType::MID)),
                    13 => Ok(Some(FileType::SADT)),
                    14 => Ok(Some(FileType::VOC)),
                    15 => Ok(Some(FileType::WAV)),
                    16 => Ok(Some(FileType::SMP8)),
                    17 => Ok(Some(FileType::SMP8S)),
                    18 => Ok(Some(FileType::SMP16)),
                    19 => Ok(Some(FileType::SMP16S)),
                    20 => Ok(Some(FileType::PATCH8)),
                    21 => Ok(Some(FileType::PATCH16)),
                    22 => Ok(Some(FileType::XM)),
                    23 => Ok(Some(FileType::HSC)),
                    24 => Ok(Some(FileType::IT)),
                    _ => Err(Box::new(SauceError::InvalidFileType)),
                }
            },
            Some(DataType::BinaryText) => Ok(Some(FileType::Variable(*self))),
            Some(DataType::XBin) => {
                if *self != 0 {
                    Err(Box::new(SauceError::InvalidFileType))
                } else {
                    Ok(Some(FileType::XBin))
                }
            },
            Some(DataType::Archive) => {
                match self {
                    0 =>Ok(Some(FileType::ZIP)),
                    1 =>Ok(Some(FileType::ARJ)),
                    2 =>Ok(Some(FileType::LZH)),
                    3 =>Ok(Some(FileType::ARC)),
                    4 =>Ok(Some(FileType::TAR)),
                    5 =>Ok(Some(FileType::ZOO)),
                    6 =>Ok(Some(FileType::RAR)),
                    7 =>Ok(Some(FileType::UC2)),
                    8 =>Ok(Some(FileType::PAK)),
                    9 =>Ok(Some(FileType::SQZ)),
                    _ => Err(Box::new(SauceError::InvalidFileType)),
                }
            },
            Some(DataType::Executable) | None => {
                if *self != 0 {
                    Err(Box::new(SauceError::InvalidFileType))
                } else {
                    Ok(None)
                }
            },
        }
    }
}
