//#![deny(warnings)]

// mod args;

// use crate::args::Args;
// use clap::Parser;
//use std::io::{stdout, BufWriter, Write};
use std::net;
use std::str;

// shameless plug: https://gist.github.com/lanedraex/bc01eb399614359470cfacc9d95993fb
// we connect to the google nameserver via UDP on port 53 and lets see what happens

fn main() {
    //println!("1");
    // https://jan.newmarch.name/NetworkProgramming/UDP/wrapper.html?rust
    let _nameserver = "8.8.8.8:53";
    let test = "we love hacking";
    let home = "127.0.0.1:34254";
    let socket = net::UdpSocket::bind("0.0.0.0:34254").expect("failed to establish connection");
    //println!("2");
    let mut buf = [0; 512];
    //socket.send_to(&buf, &home).expect("couldn't send data");
    //socket.connect(home).expect("could not connect");
    //socket.send(&[0, 1, 2]).expect("couldn't send message");
    socket
        .send_to(test.as_bytes(), home)
        .expect("Error on send");

    //println!("3");
    loop {
        let (amt, _src) = socket.recv_from(&mut buf).expect("could not receive");
        //println!("4");
        let echo = str::from_utf8(&buf[..amt]).unwrap();
        println!("received: {}", echo);
    }
}
