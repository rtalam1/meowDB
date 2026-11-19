use anyhow::{Result, bail};

#[derive(Debug)]
pub struct DbHeader {
    pub page_size: u32,
    pub reserved_bytes: u8,
    pub page_count: u32,
    pub text_encoding: u32,
}

impl DbHeader {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 100 {
            bail!("header too short");
        }
        if &bytes[0..16] != b"SQLite format 3\0" {
            bail!("not a SQLite database");
        }

        // TODO: page size at bytes 16..18 (u16, big-endian; 1 means 65536)
        let page_size_slice = &bytes[16..18];
        let page_array: [u8; 2] = page_size_slice.try_into()?;
        let raw_page = u16::from_be_bytes(page_array);
        let page_size = if raw_page == 1 {
            65536
        } else {
            raw_page as u32
        };

        let reserved_bytes = bytes[20];
        let page_count_slice = &bytes[28..32];
        let page_count_array: [u8; 4] = page_count_slice.try_into()?;
        let page_count = u32::from_be_bytes(page_count_array);
        let text_encoding_slice = &bytes[56..60];
        let text_encoding_array: [u8; 4] = text_encoding_slice.try_into()?;
        let text_encoding = u32::from_be_bytes(text_encoding_array);

        Ok(DbHeader {
            page_size,
            reserved_bytes,
            page_count,
            text_encoding,
        })
    }
}
