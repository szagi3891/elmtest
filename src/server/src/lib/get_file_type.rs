use std::path::Path;
use lib::response_type::ResponseType;
use std::ffi::OsStr;

pub fn get_file_type(path: &str) -> Option<ResponseType> {

    match Path::new(path).extension() {
        Some(ext) => Some(get_ext(ext)),
        None => None,
    }
}

fn get_ext(ext: &OsStr) -> ResponseType {
    match ext.to_str() {
        Some("html") => ResponseType::Html,
        Some("css") => ResponseType::Css,
        Some("js") => ResponseType::Js,
        Some("ico") => ResponseType::Ico,
        //txt Type::TextPlain
        //jpg Type::ImageJpeg
        //png Type::ImagePng
        Some(_) => ResponseType::ServerError,
        None => ResponseType::ServerError
    }
}
