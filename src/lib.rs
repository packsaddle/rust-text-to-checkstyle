extern crate checkstyle_formatter as checkstyle;

use std::error::Error;

pub fn run(
    message: &str,
    column: u32,
    line: u32,
    severity: &str,
    source: &str,
    name: &str,
) -> Result<String, Box<Error>> {
    let piece = checkstyle::ErrorPiece {
        column,
        line,
        message: message.to_owned(),
        severity: severity.to_owned(),
        source: source.to_owned(),
    };
    let file = checkstyle::ErrorFile {
        name: name.to_owned(),
        error_pieces: vec![piece],
    };
    let container = checkstyle::Container { error_files: vec![file] };
    return container.to_xml();
}
