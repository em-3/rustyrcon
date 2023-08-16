use std::{error, net, io::Write};
use byteorder::{self, ReadBytesExt, WriteBytesExt};
use rand;

type ErrorResult<T> = Result<T, Box<dyn error::Error>>;

mod packet;

pub use packet::{Packet, PacketType};

pub struct RCONClient<'a, I: Iterator<Item = net::SocketAddr>> {
    host: &'a dyn net::ToSocketAddrs<Iter = I>,
    password: String,
    connection: Option<net::TcpStream>,
    has_authenticated: bool
}

impl<'a, I> RCONClient<'a, I>
    where
    I: Iterator<Item = net::SocketAddr> {
    pub fn new(host: &'a impl net::ToSocketAddrs<Iter = I>, password: String) -> RCONClient<I> {
        RCONClient {
            host,
            password,
            connection: None,
            has_authenticated: false
        }
    }

    pub fn connect(&mut self) -> ErrorResult<()> {
        if let Some(_) = self.connection {
            return Err("Connection already open")?;
        }

        self.connection = Some(net::TcpStream::connect(self.host)?);

        Ok(())
    }

    pub fn authenticate(&mut self) -> ErrorResult<()> {
        if self.has_authenticated {
            return Err("Client has already authenticated")?;
        }

        let id: i32 = rand::random();

        //Send the auth packet
        let auth_packet = packet::Packet::new_with_id(id, packet::PacketType::ServerDataAuth, self.password.clone());

        self.send_packet(auth_packet)?;

        //Read the empty packet and then the response packet
        // self.read_packet()?;
        let response = self.read_packet()?;

        if response.id == -1 {
            return Err("Incorrect password")?;
        }

        Ok(())
    }

    pub fn read_packet(&mut self) -> ErrorResult<packet::Packet> {
        //Read the length of the packet
        let size = self.get_connection()?.read_i32::<byteorder::LittleEndian>()?;

        //Read the ID
        let id = self.get_connection()?.read_i32::<byteorder::LittleEndian>()?;

        //Read the type
        let packet_type = self.get_connection()?.read_i32::<byteorder::LittleEndian>()?;

        let content_size = (size - 10) as usize;

        //Read the contents of the message
        let mut contents = Vec::with_capacity(content_size);

        for _ in 0..content_size {
            contents.push(self.get_connection()?.read_u8()?);
        }

        //Read the two empty bytes
        self.get_connection()?.read_u16::<byteorder::LittleEndian>()?;

        Ok(packet::Packet {
            size, id, packet_type: packet::PacketType::from(packet_type), body: String::from_utf8(contents)?
        })
    }

    pub fn send_packet(&mut self, packet: packet::Packet) -> ErrorResult<()> {
        //Write the packet values to the network
        self.get_connection()?.write_i32::<byteorder::LittleEndian>(packet.size)?;
        self.get_connection()?.write_i32::<byteorder::LittleEndian>(packet.id)?;
        self.get_connection()?.write_i32::<byteorder::LittleEndian>(packet.packet_type.into())?;

        self.get_connection()?.write_all(packet.body.as_bytes())?;

        //Write two empty bytes for null terminated string
        self.get_connection()?.write_u16::<byteorder::LittleEndian>(0)?;

        Ok(())
    }

    fn get_connection(&mut self) -> ErrorResult<&mut net::TcpStream> {        
        if let Some(connection) = self.connection.as_mut() {
            Ok(connection)
        }else {
            Err("Connection not open")?
        }
    }
}