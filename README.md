## arp\_spoof

A program I wrote when I was bored and wanted to learn some lower-level
networking in Rust.

It sends a packet with a randomized MAC address to some destination.
This doesn't do anything useful whatsoever, however I did find a persistent way
to kick clients using [dhcpcd](https://roy.marples.name/projects/dhcpcd/) off
the network with it ;).
