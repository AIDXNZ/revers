#[macro_use]
extern crate r2pipe;
use serde_json;
use r2pipe::R2Pipe;
use std::env;

fn main() {
    let path = "/home/zdroid/Documents/Programming/revers/target/release/revers";
    //let path = Some("/home/zdroid/Documents/Programming/revers/target/debug/revers".to_owned());
    let mut r2p = open_pipe!(Some(path)).unwrap();
    

    let md5_hash = "ph md5";
    println!("[MD5 Hash Checksum]");
    println!("{}", r2p.cmd(md5_hash.clone()).unwrap());

    
    // List all Libraies used
    let list_lib = "rabin2 -l `which r2`";
    println!("{}", r2p.cmd(list_lib.clone()).unwrap());


    if let Ok(json) = r2p.cmdj("ij") {
        //println!("{}", serde_json::to_string_pretty(&json).unwrap());
        println!("ARCH {}", json["bin"]["arch"]);
        println!("Lang {}", json["bin"]["lang"]);
        println!("Is Canary {}", json["bin"]["canary"]);
        println!("Stripped {}", json["bin"]["stripped"]);
        
    }
    r2p.close();
}