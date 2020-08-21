// This listens for GTP packets coming in and de-encapsulates them

// extern crate pcap;
extern crate pnet;
// extern crate arp_cache;

use std::sync::Mutex;
use std::sync::Arc;

use std::net::{IpAddr, UdpSocket};

use pnet::packet::ipv4::Ipv4Packet;

use pnet::transport;
// use arp_cache::ArpCache;

use super::listener_statistics::Statistics;

use super::packet::Packet as GtpPacket;
use super::packet::messages::{Message, echo_response};

use crate::MTU;

pub struct GtpListener
{
    i_teid: u32,
    o_teid: u32,
    stats: Arc<Mutex<Statistics>>,
    socket: UdpSocket,
    // o_interface: datalink::NetworkInterface,
    // arp_cache: ArpCache,
}

impl GtpListener {
    pub fn new(i_teid: u32, o_teid: u32, statistics: Arc<Mutex<Statistics>>, /*o_interface: datalink::NetworkInterface, gateway: IpAddr*/) -> Option<Self> {
        
        // let arp_cache: Option<ArpCache> = match gateway {
        //     IpAddr::V4(gateway) => ArpCache::new(&o_interface.name, gateway),
        //     IpAddr::V6(_) => None // We need to support NDP for IPv6 to work
        // }

        // if let Some(arp_cache) = arp_cache {
            Some(
                GtpListener {
                    i_teid,
                    o_teid,
                    stats: statistics,
                    socket: UdpSocket::bind("0.0.0.0:2152").expect("couldn't bind to address"),
                    // o_interface,
                    // arp_cache
                }
            )
        // }
        // else {
        //     None
        // }
       
    }

    pub fn listen(&self) {
        let mut buffer = [0; MTU];

        while let Ok((number_of_bytes, src_addr)) = self.socket.recv_from(&mut buffer) {
            // Check if the incoming packet matched the TEID we are looking for
            let p = GtpPacket::parse(&buffer[..number_of_bytes]);

            if let Some((p, _pos)) = p {
                // Successfully parsed the packet
                if p.header.teid() == self.i_teid {
                    let mut s = self.stats.lock().unwrap();
                    (*s).rx_gtp_add(1);
                    drop(s);
                    // It is for us
                    match p.message {
                        Message::EchoRequest(_m) => {
                            // Send an EchoResponse
                            let mut s = self.stats.lock().unwrap();
                            (*s).rx_gtp_echo_request_add(1);
                            drop(s);
                            
                            let mut echo_response = GtpPacket::new(Message::EchoResponse(echo_response::Message::new()));
                            echo_response.header.set_teid(self.o_teid);
                            if let Ok(_n) = echo_response.send_to(&self.socket, src_addr){
                                let mut s = self.stats.lock().unwrap();
                                (*s).tx_gtp_echo_response_add(1);
                                drop(s);
                            }
                            else {}
                        },
                        Message::GPDU(m) => {
                            // Process the GPDU
                            // Now we need to parse the packet inside the GTP packet
                            
                            // Here we are assuming IPv4 for the inner packet
                            let inner_ip_packet = Ipv4Packet::new(&m.t_pdu);

                            if let Some(inner_ip_packet) = inner_ip_packet {
                                

                                let (mut sender, _) = match transport::transport_channel(
                                    4096, 
                                    transport::TransportChannelType::Layer3(inner_ip_packet.get_next_level_protocol())
                                ) {
                                    Ok((tx, rx)) => (tx, rx),
                                    Err(e) => panic!(
                                        "An error occurred when creating the transport channel: {}",
                                        e
                                    ),
                                };

                                let dest_ip_address = IpAddr::V4(inner_ip_packet.get_destination());
                                
                                if let Ok(_n) = sender.send_to(inner_ip_packet, dest_ip_address) {
                                    let mut s = self.stats.lock().unwrap();
                                    (*s).tx_ip_add(1);
                                    drop(s);
                                }
                                else {}
                            }
                            else {
                                // If the inner packet isn't IPv4 we bail out
                                // TODO: Support IPv6 inner packets
                            }

                            // let e_fields = Ethernet {
                            //     source: self.o_interface.mac.unwrap(),
                            //     destination: self.arp_cache.get(),
                            //     ethertype: EtherTypes::Ipv4,
                            //     payload: m.t_pdu,
                            // };

                            // let mut ethernet_buffer = [0u8; 42];
                            // let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

                            // ethernet_packet.populate(&e_fields);

                            // let (mut sender, _) = match datalink::channel(self.interface, Default::default()) {
                            //     Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
                            //     Ok(_) => panic!("Unknown channel type"),
                            //     Err(e) => panic!("Error happened {}", e),
                            // };
                        },
                        _ => {
                            // Do nothing
                        }
                    }
                }
                else {
                    let mut s = self.stats.lock().unwrap();
                    (*s).rx_ignored_gtp_add(1);
                    drop(s);
                }
            }
            // Clear buffer for next packet
            buffer = [0; MTU];
        }            
    }
}
