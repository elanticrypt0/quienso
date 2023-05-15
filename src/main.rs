use std::{process::exit, net::{IpAddr, SocketAddr}};
use whois_rust::{WhoIs, WhoIsLookupOptions};
use colored::*;
use dns_lookup::{lookup_host, lookup_addr, getnameinfo, AddrInfoHints, SockType, getaddrinfo};

#[tokio::main]
async fn main() {
    print_app_header();
    // read command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1{
        let domain= &args[1];
    
        check_domain(&domain);
    }else{
        println!("Ingresa un dominio para investigar. {}","Ej: google.com.ar".green());
    }

    // ingresa en un loop y no sale hasta que se escriba exit | salir
    loop{
        // prompt para ingresar un dominio
        println!("Ingresa un comando. {} para ayuda. {} o {} para terminar el programa","--help".green(),"exit".red(),"salir".red());
        let mut domain=String::new();
        std::io::stdin().read_line(&mut domain).expect("Error al leer el dominio");
        domain=domain.trim().to_string();
        if domain=="exit" || domain=="salir"{
            break;
        }else if domain=="--help" {
            show_help();
        }
        else{
            check_domain(&domain);
            whois_request(&domain).await;
            print_separator();
            dns_lookup(&domain).await;
            // PRINT IP services
            // print_separator();
            // let ips=get_domain_ip_addrs(&domain).await;
            // dbg!(&ips);
            // for ip in ips {
            //     services_lookup(ip).await;
            // }
            // services_lookup(domain).await;
            print_separator();

        }
    }
    
}

fn print_app_header(){
    let title="
    ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
    ████░▄▄░█░██░██▄██░▄▄█░▄▄▀███░▄▄█▀▄▄▀█▄░████
    ████░██░█░██░██░▄█░▄▄█░██░███▄▄▀█░██░█░▄████
    ████▄▄░▀██▄▄▄█▄▄▄█▄▄▄█▄██▄███▄▄▄██▄▄██▀█████
    ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
    ";
    println!("{}",title.green());
    // print programa version & author from Cargo.toml
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown version");
    let author = option_env!("CARGO_PKG_AUTHORS").unwrap_or("unknown author");
    let repo_url= option_env!("CARGO_PKG_REPOSITORY").unwrap_or("unknown url");

    println!("Version: {} by {}", version.green(), author.magenta());
    println!("Repo: {}", repo_url.blue());
    println!("");
    println!("#################################################");
    println!("");
}

fn show_help(){
    println!("Comandos disponibles: {}","Listado".green());
}

fn check_domain(domain: &String){
    if domain.is_empty(){
        println!("{}","El domino es obligatorio".red());
        exit(01);
    }
}

fn print_separator(){
    println!("");
    println!("---------------------------------------------");
    println!("");
}

// Realiza un whois dependiendo del dominio.

async fn whois_request(domain: &String) {
    let whois=WhoIs::from_path("whois-servers.json").unwrap();
    let domain_info=whois.lookup(WhoIsLookupOptions::from_string(domain).unwrap()).unwrap();

    println!(":::: {} ::::","DOMAIN INFO".green());
    println!("{}", domain_info);
}

// obtiene el IP V4 y V6 de los dns de un dominio.

async fn dns_lookup(domain: &String){
    let ips = lookup_host(&domain).unwrap();
    println!(":::: {} ::::","DNS LOOKUP".green());
    for ip in ips {
        println!("DNS name:{} | DNS IP:{}", lookup_addr(&ip).unwrap().green(), &ip.to_string().yellow());
    }

}

async fn get_domain_ip_addrs(domain: &String) -> Vec<std::net::IpAddr>{
    let ips: Vec<std::net::IpAddr> = lookup_host(domain).unwrap();
    println!("IP addresses for {}:", domain);
    return ips;
}

async fn services_name_by_port(ip: IpAddr,port: u16){
    // let ip: IpAddr = "127.0.0.1".parse().unwrap();
    // let ip: IpAddr = ip;
    // let port = 8081;
    let socket: SocketAddr = (ip, port).into();

    let (name, service) = match getnameinfo(&socket, 0) {
        Ok((n, s)) => (n, s),
        Err(e) => panic!("Failed to lookup socket {:?}", e),
    };

    println!("{:?} {:?}", name, service);
    let _ = (name, service);
}

async fn service_port_by_name(domain :&String, service: &str){
    let hints = AddrInfoHints {
      socktype: SockType::Stream.into(),
      .. AddrInfoHints::default()
    };
    let sockets =
      getaddrinfo(Some(domain), Some(service), Some(hints))
        .unwrap().collect::<std::io::Result<Vec<_>>>().unwrap();

    for socket in sockets {
      // Try connecting to socket
      dbg!(&socket);
      let _ = socket;
    }
}