use std::{error, net, io};

type ErrorResult<T> = Result<T, Box<dyn error::Error>>;

mod packet;

pub use packet::{Packet, PacketType};

pub struct RconClient<A> {
    host: A,
    password: String,
    connection_input: Option<Box<dyn io::Read>>,
    connection_output: Option<Box<dyn io::Write>>,
    has_authenticated: bool
}

impl<A> RconClient<A>
where
    A: net::ToSocketAddrs + Copy
{
    pub fn new(host: A, password: String) -> RconClient<A> {
        RconClient {
            host,
            password,
            connection_input: None,
            connection_output: None,
            has_authenticated: false
        }
    }

    pub fn connect(&mut self) -> ErrorResult<()> {
        if let Some(_) = self.connection_input {
            return Err("Connection already open")?;
        }

        //Create the TCP connection
        let connection_input = net::TcpStream::connect(self.host)?;

        //Duplicate the stream to get the output
        let connection_output = connection_input.try_clone()?;

        self.connection_input = Some(Box::new(connection_input));
        self.connection_output = Some(Box::new(connection_output));

        Ok(())
    }

    /// Manually sets the connection input
    pub fn set_connection_input(&mut self, conn: Box<dyn io::Read>) {
        self.connection_input = Some(conn);
    }

    /// Manually sets the connection output
    pub fn set_connection_output(&mut self, conn: Box<dyn io::Write>) {
        self.connection_output = Some(conn);
    }

    pub fn authenticate(&mut self) -> ErrorResult<()> {
        if self.has_authenticated {
            return Err("Client has already authenticated")?;
        }

        let id: i32 = 527;

        //Send the auth packet
        let auth_packet = packet::Packet::new_with_id(id, packet::PacketType::ServerDataAuth, self.password.clone());

        self.send_packet(auth_packet)?;

        //Read the empty packet and then the response packet
        // self.read_packet()?;
        let response = self.read_packet()?;

        if response.id != id && response.id == -1 {
            return Err("Incorrect password")?;
        }

        Ok(())
    }

    pub fn read_packet(&mut self) -> ErrorResult<packet::Packet> {
        if let Some(connection) = self.connection_input.as_mut() {
            Packet::read_from(connection)
        }else {
            Err("Connection not open")?
        }
    }

    pub fn send_packet(&mut self, packet: packet::Packet) -> ErrorResult<()> {
        if let Some(connection) = self.connection_output.as_mut() {
            packet.write_to(connection)
        }else {
            Err("Connection not open")?
        }
    }
}