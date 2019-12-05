#[macro_use]
extern crate log;
use rand::Rng;
use roundrobin::wrr::{Server, WeightedRoundRobinBalancer, Balancer};

fn chaos_server(rr: &mut WeightedRoundRobinBalancer, url: &String) {
    if rand::random() {
        return;
    }
    if let Some(_s) = rr.search_server_by_url(url) {
        let mut rng = rand::thread_rng();
        let times = rng.gen_range(0, 4);
        println!("Chaos: simulating request failed {}, {} times", url, times);
        for _ in 0..times {
            rr.fail(url);
        }
    }
}

fn main() {
    pretty_env_logger::init();
    let url01 = "http://localhost:8081".to_string();
    let url02 = "http://localhost:8082".to_string();
    let url03 = "http://localhost:8083".to_string();
    let url04 = "http://localhost:8084".to_string();
    let url05 = "http://localhost:8085".to_string();

    let server01 = Server::new(url01.clone(), 1);

    let mut rr = WeightedRoundRobinBalancer::new();
    rr.insert_server(server01);
    rr.insert_url(url02.clone(), 2);
    rr.insert_url(url03.clone(), 3);
    rr.insert_url(url04.clone(), 4);
    rr.insert_url(url05.clone(), 5);

    for i in 0..50 {
        let url: String;
        {
            let rv = rr.next().unwrap();
            url = rv.get_url().clone();
            println!(
                "Test {:02}: {}, weight: {}, effect_weight: {}, current_weight: {}",
                i,
                url,
                rv.get_weight(),
                rv.get_effect_weight(),
                rv.get_cur_weight()
            );
        }
        chaos_server(&mut rr, &url);
    }
}
