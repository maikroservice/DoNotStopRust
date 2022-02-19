# This is a DNS Resolver, written in Rust

Inspired by [TheMayor's](https://github.com/dievus) built your own tooling series, I wanted to learn Rust and build my own tooling to do so.

So we start with a DNS Resolver and subsequently look into other parts of the attack toolkit.

## Introduction

A Domain Name Service (DNS) Resolver is a tool that takes IP addresses and translates these into domain names or the other way around - it takes domain names and translates those to IP addresses.

When we take a url, it has different parts - scheme://subdomain.2nd-level-domain.top-level-domain so for example https://www.maikroservice.com is in the .com toplevel-domain.

Imagine the the top-level domains as individual racks in a library. They have all the information for the e.g. `.com` domains. Within the `.com` rack we can find the `2nd-level-domain` e.g. `maikroservice` in this case. They cover a specific area of the rack, and within that area you can find all the subdomains for `maikroservice.com`. In order to know where to look, there is a description at the front of the rack, sometimes also known as the `authoritive nameserver`, it is the final boss for getting our IP for the URL we provided (for example in our browser).

Moving backwards, we now know about the authoritive nameserver that points us to the IP within the correct top/2nd level domains. We also know about top-level domain servers e.g. `.com`, `.se`, `.in`, but how do we know which top level domain server to ask? We need to talk to the "bouncer" of the library, aka `DNS recursor`. The `DNS recursor` contacts the root name server which is an index for the "internet", it knows who the individual top level domain name servers are.

## DNS Queries

We query for `https://www.maikroservice.com`

1. we start with our local machine - here we might have a cache of locally stored domain/IP lists that get queried first
2. If we don't find the results locally, we reach out to our Internet Service Provider (ISP) and check if they have the corresponding record
3. our ISP now has three different options:
   1. the address record is here and gets send as a response
   2. the address record is not here but we have a NameServer (NS) record stored for the toplevel domain (`.com`) -> this means our ISP directly queries the toplevel domain NameServer skipping the root NameServer
   3. the NameServer record and the address record can not be found and our ISP queries the root NameServer (unlikely, but possible after cache purge)

We have two types of DNS Queries:

`Non-recursive DNS Queries` - these are typical for when the client already knows the authoritive server (the one that is aware of things inside the toplevel domain server - `maikroservice` inside the `.com`) - they would not be recursive because the answer either is there or not available.

`DNS query (with a recursive flag set)` - in case the e.g. 2nd-level domain/toplevel domain is unknown we need to forward our request and thus need to recurse into the depths of DNS servers.

Inverse Query - retrieve the domain name for an IP address

## DNS Request:

Usually done with `UDP` via port 53, and only uses `TCP` for zone transfer requests - [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035).

### UDP:53

What does a request look like? Where do we sent it to? What do we get as a response?

### TCP:53

Is the request the same as for the UDP counter-part?

### HTTPS

What in god's name is this? Why is there another port and when do we use this?
OHHHH - this is for encrypting the traffic, because otherwise it would be clear text :O

```text
Inverse Query:

                         +-----------------------------------------+
           Header        |          OPCODE=IQUERY, ID=997          |
                         +-----------------------------------------+
          Question       |                 <empty>                 |
                         +-----------------------------------------+
           Answer        |        <anyname> A IN <IP-address>      |
                         +-----------------------------------------+
          Authority      |                 <empty>                 |
                         +-----------------------------------------+
         Additional      |                 <empty>                 |
                         +-----------------------------------------+
```

```text
Response:
                         +-----------------------------------------+
           Header        |         OPCODE=RESPONSE, ID=997         |
                         +-----------------------------------------+
          Question       |QTYPE=A, QCLASS=IN, QNAME=VENERA.ISI.EDU |
                         +-----------------------------------------+
           Answer        |  VENERA.ISI.EDU  A IN 10.1.0.52         |
                         +-----------------------------------------+
          Authority      |                 <empty>                 |
                         +-----------------------------------------+
         Additional      |                 <empty>                 |
                         +-----------------------------------------+
```

## Records

We have different records:

- address (A) - basically an IPv4 address
- quad A (AAAA) - IPv6 address?
- ANAME - ?
- canonical name (CNAME) - ?
- mail-exchange (MX) - mail servers
- pointer (PTR) - ?
- state of authority (SOA) - ?
- text (TXT) - ?
- name server (NS) - ?
- sender policy framework (SPF) - ?
- service (SRV) - ?

### Todos:

What are Name Servers?
How do we identify Name Servers?
Is there a list of Name Servers that we can use? https://public-dns.info/nameserver/nameservers.json
DNS works mostly via UDP
