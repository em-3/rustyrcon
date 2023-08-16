use super::*;

#[test]
fn new_packet_data_auth() {
    let packet = Packet::new(PacketType::ServerDataAuth, String::from("APassword"));

    assert_eq!(packet.packet_type, PacketType::ServerDataAuth);
    assert_eq!(packet.body, String::from("APassword"));
}

#[test]
fn new_packet_auth_response() {
    let packet = Packet::new(PacketType::ServerDataAuthResponseOrExecCommand, String::from("command"));

    assert_eq!(packet.packet_type, PacketType::ServerDataAuthResponseOrExecCommand);
    assert_eq!(packet.body, String::from("command"));
}

#[test]
fn new_packet_response_value() {
    let packet = Packet::new(PacketType::ServerDataResponseValue, String::from("Executed command"));

    assert_eq!(packet.packet_type, PacketType::ServerDataResponseValue);
    assert_eq!(packet.body, String::from("Executed command"));
}

#[test]
fn new_packet_invalid_type() {
    let packet = Packet::new(PacketType::InvalidType, String::from("Error: Invalid type"));

    assert_eq!(packet.packet_type, PacketType::InvalidType);
    assert_eq!(packet.body, String::from("Error: Invalid type"));
}

#[test]
fn new_packet_with_id_data_auth() {
    let id = 527;

    let packet = Packet::new_with_id(id, PacketType::ServerDataAuth, String::from("APassword"));

    assert_eq!(packet.id, id);
    assert_eq!(packet.packet_type, PacketType::ServerDataAuth);
    assert_eq!(packet.body, String::from("APassword"));
}

#[test]
fn new_packet_with_id_auth_response() {
    let id = 527;

    let packet = Packet::new_with_id(id, PacketType::ServerDataAuthResponseOrExecCommand, String::from("command"));

    assert_eq!(packet.id, id);
    assert_eq!(packet.packet_type, PacketType::ServerDataAuthResponseOrExecCommand);
    assert_eq!(packet.body, String::from("command"));
}

#[test]
fn new_packet_with_id_response_value() {
    let id = 527;

    let packet = Packet::new_with_id(id, PacketType::ServerDataResponseValue, String::from("Executed command"));

    assert_eq!(packet.id, id);
    assert_eq!(packet.packet_type, PacketType::ServerDataResponseValue);
    assert_eq!(packet.body, String::from("Executed command"));
}

#[test]
fn new_packet_with_id_invalid_type() {
    let id = 527;

    let packet = Packet::new_with_id(id, PacketType::InvalidType, String::from("Error: Invalid type"));

    assert_eq!(packet.id, id);
    assert_eq!(packet.packet_type, PacketType::InvalidType);
    assert_eq!(packet.body, String::from("Error: Invalid type"));
}

#[test]
fn packet_type_from_i32_data_auth() {
    let input = 3; // Data auth value

    assert_eq!(PacketType::from(input), PacketType::ServerDataAuth);
}

#[test]
fn packet_type_from_i32_auth_response() {
    let input = 2; // Auth response value

    assert_eq!(PacketType::from(input), PacketType::ServerDataAuthResponseOrExecCommand);
}

#[test]
fn packet_type_display_data_auth() {
    let packet_type = PacketType::ServerDataAuth;

    assert_eq!(packet_type.to_string(), "Server Data Auth (3)");
}

#[test]
fn packet_type_display_auth_response() {
    let packet_type = PacketType::ServerDataAuthResponseOrExecCommand;

    assert_eq!(packet_type.to_string(), "Server Data Auth Response or Execute Command (2)");
}

#[test]
fn packet_type_display_response_value() {
    let packet_type = PacketType::ServerDataResponseValue;

    assert_eq!(packet_type.to_string(), "Server Data Response Value (0)");
}

#[test]
fn packet_type_display_invalid_type() {
    let packet_type = PacketType::InvalidType;

    assert_eq!(packet_type.to_string(), "Invalid Type (-1)");
}

#[test]
fn packet_type_from_i32_response_value() {
    let input = 0; // Response value value

    assert_eq!(PacketType::from(input), PacketType::ServerDataResponseValue);
}

#[test]
fn packet_type_from_i32_invalid_type() {
    let input = 4; // Invalid value

    assert_eq!(PacketType::from(input), PacketType::InvalidType);
}

#[test]
fn packet_type_into_i32_data_auth() {
    let packet_type = PacketType::ServerDataAuth;

    assert_eq!(<PacketType as Into<i32>>::into(packet_type), 3);
}

#[test]
fn packet_type_into_i32_auth_response() {
    let packet_type = PacketType::ServerDataAuthResponseOrExecCommand;

    assert_eq!(<PacketType as Into<i32>>::into(packet_type), 2);
}

#[test]
fn packet_type_into_i32_response_value() {
    let packet_type = PacketType::ServerDataResponseValue;

    assert_eq!(<PacketType as Into<i32>>::into(packet_type), 0);
}

#[test]
fn packet_type_into_i32_invalid_type() {
    let packet_type = PacketType::InvalidType;

    assert_eq!(<PacketType as Into<i32>>::into(packet_type), -1);
}

#[test]
fn packet_type_integer_addition() {
    let packet_type = PacketType::ServerDataAuth;

    assert_eq!(packet_type + 5, 8);
}