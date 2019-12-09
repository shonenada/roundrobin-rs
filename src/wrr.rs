use std::cmp;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Server {
    weight: i32,
    effect_weight: Arc<RwLock<i32>>,
    cur_weight: Arc<RwLock<i32>>,
    url: String,
}

pub trait Balancer {
    fn next(&self) -> Option<&Server>;
    fn fail(&self, url: &String);
}

impl Server {
    pub fn new(url: String, weight: i32) -> Server {
        return Server {
            effect_weight: Arc::new(RwLock::new(1)),
            cur_weight: Arc::new(RwLock::new(0)),
            weight,
            url,
        };
    }

    pub fn get_weight(&self) -> i32 {
        self.weight
    }

    pub fn get_effect_weight(&self) -> i32 {
        self.effect_weight.read().unwrap().to_owned()
    }

    pub fn get_cur_weight(&self) -> i32 {
        self.cur_weight.read().unwrap().to_owned()
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }
}

#[derive(Debug)]
pub struct WeightedRoundRobinBalancer {
    servers: Vec<Server>,
}

impl WeightedRoundRobinBalancer {
    pub fn new() -> WeightedRoundRobinBalancer {
        return WeightedRoundRobinBalancer { servers: vec![] };
    }

    pub fn get_servers(&self) -> &Vec<Server> {
        &self.servers
    }

    pub fn get_servers_mut(&mut self) -> &mut Vec<Server> {
        &mut self.servers
    }

    pub fn search_server_by_url(&self, url: &String) -> Option<&Server> {
        for each in self.get_servers() {
            if each.get_url() == url {
                return Some(each);
            }
        }
        None
    }

    pub fn insert_server(&mut self, server: Server) {
        if let None = self.search_server_by_url(server.get_url()) {
            self.servers.push(server);
        }
    }

    pub fn insert_url(&mut self, url: String, weight: i32) {
        let server = Server::new(url, weight);
        self.insert_server(server);
    }
}

impl Balancer for WeightedRoundRobinBalancer {
    fn next(&self) -> Option<&Server> {
        let mut best_idx = 0;
        let mut best_weight = 0;
        let mut total = 0;

        for (idx, each) in self.servers.iter().enumerate() {
            let mut cw = each.cur_weight.write().unwrap();
            let mut ew = each.effect_weight.write().unwrap();
            *cw += *ew;
            total += *ew;
            if *ew < each.weight {
                *ew += 1;
            }

            if *ew > best_weight {
                best_idx = idx;
                best_weight = *cw;
            }
        }
        if let Some(best) = self.servers.get(best_idx) {
            let mut bcw = best.cur_weight.write().unwrap();
            *bcw -= total;
            Some(best)
        } else {
            None
        }
    }

    fn fail(&self, url: &String) {
        for each in self.get_servers() {
            if each.get_url() == url {
                let mut ew = each.effect_weight.write().unwrap();
                let diff = cmp::max(each.weight / 3, 1);
                *ew -= diff;
                if *ew < 0 {
                    *ew = 0
                }

                debug!(
                    "Failed server: {}, effect_weight: {}, diff: {}",
                    url, ew, diff
                );
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let url = "http://localhost:8081".to_string();

        let mut wrr = WeightedRoundRobinBalancer::new();
        wrr.insert_url(url.clone(), 5);

        let r1 = wrr.next().unwrap();
        assert_eq!(r1.url, url.clone());
        assert_eq!(*(r1.effect_weight.read().unwrap()), 2);

        let r2 = wrr.next().unwrap();
        assert_eq!(*(r2.effect_weight.read().unwrap()), 3);

        let _r3 = wrr.next().unwrap(); // effect_weight = 4
        let _r4 = wrr.next().unwrap(); // effect_weight = 5

        let r5 = wrr.next().unwrap(); // effect_weight = max_weight
        assert_eq!(*(r5.effect_weight.read().unwrap()), 5);
    }

    #[test]
    fn test_fail() {
        let url = "http://localhost:8081".to_string();

        let mut wrr = WeightedRoundRobinBalancer::new();
        wrr.insert_url(url.clone(), 5);

        let r1 = wrr.next().unwrap();
        assert_eq!(r1.url, url.clone());
        assert_eq!(*(r1.effect_weight.read().unwrap()), 2);

        wrr.fail(&url);
        let s1 = wrr.search_server_by_url(&url).unwrap(); // effect_weight = 1
        assert_eq!(*(s1.effect_weight.read().unwrap()),  1);

        wrr.fail(&url);
        let _s2 = wrr.search_server_by_url(&url).unwrap(); // effect_weight = 0
        let s3 = wrr.search_server_by_url(&url).unwrap(); // still effect_weight = 0
        assert_eq!(*(s3.effect_weight.read().unwrap()), 0);
    }

    #[test]
    fn test_fail_then_next() {
        let url = "http://localhost:8086".to_string();

        let mut wrr = WeightedRoundRobinBalancer::new();
        wrr.insert_url(url.clone(), 3);

        let r1 = wrr.next().unwrap();
        assert_eq!(r1.url, url.clone());
        assert_eq!(*(r1.effect_weight.read().unwrap()), 2);

        wrr.fail(&url);
        let s1 = wrr.search_server_by_url(&url).unwrap(); // effect_weight = 1
        assert_eq!(*(s1.effect_weight.read().unwrap()), 1);

        let r2 = wrr.next().unwrap();
        assert_eq!(*(r2.effect_weight.read().unwrap()), 2);
    }
}
