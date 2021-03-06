// This listens for IP packets coming in (matching a pattern) encapsulates the in a GTP tunnel and sends them to a peer

use std::sync::Mutex;
use std::sync::Arc;

use std::net::IpAddr;
use std::net::UdpSocket;

use pnet::datalink::{self, NetworkInterface};

use pnet::datalink::Channel::Ethernet;

use pnet::packet::Packet;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;

use super::listener_statistics::Statistics;

use super::packet::Packet as GtpPacket;
use super::packet::messages::{
    Message,
    g_pdu
};

pub struct IpListener
{
    peer: IpAddr,
    o_teid: u32,
    stats: Arc<Mutex<Statistics>>,
    socket: UdpSocket,
    interface: NetworkInterface,
    pub src: Option<IpAddr>,
    pub dest: Option<IpAddr>
}

impl IpListener {
    pub fn new(
        peer: IpAddr, 
        o_teid: u32,  
        statistics: Arc<Mutex<Statistics>>, 
        interface_name: &str,
        src: Option<IpAddr>,
        dest: Option<IpAddr>
    ) -> Option<Self> {

        let interfaces = datalink::interfaces();

        let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

        let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap_or_else(|| panic!("No such network interface: {}", interface_name));

        Some(IpListener {
            peer,
            // i_teid,
            o_teid,
            stats: statistics,
            socket: UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address"),
            interface,
            src,
            dest
        })
    }

    pub fn listen(&self) {
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
        let gpdu = g_pdu::Message::new(gtp_payload);

        if let Ok(gpdu) = gpdu {
            let mut p = GtpPacket::new(Message::GPDU(gpdu));
            p.header.set_teid(self.o_teid);
            
            p.send_to(&self.socket, (self.peer, 2152)).expect("Couldn't send GTP Packet");
            let mut s = self.stats.lock().unwrap();
            (*s).tx_gtp_add(1);
            drop(s);
        }
    }
}