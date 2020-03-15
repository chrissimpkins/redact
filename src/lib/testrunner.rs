use tempfile::tempfile;

use failure::Error;

use crate::lib::io::write_tempfile_get_filesize;

#[derive(Debug, Clone)]
struct RustSource {
    src: String,
    size: u64,
}

impl RustSource {
    fn new(src: &str) -> Self {
        Self {
            src: src.to_string(),
            size: RustSource::set_filesize_with_str(src),
        }
    }

    fn set_filesize_with_str(src: &str) -> u64 {
        match write_tempfile_get_filesize(src) {
            Ok(filesize) => filesize,
            Err(error) => panic!("{}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_mock_source() -> String {
        String::from("\nlet x = 1;\n")
    }

    fn get_mock_source_modified() -> String {
        String::from("\nlet x = 1;\nlet y = 2;\n")
    }

    #[test]
    fn test_rustsource_instantiation() {
        let rs = RustSource::new(&get_mock_source());
        assert_eq!(rs.src, get_mock_source());
        assert_eq!(rs.size, 12);
        let rs2 = RustSource::new(&get_mock_source_modified());
        assert_eq!(rs2.src, get_mock_source_modified());
        assert_eq!(rs2.size, 23);
    }
}
