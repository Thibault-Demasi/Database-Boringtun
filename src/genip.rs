use rand::Rng;

fn random_number() -> String {
    let premiere_ip = rand::thread_rng().gen_range(1..254);
    //println!("entier : {}", premiere_ip);
    return premiere_ip.to_string();
}

pub fn ip_generator() -> String {
    let ip = "10.10.".to_owned();
    let first = random_number();
    let second = random_number();
    let ip = ip + &first + "." + &second;
    //println!("Adresse IP Finale = {:?}", ip);
    return ip;
}


