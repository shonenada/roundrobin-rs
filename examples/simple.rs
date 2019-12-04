use roundrobin::wrr::{Server, WeightedRoundRobinBalancer};

fn main() {
    let url01 = "http://localhost:8081".to_string();
    let url02 = "http://localhost:8082".to_string();
    let server01 = Server::new(url01.clone(), 1);
    let mut rr = WeightedRoundRobinBalancer::new();
    rr.insert_server(server01); // default weight 1
    rr.insert_url(url02.clone(), 5);

    for _i in 0..5 {
        println!("{:?}", rr.next().unwrap());
    }
}
