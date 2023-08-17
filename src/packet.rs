#[cfg(test)]
mod tests;

use std::{fmt, io, ops, error};

use byteorder::{WriteBytesExt, ReadBytesExt, LittleEndian};

#[derive(Debug, PartialEq)]
pub struct Packet {
    pub size: i32,
    pub id: i32,
    pub packet_type: PacketType,
    pub body: String
}

impl Packet {
    pub fn new(packet_type: PacketType, body: String) -> Self {
        let id = rand::random();

        Self::new_with_id(id, packet_type, body)
    }

    pub fn new_with_id(id: i32, packet_type: PacketType, body: String) -> Self {
        // Packet size = 4 bytes (id) + 4 bytes (type) + x bytes (body length) + 2 bytes (null terminated strings)
        let packet_size: i32 = 4 + 4 + body.len() as i32 + 2;

        Packet {
            size: packet_size,
            id,
            packet_type: packet_type,
            body
        }
    }

    pub fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, Box<dyn error::Error>> {
        //Read the length of the packet
        let size = buf.read_i32::<LittleEndian>()?;

        //Read the ID
        let id = buf.read_i32::<LittleEndian>()?;

        //Read the type
        let packet_type = buf.read_i32::<LittleEndian>()?;

        let content_size = (size - 10) as usize;

        //Read the contents of the message
        let mut contents = Vec::with_capacity(content_size);

        for _ in 0..content_size {
            contents.push(buf.read_u8()?);
        }

        //Read the two empty bytes
        buf.read_u16::<LittleEndian>()?;

        Ok(Self {
            size, id, packet_type: PacketType::from(packet_type), body: String::from_utf8(contents)?
        })
    }

    pub fn write_to<T: io::Write>(self, buf: &mut T) -> Result<(), Box<dyn error::Error>> {
        //Write the contents of the packet
        buf.write_i32::<LittleEndian>(self.size)?;
        buf.write_i32::<LittleEndian>(self.id)?;
        buf.write_i32::<LittleEndian>(self.packet_type.into())?;
        buf.write_all(self.body.as_bytes())?;

        //Write two empty bytes for null terminated string
        buf.write_u16::<LittleEndian>(0)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum PacketType {
    ServerDataAuth,
    ServerDataAuthResponseOrExecCommand,
    ServerDataResponseValue,
    InvalidType
}

impl fmt::Display for PacketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::ServerDataAuth => "Server Data Auth (3)",
            Self::ServerDataAuthResponseOrExecCommand => "Server Data Auth Response or Execute Command (2)",
            Self::ServerDataResponseValue => "Server Data Response Value (0)",
            Self::InvalidType => "Invalid Type (-1)"
        })
    }
}

impl From<i32> for PacketType {
    fn from(value: i32) -> Self {
        match value {
            3 => Self::ServerDataAuth,
            2 => Self::ServerDataAuthResponseOrExecCommand,
            0 => Self::ServerDataResponseValue,
            _ => Self::InvalidType
        }
    }
}

impl Into<i32> for PacketType {
    fn into(self) -> i32 {
        match self {
            Self::ServerDataAuth => 3,
            Self::ServerDataAuthResponseOrExecCommand => 2,
            Self::ServerDataResponseValue => 0,
            Self::InvalidType => -1,
        }
    }
}

impl ops::Add<i32> for PacketType {
    type Output = i32;

    fn add(self, rhs: i32) -> Self::Output {
        <PacketType as Into<i32>>::into(self) + rhs
    }
}