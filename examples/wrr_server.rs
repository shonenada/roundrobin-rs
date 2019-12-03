use roundrobin_rs::wrr::*;

fn main() {
    let url01 = "http://localhost:8081".to_string();
    let url02 = "http://localhost:8082".to_string();
    let url03 = "http://localhost:8083".to_string();
    let url04 = "http://localhost:8084".to_string();
    let url05 = "http://localhost:8085".to_string();

    let server01 = Server::new(url01.clone(), 1);

    let mut rr = WeightedRoundRobinBalancer::new();
    rr.insert_server(server01); // default weight 1
    rr.insert_url(url02.clone(), 2);
    rr.insert_url(url03.clone(), 3);
    rr.insert_url(url04.clone(), 4);
    rr.insert_url(url05.clone(), 5);

    for i in 0..50 {
        let rv = rr.next().unwrap();
        println!("Test {:02}: {}", i, rv.url);
    }
}
