extern crate byteorder ;
# [ derive ( Debug ) ]
struct packet_AcknowledgePacket__packets__inner__values__0 {
    start: (),
    end: (),
}
# [ derive ( Debug ) ]
enum packet_AcknowledgePacket__packets__inner__values {
    _0(packet_AcknowledgePacket__packets__inner__values__0),
}
# [ derive ( Debug ) ]
struct packet_AcknowledgePacket__packets__inner {
    one: i8,
    values: packet_AcknowledgePacket__packets__inner__values,
}
# [ derive ( Debug ) ]
struct packet_AcknowledgePacket {
    packets: Vec<packet_AcknowledgePacket__packets__inner>,
}
# [ derive ( Debug ) ]
struct packet_advertise_system {
    pingID: i64,
    serverID: i64,
    magic: (),
    serverName: String,
}
# [ derive ( Debug ) ]
struct packet_client_connect {
    clientID: i64,
    sendPing: i64,
    useSecurity: i8,
    password: (),
}
# [ derive ( Debug ) ]
struct packet_client_disconnect {}
# [ derive ( Debug ) ]
struct address_port {
    version: i8,
    address: (),
    port: u16,
}
# [ derive ( Debug ) ]
struct packet_client_handshake {
    serverAddress: address_port,
    systemAddresses: Vec<address_port>,
}
# [ derive ( Debug ) ]
struct packet_open_connection_reply_1 {
    magic: (),
    serverID: i64,
    serverSecurity: i8,
    mtuSize: i16,
}
# [ derive ( Debug ) ]
struct packet_open_connection_reply_2 {
    magic: (),
    serverID: i64,
    clientAddress: address_port,
    mtuSize: i16,
    serverSecurity: i8,
}
# [ derive ( Debug ) ]
struct packet_open_connection_request_1 {
    magic: (),
    protocol: i8,
    mtuSize: (),
}
# [ derive ( Debug ) ]
struct packet_open_connection_request_2 {
    magic: (),
    serverAddress: address_port,
    mtuSize: i16,
    clientID: i64,
}
# [ derive ( Debug ) ]
struct packet_ping {
    pingID: i64,
}
# [ derive ( Debug ) ]
struct packet_pong {
    pingID: i64,
}
# [ derive ( Debug ) ]
struct packet_server_handshake {
    clientAddress: address_port,
    serverSecurity: i8,
    systemAddresses: Vec<address_port>,
    sendPing: i64,
    sendPong: i64,
}
# [ derive ( Debug ) ]
struct packet_unconnected_ping {
    pingID: i64,
    magic: (),
    unknown: i64,
}
# [ derive ( Debug ) ]
struct packet_unconnected_ping_open_connections {
    pingID: i64,
    magic: (),
}
# [ derive ( Debug ) ]
struct packet_unconnected_pong {
    pingID: i64,
    serverID: i64,
    magic: (),
    serverName: String,
}
# [ derive ( Debug ) ]
struct packet_data_packet {
    seqNumber: (),
    encapsulatedPackets: (),
}
# [ derive ( Debug ) ]
enum packet__params {
    ack(packet_AcknowledgePacket),
    nack(packet_AcknowledgePacket),
    advertise_system(packet_advertise_system),
    client_connect(packet_client_connect),
    client_disconnect(packet_client_disconnect),
    client_handshake(packet_client_handshake),
    open_connection_reply_1(packet_open_connection_reply_1),
    open_connection_reply_2(packet_open_connection_reply_2),
    open_connection_request_1(packet_open_connection_request_1),
    open_connection_request_2(packet_open_connection_request_2),
    ping(packet_ping),
    pong(packet_pong),
    server_handshake(packet_server_handshake),
    unconnected_ping(packet_unconnected_ping),
    unconnected_ping_open_connections(packet_unconnected_ping_open_connections),
    unconnected_pong(packet_unconnected_pong),
    data_packet_0(packet_data_packet),
    data_packet_1(packet_data_packet),
    data_packet_2(packet_data_packet),
    data_packet_3(packet_data_packet),
    data_packet_4(packet_data_packet),
    data_packet_5(packet_data_packet),
    data_packet_6(packet_data_packet),
    data_packet_7(packet_data_packet),
    data_packet_8(packet_data_packet),
    data_packet_9(packet_data_packet),
    data_packet_A(packet_data_packet),
    data_packet_B(packet_data_packet),
    data_packet_C(packet_data_packet),
    data_packet_D(packet_data_packet),
    data_packet_E(packet_data_packet),
    data_packet_F(packet_data_packet),
}
# [ derive ( Debug ) ]
struct packet {
    name: &'static str,
    params: packet__params,
}
fn read(read: &mut ::std::io::Read) -> std::io::Result<packet> {
    let res =
        {
            let name_1 = {
                match byteorder::ReadBytesExt::read_uint::<byteorder::BigEndian>(read, 1usize)? as
                      u8 {
                    0x00 => "ping",
                    0x01 => "unconnected_ping",
                    0x02 => "unconnected_ping_open_connections",
                    0x03 => "pong",
                    0x05 => "open_connection_request_1",
                    0x06 => "open_connection_reply_1",
                    0x07 => "open_connection_request_2",
                    0x08 => "open_connection_reply_2",
                    0x09 => "client_connect",
                    0x10 => "server_handshake",
                    0x13 => "client_handshake",
                    0x15 => "client_disconnect",
                    0x1c => "unconnected_pong",
                    0x1d => "advertise_system",
                    0x80 => "data_packet_0",
                    0x81 => "data_packet_1",
                    0x82 => "data_packet_2",
                    0x83 => "data_packet_3",
                    0x84 => "data_packet_4",
                    0x85 => "data_packet_5",
                    0x86 => "data_packet_6",
                    0x87 => "data_packet_7",
                    0x88 => "data_packet_8",
                    0x89 => "data_packet_9",
                    0x8a => "data_packet_A",
                    0x8b => "data_packet_B",
                    0x8c => "data_packet_C",
                    0x8d => "data_packet_D",
                    0x8e => "data_packet_E",
                    0x8f => "data_packet_F",
                    0xa0 => "nack",
                    0xc0 => "ack",
                    _ => panic!("WTF"),
                }
            };
            let params_1 =
                match name_1 {
                    "ack" => {
                        packet__params::ack({
                            let packets_2 = {
                                let count = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 2usize ) ? as i16 as usize ;
                                let mut v = Vec::with_capacity(count);
                                for i in 0..count {
                                    v . insert ( i , { let one_3 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ; let values_3 = match one_3 { "0" => packet_AcknowledgePacket__packets__inner__values :: _0 ( { let start_4 = ( ) ; let end_4 = ( ) ; packet_AcknowledgePacket__packets__inner__values__0 { start : start_4 , end : end_4 } } ) , _ => panic ! ( "WTF" ) } ; packet_AcknowledgePacket__packets__inner { one : one_3 , values : values_3 } } ) ;
                                }
                                v
                            };
                            packet_AcknowledgePacket { packets: packets_2 }
                        })
                    }
                    "nack" => {
                        packet__params::nack({
                            let packets_2 = {
                                let count = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 2usize ) ? as i16 as usize ;
                                let mut v = Vec::with_capacity(count);
                                for i in 0..count {
                                    v . insert ( i , { let one_3 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ; let values_3 = match one_3 { "0" => packet_AcknowledgePacket__packets__inner__values :: _0 ( { let start_4 = ( ) ; let end_4 = ( ) ; packet_AcknowledgePacket__packets__inner__values__0 { start : start_4 , end : end_4 } } ) , _ => panic ! ( "WTF" ) } ; packet_AcknowledgePacket__packets__inner { one : one_3 , values : values_3 } } ) ;
                                }
                                v
                            };
                            packet_AcknowledgePacket { packets: packets_2 }
                        })
                    }
                    "advertise_system" => {
                        packet__params::advertise_system({
                            let pingID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            let serverID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            let magic_2 = ();
                            let serverName_2 = {
                                let mut v = String :: with_capacity ( byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 2usize ) ? as i16 as usize ) ;
                                read.read_to_string(&mut v);
                                v
                            };
                            packet_advertise_system {
                                pingID: pingID_2,
                                serverID: serverID_2,
                                magic: magic_2,
                                serverName: serverName_2,
                            }
                        })
                    }
                    "client_connect" => {
                        packet__params::client_connect({
                            let clientID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            let sendPing_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            let useSecurity_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ;
                            let password_2 = ();
                            packet_client_connect {
                                clientID: clientID_2,
                                sendPing: sendPing_2,
                                useSecurity: useSecurity_2,
                                password: password_2,
                            }
                        })
                    }
                    "client_disconnect" => {
                        packet__params::client_disconnect({
                            packet_client_disconnect {}
                        })
                    }
                    "client_handshake" => {
                        packet__params::client_handshake({
                            let serverAddress_2 = {
                                let version_3 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ;
                                let address_3 = ();
                                let port_3 = byteorder :: ReadBytesExt :: read_uint :: < byteorder :: BigEndian > ( read , 2usize ) ? as u16 ;
                                address_port {
                                    version: version_3,
                                    address: address_3,
                                    port: port_3,
                                }
                            };
                            let systemAddresses_2 = {
                                let count = 10u64 as usize;
                                let mut v = Vec::with_capacity(count);
                                for i in 0..count {
                                    v . insert ( i , { let version_3 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ; let address_3 = ( ) ; let port_3 = byteorder :: ReadBytesExt :: read_uint :: < byteorder :: BigEndian > ( read , 2usize ) ? as u16 ; address_port { version : version_3 , address : address_3 , port : port_3 } } ) ;
                                }
                                v
                            };
                            packet_client_handshake {
                                serverAddress: serverAddress_2,
                                systemAddresses: systemAddresses_2,
                            }
                        })
                    }
                    "open_connection_reply_1" => {
                        packet__params::open_connection_reply_1({
                            let magic_2 = ();
                            let serverID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            let serverSecurity_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ;
                            let mtuSize_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 2usize ) ? as i16 ;
                            packet_open_connection_reply_1 {
                                magic: magic_2,
                                serverID: serverID_2,
                                serverSecurity: serverSecurity_2,
                                mtuSize: mtuSize_2,
                            }
                        })
                    }
                    "open_connection_reply_2" => {
                        packet__params::open_connection_reply_2({
                            let magic_2 = ();
                            let serverID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            let clientAddress_2 = {
                                let version_3 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ;
                                let address_3 = ();
                                let port_3 = byteorder :: ReadBytesExt :: read_uint :: < byteorder :: BigEndian > ( read , 2usize ) ? as u16 ;
                                address_port {
                                    version: version_3,
                                    address: address_3,
                                    port: port_3,
                                }
                            };
                            let mtuSize_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 2usize ) ? as i16 ;
                            let serverSecurity_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ;
                            packet_open_connection_reply_2 {
                                magic: magic_2,
                                serverID: serverID_2,
                                clientAddress: clientAddress_2,
                                mtuSize: mtuSize_2,
                                serverSecurity: serverSecurity_2,
                            }
                        })
                    }
                    "open_connection_request_1" => {
                        packet__params::open_connection_request_1({
                            let magic_2 = ();
                            let protocol_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ;
                            let mtuSize_2 = ();
                            packet_open_connection_request_1 {
                                magic: magic_2,
                                protocol: protocol_2,
                                mtuSize: mtuSize_2,
                            }
                        })
                    }
                    "open_connection_request_2" => {
                        packet__params::open_connection_request_2({
                            let magic_2 = ();
                            let serverAddress_2 = {
                                let version_3 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ;
                                let address_3 = ();
                                let port_3 = byteorder :: ReadBytesExt :: read_uint :: < byteorder :: BigEndian > ( read , 2usize ) ? as u16 ;
                                address_port {
                                    version: version_3,
                                    address: address_3,
                                    port: port_3,
                                }
                            };
                            let mtuSize_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 2usize ) ? as i16 ;
                            let clientID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            packet_open_connection_request_2 {
                                magic: magic_2,
                                serverAddress: serverAddress_2,
                                mtuSize: mtuSize_2,
                                clientID: clientID_2,
                            }
                        })
                    }
                    "ping" => {
                        packet__params::ping({
                            let pingID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            packet_ping { pingID: pingID_2 }
                        })
                    }
                    "pong" => {
                        packet__params::pong({
                            let pingID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            packet_pong { pingID: pingID_2 }
                        })
                    }
                    "server_handshake" => {
                        packet__params::server_handshake({
                            let clientAddress_2 = {
                                let version_3 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ;
                                let address_3 = ();
                                let port_3 = byteorder :: ReadBytesExt :: read_uint :: < byteorder :: BigEndian > ( read , 2usize ) ? as u16 ;
                                address_port {
                                    version: version_3,
                                    address: address_3,
                                    port: port_3,
                                }
                            };
                            let serverSecurity_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ;
                            let systemAddresses_2 = {
                                let count = 10u64 as usize;
                                let mut v = Vec::with_capacity(count);
                                for i in 0..count {
                                    v . insert ( i , { let version_3 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 1usize ) ? as i8 ; let address_3 = ( ) ; let port_3 = byteorder :: ReadBytesExt :: read_uint :: < byteorder :: BigEndian > ( read , 2usize ) ? as u16 ; address_port { version : version_3 , address : address_3 , port : port_3 } } ) ;
                                }
                                v
                            };
                            let sendPing_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            let sendPong_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            packet_server_handshake {
                                clientAddress: clientAddress_2,
                                serverSecurity: serverSecurity_2,
                                systemAddresses: systemAddresses_2,
                                sendPing: sendPing_2,
                                sendPong: sendPong_2,
                            }
                        })
                    }
                    "unconnected_ping" => {
                        packet__params::unconnected_ping({
                            let pingID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            let magic_2 = ();
                            let unknown_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            packet_unconnected_ping {
                                pingID: pingID_2,
                                magic: magic_2,
                                unknown: unknown_2,
                            }
                        })
                    }
                    "unconnected_ping_open_connections" => {
                        packet__params::unconnected_ping_open_connections({
                            let pingID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            let magic_2 = ();
                            packet_unconnected_ping_open_connections {
                                pingID: pingID_2,
                                magic: magic_2,
                            }
                        })
                    }
                    "unconnected_pong" => {
                        packet__params::unconnected_pong({
                            let pingID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            let serverID_2 = byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 8usize ) ? as i64 ;
                            let magic_2 = ();
                            let serverName_2 = {
                                let mut v = String :: with_capacity ( byteorder :: ReadBytesExt :: read_int :: < byteorder :: BigEndian > ( read , 2usize ) ? as i16 as usize ) ;
                                read.read_to_string(&mut v);
                                v
                            };
                            packet_unconnected_pong {
                                pingID: pingID_2,
                                serverID: serverID_2,
                                magic: magic_2,
                                serverName: serverName_2,
                            }
                        })
                    }
                    "data_packet_0" => {
                        packet__params::data_packet_0({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_1" => {
                        packet__params::data_packet_1({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_2" => {
                        packet__params::data_packet_2({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_3" => {
                        packet__params::data_packet_3({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_4" => {
                        packet__params::data_packet_4({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_5" => {
                        packet__params::data_packet_5({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_6" => {
                        packet__params::data_packet_6({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_7" => {
                        packet__params::data_packet_7({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_8" => {
                        packet__params::data_packet_8({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_9" => {
                        packet__params::data_packet_9({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_A" => {
                        packet__params::data_packet_A({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_B" => {
                        packet__params::data_packet_B({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_C" => {
                        packet__params::data_packet_C({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_D" => {
                        packet__params::data_packet_D({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_E" => {
                        packet__params::data_packet_E({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    "data_packet_F" => {
                        packet__params::data_packet_F({
                            let seqNumber_2 = ();
                            let encapsulatedPackets_2 = ();
                            packet_data_packet {
                                seqNumber: seqNumber_2,
                                encapsulatedPackets: encapsulatedPackets_2,
                            }
                        })
                    }
                    _ => panic!("WTF"),
                };
            packet {
                name: name_1,
                params: params_1,
            }
        };
    Ok(res)
}

