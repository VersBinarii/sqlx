use super::Decode;
use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;

use std::io;

type ObjectId = u32;

#[derive(Debug)]
pub struct ParameterDescription {
    ids: Vec<ObjectId>,
}

impl ParameterDescription {
    pub fn decode2(src: &[u8]) -> Self {
        let count = BigEndian::read_u16(&*src) as usize;

        // todo: error handling
        assert_eq!(src.len(), count * 4 + 2);

        let mut ids = Vec::with_capacity(count);
        for i in 0..count {
            let offset = i * 4 + 2; // 4==size_of(u32), 2==size_of(u16)
            ids.push(BigEndian::read_u32(&src[offset..]));
        }

        ParameterDescription { ids }
    }
}

#[cfg(test)]
mod test {
    use super::{Decode, ParameterDescription};
    use bytes::Bytes;
    use std::io;

    #[test]
    fn it_decodes_parameter_description() -> io::Result<()> {
        let src = b"\x00\x02\x00\x00\x00\x00\x00\x00\x05\x00";
        let desc = ParameterDescription::decode2(src);

        assert_eq!(desc.ids.len(), 2);
        assert_eq!(desc.ids[0], 0x0000_0000);
        assert_eq!(desc.ids[1], 0x0000_0500);
        Ok(())
    }

    #[test]
    fn it_decodes_empty_parameter_description() -> io::Result<()> {
        let src = b"\x00\x00";
        let desc = ParameterDescription::decode2(src);

        assert_eq!(desc.ids.len(), 0);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn parameter_description_wrong_length_fails() -> () {
        let src = b"\x00\x00\x00\x01\x02\x03";
        ParameterDescription::decode2(src);
    }
}