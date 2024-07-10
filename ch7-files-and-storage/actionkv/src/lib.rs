#[macro_use]
extern crate serde_derive;

use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::crc32;

/// The workhorse when we want to use a type that behaves like a
/// String. It’s also one that can contain arbitrary binary data.
pub type ByteString = Vec<u8>;

/// For data that tends to be used as a string but happens to be in a
/// binary (raw bytes) form. Its text-based peer is the built-in str.
/// Unlike str, ByteStr is not guaranteed to contain valid UTF-8 text.
///
/// Both str and [u8] (or its alias ByteStr) are seen in the wild as
/// &str and &\[u8\] (or &ByteStr). These are both called slices.
pub type ByteStr = [u8];

/// Instructs the compiler to generate serialized code to enable
/// writing KeyValuePair data to disk.
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>, // Maintains a mapping between keys and file locations
}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let index = HashMap::new();

        Ok(ActionKV { f, index })
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        loop {
            let position = f.seek(SeekFrom::Current(0))?;

            let maybe_kv = ActionKV::process_record(&mut f);

            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => {
                            break;
                        }
                        _ => return Err(err),
                    }
                }
            };

            self.index.insert(kv.key, position);
        }

        Ok(())
    }

    pub fn seek_to_end(&mut self) -> io::Result<u64> {
        //let mut f = BufReader::new(&mut self.f);
        self.f.seek(SeekFrom::End(0))
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        // we need to wrap Option within Result to allow for the possibilities of I/O errors as well as missing values
        // occurring
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(position) => *position,
        };

        let kv = self.get_at(position)?;

        Ok(Some(ByteString::from(kv.value)))
    }

    pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&mut self.f);
        f.seek(SeekFrom::Start(position))?;
        let kv = ActionKV::process_record(&mut f)?;

        Ok(kv)
    }

    fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;

        let key_len = f.read_u32::<LittleEndian>()?;

        let val_len = f.read_u32::<LittleEndian>()?;

        let data_len = key_len + val_len;

        let mut data = ByteString::with_capacity(data_len as usize);

        {
            f.by_ref()
                .take(data_len as u64)
                .read_to_end(&mut data)?;
        }

        debug_assert_eq!(data.len(), data_len as usize);

        let checksum = crc32::checksum_ieee(&data);
        if checksum != saved_checksum {
            panic!("data corruption encountered ({:08x} != {:08x})", checksum, saved_checksum)
        }

        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair { key, value })
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;

        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    pub fn insert_but_ignore_index (&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let mut f = BufWriter::new(&mut self.f);

        let key_len = key.len();
        let val_len = value.len();
        let mut tmp = ByteString::with_capacity(key_len + val_len);

        for byte in key {
            tmp.push(*byte);
        }

        for byte in value {
            tmp.push(*byte);
        }

        let checksum = crc32::checksum_ieee(&tmp);

        let next_byte = SeekFrom::End(0);
        let current_position = f.seek(SeekFrom::Current(0))?;

        f.seek(next_byte)?;
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(val_len as u32)?;
        f.write_all(&mut tmp)?;

        Ok(current_position)
    }

    #[inline]
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    #[inline]
    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }
}
