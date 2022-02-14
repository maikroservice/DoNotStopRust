# This is a DNS Resolver, written in Rust

Inspired by [TheMayor](https://github.com/dievus), I wanted to learn Rust and build my own tooling to do so.

So we start with a DNS Resolver, dive into subdomain enumeration later and subsequently potentially look into other parts of the attack toolkit.

## Introduction

A Domain Name Service (DNS) Resolver is a tool that takes IP addresses and translates these into domain names.
We have different records:

- address (A)
- quad A (AAAA)
- ANAME
- canonical name (CNAME)
- mail-exchange (MX)
- pointer (PTR)
- state of authority (SOA)
- text (TXT)
- name server (NS)
- sender policy framework (SPF)
- service (SRV)
