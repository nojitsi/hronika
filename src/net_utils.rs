use local_ip_address::local_ip;
use local_ip_address::list_afinet_netifas;

fn show_my_ips() {
    let my_local_ip = local_ip().unwrap();

    println!("This is my local IP address: {:?}", my_local_ip);

    let network_interfaces = list_afinet_netifas().unwrap();

    for (name, ip) in network_interfaces.iter() {
        println!("{}:\t{:?}", name, ip);
    }    
}
