// This listens for IP packets coming in (matching a pattern) encapsulates the in a GTP tunnel and sends them to a peer

use std::sync::Mutex;
use std::sync::Arc;

use std::net::IpAddr;
use std::net::UdpSocket;

extern crate pcap;

use pnet::datalink::{self, NetworkInterface};

use pnet::datalink::Channel::Ethernet;

use pnet::packet::Packet;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;

use super::listener_statistics::Statistics;

use super::packet::Packet as GtpPacket;
use super::packet::messages::{
    MessageType,
    Message
};

pub struct IpListener
{
    peer: IpAddr,
    // i_teid: u32,
    o_teid: u32,
    filter: String,
    stats: Arc<Mutex<Statistics>>,
    socket: UdpSocket,
    interface: NetworkInterface,
    pub src: Option<IpAddr>,
    pub dest: Option<IpAddr>
}

impl IpListener {
    pub fn new(peer: IpAddr, /* i_teid: u32,*/ o_teid: u32, filter: String, statistics: Arc<Mutex<Statistics>>, interface: NetworkInterface) -> Option<Self> {
        Some(
            IpListener {
                peer,
                // i_teid,
                o_teid,
                filter,
                stats: statistics,
                socket: UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address"),
                interface,
                src: None,
                dest: None
            }
        )
    }

    pub fn listen(&self) {
        /* using pcap crate */
        /*
        let mut cap = pcap::Capture::from_device("any").unwrap().timeout(1).open().unwrap();

        cap.filter(&self.filter).unwrap();

        while let Ok(packet) = cap.next() {
            let mut s = self.stats.lock().unwrap();
            (*s).rx_ip_add(1);
            drop(s);

            if let Some(e_packet) = EthernetPacket::new(packet.data) {
                let mut p = GtpPacket::new(MessageType::GPDU);
                p.header.set_teid(self.o_teid);

                match p.message {
                    Message::GPDU(ref mut m) => {
                        // We skip the first 2 packets of the payload as they contain the ethertype
                        m.attach_packet(&e_packet.payload()[2..]).unwrap();
                        p.send_to(&self.socket, (self.peer, 2152)).expect("Couldn't send GTP Packet");
                        let mut s = self.stats.lock().unwrap();
                        (*s).tx_gtp_add(1);
                        drop(s);
                    }
                    _ => {} // Do nothing
                }
            }
        }
        */

        /* using datalink listener */

        // Create a channel to receive on
        let (_, mut rx) = match datalink::channel(&self.interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("IpListener: unhandled channel type: {}"),
            Err(e) => panic!("IpListener: unable to create channel: {}", e),
        };

        while let Ok(packet) = rx.next() {
            if let Some(e_packet) = EthernetPacket::new(packet) {
                match e_packet.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        if let Some(ip_packet) = Ipv4Packet::new(e_packet.payload()) {
                            if let Some(addr) = self.src {
                                // Is src address filter specified?
                                if addr.is_ipv4() && addr == ip_packet.get_source() {
                                    // Does it match our filter?
                                    self.send_gtp_packet(e_packet.payload());
                                }
                                else { /* Doens't match source filter. Ignore */ }
                            }
                            else if let Some(addr) = self.dest {
                                // Is src address filter specified?
                                if addr.is_ipv4() && addr == ip_packet.get_destination() {
                                    // Does it match our filter?
                                    self.send_gtp_packet(e_packet.payload());
                                }
                                else { /* Doens't match destination filter. Ignore */ }
                            }
                            else { }
                        }
                        else { /* Couldn't parse IPv4 Packet. Ignore */ }
                    },
                    EtherTypes::Ipv6 => {
                        if let Some(ip_packet) = Ipv6Packet::new(e_packet.payload()) {
                            if let Some(addr) = self.src {
                                // Is src address filter specified?
                                if addr.is_ipv6() && addr == ip_packet.get_source() {
                                    // Does it match our filter?
                                    self.send_gtp_packet(e_packet.payload());
                                }
                                else { /* Doens't match source filter. Ignore */ }
                            }
                            else if let Some(addr) = self.dest {
                                // Is src address filter specified?
                                if addr.is_ipv6() && addr == ip_packet.get_destination() {
                                    // Does it match our filter?
                                    self.send_gtp_packet(e_packet.payload());
                                }
                                else { /* Doens't match destination filter. Ignore */ }
                            }
                            else { }
                        }
                        else { /* Couldn't parse IPv4 Packet. Ignore */ }
                    },
                    _ => { /* Only look at IP packets. Ignore */ },
                }
            }
        }
    }

fn send_gtp_packet(&self, gtp_payload: &[u8]) {
        let mut p = GtpPacket::new(MessageType::GPDU);
        p.header.set_teid(self.o_teid);

        match p.message {
            Message::GPDU(ref mut m) => {
                // We skip the first 2 packets of the payload as they contain the ethertype
                m.attach_packet(gtp_payload).unwrap();
                p.send_to(&self.socket, (self.peer, 2152)).expect("Couldn't send GTP Packet");
                let mut s = self.stats.lock().unwrap();
                (*s).tx_gtp_add(1);
                drop(s);
            }
            _ => {} // Do nothing
        }
    }
}