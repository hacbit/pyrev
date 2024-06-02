use byteorder::{BigEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use pyrev_core::prelude::*;
use pyrev_marshal::loads;
use pyrev_object::PyObject;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// For PyInstaller 2.0
pub const PYINST20_COOKIE_SIZE: usize = 0x18;
/// For PyInstaller 2.1+
pub const PYINST21_COOKIE_SIZE: usize = 0x58;
/// Magic number which identifies a PyInstaller archive
pub const PYINST_MAGIC: &[u8] = b"MEI\x0C\x0B\x0A\x0B\x0E";

pub struct CTOCEntry {
    pub pos: u32,
    pub compressed_data_size: u32,
    pub uncompressed_data_size: u32,
    pub compress_flag: u8,
    pub compress_type: u8,
    pub name: String,
}

#[derive(Default)]
pub enum PyInstVersion {
    /// PyInstaller 2.0
    V20,
    #[default]
    /// PyInstaller 2.1+
    V21,
}

#[derive(Default)]
pub struct PyInstArchive {
    pub path: PathBuf,
    pub version: PyInstVersion,
    pub python_version: u32,
    pub file_ptr: Option<File>,
    pub file_size: u64,
    pub overlay_size: u32,
    pub overlay_pos: u32,
    /// table of contents position
    pub toc_pos: u32,
    /// table of contents size
    pub toc_size: u32,
    pub toc: Vec<CTOCEntry>,
}

impl PyInstArchive {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    pub fn open(&mut self) -> Result<&mut Self> {
        self.file_ptr = Some(File::open(&self.path)?);
        self.file_size = self.file_ptr.as_ref().unwrap().metadata()?.len();
        Ok(self)
    }

    pub fn close(&mut self) -> Result<()> {
        self.file_ptr = None;
        Ok(())
    }

    pub fn check_file(&mut self) -> Result<&mut Self> {
        info!("Checking file: {}", self.path.display());
        if let Some(file) = self.file_ptr.as_mut() {
            // Check whether the file is a 2.0 PyInstaller archive
            file.seek(SeekFrom::Start(
                self.file_size - PYINST20_COOKIE_SIZE as u64,
            ))?;
            let mut magic_from_file = [0u8; PYINST_MAGIC.len()];
            file.read_exact(&mut magic_from_file)?;

            if magic_from_file == PYINST_MAGIC {
                info!("PyInstaller version: 2.0");
                self.version = PyInstVersion::V20;
                return Ok(self);
            }

            // Check whether the file is a 2.1+ PyInstaller archive
            file.seek(SeekFrom::Start(
                self.file_size - PYINST21_COOKIE_SIZE as u64,
            ))?;
            let mut magic_from_file = [0u8; PYINST_MAGIC.len()];
            file.read_exact(&mut magic_from_file)?;

            if magic_from_file == PYINST_MAGIC {
                info!("PyInstaller version: 2.1+");
                self.version = PyInstVersion::V21;
                return Ok(self);
            }

            return Err("Unsupported PyInstaller version or Not a PyInstaller archive".into());
        }
        Err("File not opened".into())
    }

    pub fn get_c_archive_info(&mut self) -> Result<&mut Self> {
        match self.version {
            PyInstVersion::V20 => {
                if let Some(file) = self.file_ptr.as_mut() {
                    file.seek(SeekFrom::Start(
                        self.file_size - PYINST20_COOKIE_SIZE as u64,
                    ))?;

                    // Read CArchive cookie
                    let mut magic = [0u8; PYINST_MAGIC.len()];
                    file.read_exact(&mut magic)?;

                    self.overlay_size = file.read_u32::<BigEndian>()?;
                    self.overlay_pos = self.file_size as u32 - self.overlay_size;
                    self.toc_pos = file.read_u32::<BigEndian>()? + self.overlay_pos;
                    self.toc_size = file.read_u32::<BigEndian>()?;
                    self.python_version = file.read_u32::<BigEndian>()?;
                } else {
                    return Err("File not opened".into());
                }
            }
            PyInstVersion::V21 => {
                if let Some(file) = self.file_ptr.as_mut() {
                    file.seek(SeekFrom::Start(
                        self.file_size - PYINST21_COOKIE_SIZE as u64,
                    ))?;

                    // Read CArchive cookie
                    let mut magic = [0u8; PYINST_MAGIC.len()];
                    file.read_exact(&mut magic)?;

                    self.overlay_size = file.read_u32::<BigEndian>()?;
                    self.overlay_pos = self.file_size as u32 - self.overlay_size;
                    self.toc_pos = file.read_u32::<BigEndian>()? + self.overlay_pos;
                    self.toc_size = file.read_u32::<BigEndian>()?;
                    self.python_version = file.read_u32::<BigEndian>()?;
                } else {
                    return Err("File not opened".into());
                }
            }
        }

        info!("Python version: {}", self.python_version);
        info!("Length of package: {}", self.overlay_size);

        Ok(self)
    }

    pub fn parse_toc(&mut self) -> Result<&mut Self> {
        // go to the table of contents
        if let Some(file) = self.file_ptr.as_mut() {
            file.seek(SeekFrom::Start(self.toc_pos as u64))?;

            let mut toc = Vec::new();
            let mut parsed_len = 0;

            while parsed_len < self.toc_size {
                let entry_size = file.read_u32::<BigEndian>()?;
                let name_len = 4 * 4 + 1 + 1;

                let entry: CTOCEntry = CTOCEntry {
                    pos: self.overlay_pos + file.read_u32::<BigEndian>()?,
                    compressed_data_size: file.read_u32::<BigEndian>()?,
                    uncompressed_data_size: file.read_u32::<BigEndian>()?,
                    compress_flag: file.read_u8()?,
                    compress_type: file.read_u8()?,
                    name: {
                        let mut name_buf = vec![0u8; entry_size as usize - name_len];
                        file.read_exact(&mut name_buf)?;
                        let mut name = String::from_utf8(name_buf)?.trim_matches('\0').to_string();
                        if name.is_empty() {
                            // generate a random uuid as name
                            name = Uuid::new_v4().to_string();
                            warn!("Empty name found, using random uuid: {}", name);
                        }

                        name
                    },
                };
                toc.push(entry);
                parsed_len += entry_size;
            }

            self.toc = toc;

            info!("Found {} entries in the table of contents", self.toc.len());
        }

        Ok(self)
    }

    pub fn extract_files(&mut self) -> Result<&mut Self> {
        info!("Extracting files...");
        let current_dir = std::env::current_dir()?;
        let extraction_dir = current_dir.join(format!(
            "{}_extracted",
            self.path.file_stem().unwrap().to_str().unwrap()
        ));
        if !extraction_dir.exists() {
            std::fs::create_dir(&extraction_dir)?;
        }

        std::env::set_current_dir(&extraction_dir)?;

        if let Some(file) = self.file_ptr.as_mut() {
            for entry in &self.toc {
                let base_path = if let Some(path) = Path::new(&entry.name).parent() {
                    path
                } else {
                    error!("Invalid path: {}", entry.name);
                    continue;
                };

                if !base_path.as_os_str().is_empty() && !base_path.exists() {
                    std::fs::create_dir_all(base_path)?;
                }

                file.seek(SeekFrom::Start(entry.pos as u64))?;
                let mut data = vec![0u8; entry.compressed_data_size as usize];
                file.read_exact(&mut data)?;

                if entry.compress_flag == 1 {
                    let mut decoder = ZlibDecoder::new(&data[..]);
                    let mut decoded_data = Vec::new();
                    decoder.read_to_end(&mut decoded_data)?;
                    data = decoded_data;

                    assert_eq!(
                        entry.uncompressed_data_size as usize,
                        data.len(),
                        "Decompressed data size mismatch"
                    );
                }

                File::create(&entry.name)?.write_all(&data)?;

                if entry.compress_type == b's' {
                    info!("Possible entry point: {}", entry.name);
                } else if entry.compress_type.to_ascii_lowercase() == b'z' {
                    extract_pyz(&entry.name, self.python_version)?;
                }
            }
        }

        Ok(self)
    }
}

pub fn extract_pyz(pyz_name: &str, python_version: u32) -> Result<()> {
    let dir_name = PathBuf::from(format!("{}_extracted", pyz_name));
    if !dir_name.exists() {
        std::fs::create_dir(&dir_name)?;
    }

    let mut pyz_file = File::open(&pyz_name)?;
    let mut pyz_magic = [0u8; 4];
    pyz_file.read_exact(&mut pyz_magic)?;
    assert_eq!(&pyz_magic, b"PYZ\0", "Invalid PYZ magic");

    let mut pyc_header = [0u8; 4];
    pyz_file.read_exact(&mut pyc_header)?;

    let toc_pos = pyz_file.read_u32::<BigEndian>()?;
    pyz_file.seek(SeekFrom::Start(toc_pos as u64))?;

    let mut data = vec![];
    pyz_file.read_to_end(&mut data)?;
    let pyobject = loads(&data);

    if pyobject.is_null() {
        error!(
            "Unmarshalling FAILED. Cannot extract {}. Extracting remaining files.",
            pyz_name
        );
        return Ok(());
    }

    // From PyInstaller 3.1+ toc is a list of tuples
    let pyobject = if let PyObject::List(pylist) = pyobject {
        let mut dict = vec![];
        for pytuple in pylist {
            if let PyObject::Tuple(pytuple) = pytuple {
                dict.push((pytuple[0].clone(), pytuple[1].clone()));
            }
        }
        PyObject::Dict(dict)
    } else {
        pyobject
    };

    assert!(
        matches!(pyobject, PyObject::Dict(_)),
        "Invalid TOC, expected a dict"
    );

    info!("Found {} files in PYZ archive", pyobject.len());

    for (key, value) in pyobject.into_iter_items() {
        let (_ispkg, pos, length) = if let PyObject::Tuple(pytuple) = value {
            if pytuple.len() != 3 {
                error!("Invalid TOC entry, expected a tuple of length 3");
                continue;
            }
            (pytuple[0].clone(), pytuple[1].clone(), pytuple[2].clone())
        } else {
            error!("Invalid TOC entry, expected a tuple");
            continue;
        };
        let pos = match pos {
            PyObject::Int(pyint) => pyint as u64,
            PyObject::Int64(pyint64) => pyint64 as u64,
            _ => {
                error!("Invalid TOC entry, expected an integer");
                continue;
            }
        };
        let length = match length {
            PyObject::Int(pyint) => pyint as u64,
            PyObject::Int64(pyint64) => pyint64 as u64,
            _ => {
                error!("Invalid TOC entry, expected an integer");
                continue;
            }
        };
        pyz_file.seek(SeekFrom::Start(pos))?;

        let file_name = match key {
            PyObject::String(pystr) => {
                if let Ok(pystr) = String::from_utf8(pystr) {
                    pystr
                } else {
                    error!("Invalid TOC entry, expected an ascii string");
                    continue;
                }
            }
            PyObject::AsciiString(pystr) => pystr.clone(),
            _ => {
                error!("Invalid TOC entry, expected an ascii string");
                continue;
            }
        };

        let dest_name = dir_name.join(file_name.replace("..", "__"));
        let dest_dir_name = if let Some(dir) = dest_name.parent() {
            dir
        } else {
            error!("Invalid path: {}", file_name);
            continue;
        };

        if !dest_dir_name.exists() {
            std::fs::create_dir_all(dest_dir_name)?;
        }

        let mut data = vec![0u8; length as usize];
        pyz_file.read_exact(&mut data)?;

        let mut decoder = ZlibDecoder::new(&data[..]);
        let mut decoded_data = Vec::new();
        decoder.read_to_end(&mut decoded_data)?;

        let mut pyc_file = File::create(format!("{}.pyc", dest_name.display()))?;
        pyc_file.write(&pyc_header)?;
        pyc_file.write(b"\0\0\0\0")?;
        if python_version >= 33 {
            pyc_file.write(b"\0\0\0\0")?;
        }
        pyc_file.write(&decoded_data)?;
        pyc_file.sync_all()?;
    }

    Ok(())
}

pub fn extract_pyinstaller_archive<P: AsRef<Path>>(path: P) -> Result<()> {
    PyInstArchive::new(path.as_ref().to_path_buf())
        .open()?
        .check_file()?
        .get_c_archive_info()?
        .parse_toc()?
        .extract_files()?
        .close()?;

    info!(
        "Successfully extracted PyInstaller archive: {}",
        path.as_ref().display()
    );
    info!("You can now use a python decompiler on the pyc files within the extracted directory.");

    Ok(())
}
