use std::{io::{self, Write}, error::Error, process};

fn main() {
    println!("Starting...");

    let mut host = String::new();

    //Read the host
    if let Err(e) = read_line_with_prompt(String::from("Host: "), &mut host) {
        eprintln!("Could not read host. Error: {e}");
        process::exit(1);
    }

    let mut password = String::new();

    //Read the password
    if let Err(e) = read_line_with_prompt(String::from("Password: "),&mut password) {
        eprintln!("Could not read password. Error: {e}");
        process::exit(1);
    }

    let mut client = rustyrcon::RCONClient::new(&host, password);

    if let Err(e) = client.connect() {
        eprintln!("Could not establish a connection to the server. Error: {e}");
        process::exit(1);
    }

    if let Err(e) = client.authenticate() {
        eprintln!("Error during authentication. Error: {e}");
        process::exit(1);
    }

    loop {
        let mut command = String::new();
    
        //Read the command
        if let Err(e) = read_line(&mut command) {
            eprintln!("Could not read command. Error: {e}");
            process::exit(1);
        }

        if command.to_lowercase() == "quit" {
            break;
        }

        //Send the command
        if let Err(e) = client.send_packet(rustyrcon::Packet::new(rustyrcon::PacketType::ServerDataAuthResponseOrExecCommand, command)) {
            eprintln!("Error sending command. Error: {e}");
            process::exit(1);
        }
    
        //Read the response
        match client.read_packet() {
            Ok(packet) => println!("{}", packet.body),
            Err(e) => {
                eprintln!("Error reading response from server. Error: {e}");
                process::exit(1);
            }
        }
    }
}

fn read_line_with_prompt(prompt: String, buf: &mut String) -> Result<(), Box<dyn Error>> {
    print!("{}", prompt);

    io::stdout().flush()?;

    io::stdin().read_line(buf)?;

    *buf = String::from(buf.trim());

    Ok(())
}

fn read_line(buf: &mut String) -> Result<(), Box<dyn Error>> {
    read_line_with_prompt(String::from("> "), buf)
}



