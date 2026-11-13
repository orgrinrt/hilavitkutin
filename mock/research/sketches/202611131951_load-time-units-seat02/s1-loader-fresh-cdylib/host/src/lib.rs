//! Spike host. The checks are in `tests/`; this file only names the shipped error
//! variants, which carry no `Debug`.

use hilavitkutin_extensions::ExtensionError;
use hilavitkutin_linking::LinkError;

pub fn link_label(e: &LinkError) -> String {
    match e {
        LinkError::PathNotFound => "PathNotFound".into(),
        LinkError::LoadFailed { platform_code } => format!("LoadFailed {{ platform_code: {} }}", platform_code.0),
        LinkError::SymbolMissing => "SymbolMissing".into(),
        LinkError::PlatformMismatch => "PlatformMismatch".into(),
        LinkError::IncompatibleVersion => "IncompatibleVersion".into(),
        LinkError::PathEncodingUnsupported => "PathEncodingUnsupported".into(),
        _ => "another LinkError".into(),
    }
}

pub fn ext_label(e: &ExtensionError) -> String {
    match e {
        ExtensionError::LinkFailed { cause } => format!("LinkFailed {{ cause: {} }}", link_label(cause)),
        ExtensionError::DescriptorMissing => "DescriptorMissing".into(),
        ExtensionError::DescriptorInvalid => "DescriptorInvalid".into(),
        _ => "another ExtensionError".into(),
    }
}
