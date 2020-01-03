use crate::io::BufMut;
use crate::postgres::protocol::Encode;
use byteorder::NetworkEndian;

pub struct SaslInitialResponse {
    // pub username: String,
    // pub passord: String,
    pub s: String,
}

impl Encode for SaslInitialResponse {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(b'p');
        buf.put_u32::<NetworkEndian>(4u32 + self.s.as_str().as_bytes().len() as u32 + 14u32 + 4u32);
        buf.put_str_nul("SCRAM-SHA-256");
        buf.put_u32::<NetworkEndian>(self.s.as_str().as_bytes().len() as u32);
        buf.extend_from_slice(self.s.as_str().as_bytes());
    }
}

pub struct SaslResponse {
    pub s: String,
}

impl Encode for SaslResponse {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(b'p');
        buf.put_u32::<NetworkEndian>(4u32 + self.s.as_str().as_bytes().len() as u32);
        buf.extend_from_slice(self.s.as_str().as_bytes());
    }
}
