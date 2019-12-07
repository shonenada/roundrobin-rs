use std::cmp;

#[derive(Debug)]
pub struct Server {
    weight: i32,
    effect_weight: i32,
    cur_weight: i32,
    url: String,
}

pub trait Balancer {
    fn next(&mut self) -> Option<&Server>;
    fn fail(&mut self, url: &String);
}

impl Server {
    pub fn new(url: String, weight: i32) -> Server {
        return Server {
            effect_weight: 1,
            cur_weight: 0,
            weight,
            url,
        };
    }

    pub fn get_weight(&self) -> i32 {
        self.weight
    }

    pub fn get_effect_weight(&self) -> i32 {
        self.effect_weight
    }

    pub fn get_cur_weight(&self) -> i32 {
        self.cur_weight
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
    fn next(&mut self) -> Option<&Server> {
        let mut best_idx = 0;
        let mut best_weight = 0;
        let mut total = 0;

        for (idx, each) in self.servers.iter_mut().enumerate() {
            each.cur_weight = each.cur_weight + each.effect_weight;
            total += each.effect_weight;
            if each.effect_weight < each.weight {
                each.effect_weight += 1
            }

            if each.cur_weight > best_weight {
                best_idx = idx;
                best_weight = each.cur_weight;
            }
        }
        if let Some(mut best) = self.servers.get_mut(best_idx) {
            best.cur_weight = best.cur_weight - total;
            Some(best)
        } else {
            None
        }
    }

    fn fail(&mut self, url: &String) {
        for mut each in self.get_servers_mut() {
            if each.get_url() == url {
                let diff = cmp::max(each.weight / 3, 1);
                each.effect_weight -= diff;
                if each.effect_weight < 0 {
                    each.effect_weight = 0
                }

                debug!(
                    "Failed server: {}, effect_weight: {}, diff: {}",
                    url, each.effect_weight, diff
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
        assert!(r1.url == url.clone());
        assert!(r1.effect_weight == 2);

        let r2 = wrr.next().unwrap();
        assert!(r2.effect_weight == 3);

        let _r3 = wrr.next().unwrap(); // effect_weight = 4
        let _r4 = wrr.next().unwrap(); // effect_weight = 5

        let r5 = wrr.next().unwrap(); // effect_weight = max_weight
        assert!(r5.effect_weight == 5);
    }

    #[test]
    fn test_fail() {
        let url = "http://localhost:8081".to_string();

        let mut wrr = WeightedRoundRobinBalancer::new();
        wrr.insert_url(url.clone(), 5);

        let r1 = wrr.next().unwrap();
        assert!(r1.url == url.clone());
        assert!(r1.effect_weight == 2);

        wrr.fail(&url);
        let s1 = wrr.search_server_by_url(&url).unwrap(); // effect_weight = 1
        assert!(s1.effect_weight == 1);

        wrr.fail(&url);
        let _s2 = wrr.search_server_by_url(&url).unwrap(); // effect_weight = 0
        let s3 = wrr.search_server_by_url(&url).unwrap(); // still effect_weight = 0
        assert!(s3.effect_weight == 0);
    }

    #[test]
    fn test_fail_then_next() {
        let url = "http://localhost:8086".to_string();

        let mut wrr = WeightedRoundRobinBalancer::new();
        wrr.insert_url(url.clone(), 3);

        let r1 = wrr.next().unwrap();
        assert!(r1.url == url.clone());
        assert!(r1.effect_weight == 2);

        wrr.fail(&url);
        let s1 = wrr.search_server_by_url(&url).unwrap(); // effect_weight = 1
        assert!(s1.effect_weight == 1);

        let r2 = wrr.next().unwrap();
        assert!(r2.effect_weight == 2);
    }
}
