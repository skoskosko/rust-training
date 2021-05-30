extern crate yaml_rust;
extern crate ctrlc;
extern crate ureq;

use dns_lookup::lookup_host;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use std::{thread, time};
use std::process;

#[tokio::main]
async fn main() {

    // Cntrl c listener
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        process::exit(0x0100);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        
        // Get my public ip address
        if let Some(ip) = public_ip::addr().await {
            println!("public ip address: {:?}", ip);
            
            // Read configuration file
            let file = "./credentials.yaml";
            let mut file = File::open(file).expect("Unable to open file");
            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("Unable to read file");
        
            let docs = YamlLoader::load_from_str(&contents).unwrap();
        
            let doc = &docs[0];
            
            println!("{:?}", doc["username"].as_str().unwrap());
            println!("{:?}", doc["password"].as_str().unwrap());

            let hostname = doc["domain"].as_str().unwrap();
            let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();

            println!("{:?}", ips[0]);

            // Check if ip is correct
            if ips[0] == ip {
                println!("No action needed");

            } else {
                println!("Action needed");
                
                let username: &str  =  doc["username"].as_str().unwrap();
                let password: &str  = doc["password"].as_str().unwrap();
                let domain: &str  = doc["domain"].as_str().unwrap();

                let url : String =  format!( "https://svc.joker.com/nic/update?username={user}&password={pass}&hostname={host}", user = username ,pass = password , host = domain ).to_owned();
                let cleanurl : &str =  &url[..]; 
                let body: String = ureq::get( cleanurl )
                    .call().unwrap().into_string().unwrap();

                println!("{:?}", body);

                thread::sleep(time::Duration::from_secs(22000));

            }

        } else {
            println!("couldn't get an IP address");
        }

        thread::sleep(time::Duration::from_secs(3600));
    }

}