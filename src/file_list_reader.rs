use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;


/// Chains multiple files together in a single `Read` object.
///
/// The main advantage of this type compared to multiple `chain`ed `Read` values is, that only
/// one file is open at any given time.
///
/// # Examples
///
/// ```rust
/// use std::io::Read;
///
/// use resol_vbus::FileListReader;
///
/// let files: Vec<_> = std::env::args().skip(1).collect();
///
/// let mut flr = FileListReader::new(files);
///
/// let mut buf = [0u8; 4096];
///
/// loop {
///     let size = flr.read(&mut buf).unwrap();
///     if size == 0 {
///         break;
///     }
///
///     // process the data
/// }
/// ```
#[derive(Debug)]
pub struct FileListReader<T: AsRef<Path>> {
    file_list: Vec<T>,
    file_index: usize,
    file: Option<File>,
}


impl<T: AsRef<Path>> FileListReader<T> {

    /// Construct a new `FileListReader` from a list of paths.
    pub fn new(file_list: Vec<T>) -> FileListReader<T> {
        FileListReader {
            file_list: file_list,
            file_index: 0,
            file: None,
        }
    }
}


impl<T: AsRef<Path>> Read for FileListReader<T> {

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        loop {
            if let Some(ref mut file) = self.file {
                let size = file.read(buf)?;
                if size > 0 {
                    return Ok(size);
                }
            }

            if self.file_index >= self.file_list.len() {
                return Ok(0)
            } else {
                let file = File::open(&self.file_list [self.file_index])?;
                self.file = Some(file);
                self.file_index += 1;
            }
        }
    }

}
