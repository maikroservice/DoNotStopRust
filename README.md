# This is a DNS Resolver, written in Rust

Inspired by [TheMayor](https://github.com/dievus), I wanted to learn Rust and build my own tooling to do so.

So we start with a DNS Resolver, dive into subdomain enumeration later and subsequently potentially look into other parts of the attack toolkit.

## Introduction

A Domain Name Service (DNS) Resolver is a tool that takes IP addresses and translates these into domain names.
We have different records:

- address (A) - basically an IPv4 address
- quad A (AAAA) - IPv6 address
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
