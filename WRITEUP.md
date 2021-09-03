## Kicking dhcpcd clients off the network only using ARP replies

70:85:C2:85:0C:07 is the real MAC address of the device we're kicking off.
Everything else is spoofed. The DHCP server resides at 192.168.0.1.

--------------------------------------------------------------------------------

We send out an ARP reply with an IP we know exists.

```
01. ARP -> Reply: 192.168.0.106 is-at 77:94:13:31:72:E1
```

The device asks who is holding its own IP. This should not happen because
REPLIES should be sent out AFTER REQUESTS, not the other way around.

```
02. ARP -> Request: who-has 192.168.0.106 tell 192.168.0.106
```

If we reply to this request

```
03. ARP -> Reply: 192.168.0.106 is-at 39:27:00:72:91:80
```

the replied-to device will let go of its IP and ask for a new one

```
04. IP --> 0.0.0.0 > 255.255.255.255: BOOTP/DHCP, Request from 70:85:C2:85:0C:07
```

The DHCP server, of course, still holds the IP-MAC pair in its cache and assigns
the device its old IP.

```
05. IP --> 192.168.0.1. > 192.168.0.106: BOOTP/DHCP, Reply
```

The device now broadcasts an ARP request to see if there's someone who already
holds this IP.

```
06. ARP -> Request: who-has 192.168.0.106 tell 0.0.0.0
07. ARP -> Request: who-has 192.168.0.106 tell 0.0.0.0
08. ARP -> Request: who-has 192.168.0.106 tell 0.0.0.0
```

If we send out an ARP reply,

```
09. ARP -> Reply: 192.168.0.106 is-at AF:41:C3:43:78:69
```

the device will think that someone already holds this IP and asks the DHCP
server for a different IP.

```
10. IP --> 0.0.0.0 > 255.255.255.255: BOOTP/DHCP, Request from 70:85:C2:85:0C:07
11. IP --> 0.0.0.0 > 255.255.255.255: BOOTP/DHCP, Request from 70:85:C2:85:0C:07
```

At this point, the DHCP server itself asks if someone isn't holding (the new)
192.168.0.107

```
12. ARP -> Request: who-has 192.168.0.107 tell 192.168.0.1
```

Nobody responds, so the server offers the .107 IP

```
13. IP --> 192.168.0.1. > 192.168.0.107: BOOTP/DHCP, Reply
14. IP --> 0.0.0.0 > 255.255.255.255: BOOTP/DHCP, Request from 70:85:C2:85:0C:07
15. IP --> 192.168.0.1. > 192.168.0.107: BOOTP/DHCP, Reply
```

And the device checks if someone already holds this IP

```
16. ARP -> Request: who-has 192.168.0.107 tell 0.0.0.0
17. ARP -> Request: who-has 192.168.0.107 tell 0.0.0.0
18. ARP -> Request: who-has 192.168.0.107 tell 0.0.0.0
19. ARP -> Request: who-has 192.168.0.107 tell 192.168.0.107
```

Here we can send an ARP reply from 192.168.0.107, the device will ask for
192.168.0.108, we can reply from 192.168.0.108 and so on and so forth..
This way the device will never find a free IP address. The good old classic.
This isn't something new (it's _very_ old, in fact).

Also I'm not actually sure if this is caused by dhcpcd or something else in
Linux (I haven't tried anything else out because I don't find it that
interesting, but I have my doubts that it would be somehow caused by a weird
Linux config). So who knows.
