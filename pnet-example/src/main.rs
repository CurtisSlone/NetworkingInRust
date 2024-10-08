extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{Ethertypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ip::IpnextHeaderProtocols;
use pnet::packet::Packet;

use std::env;

fn handle_packet(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let header = Ipv4Packet::new(ethernet.payload());
            if let Some(header) = header {
                match header.get_next_level_protocol(){
                    IpnextHeaderProtocols::Tcp => {
                        let tcp = TcpPacket::new(header.payload());
                        if let Some(tcp) tcp {
                            println!(
                                "Got a TCP Packet {}:{} to {}:{}",
                                header.get_source(),
                                tcp.get_source(),
                                header.get_destination(),
                                tcp.get_destination()
                            );
                        }
                    }

                    _ => println!("Ignoring non TCP pasket"),
                }
            }
        }
        _ => println!("Ignoring non IPv4 packet"),
    }
}


fn main() {

    let interface_name = env::args().nth(1).unwrap();

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .expect("Error getting interface");

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()){
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => {
            panic!(
                "An error occured when creting the dtalink channel: {}", e
            )
        }
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                handle_packet(&packet);
            }

            Err(e) => {
                eprintln!("An error occurred while reading from the network: {}", e);
            }
        }
    }
}
