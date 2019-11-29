#[derive(Debug)]
pub struct Server {
    weight: i32,
    cur_weight: i32,
    url: String,
}

impl Server {
    pub fn new(url: String) -> Server {
        return Server{
            weight: 0,
            cur_weight: -1,
            url: url,
        }
    }
}

#[derive(Debug)]
pub struct RoundRobinBalancer {
    pub servers: Vec<Server>,
    cur_idx: usize,
}

impl RoundRobinBalancer {

    pub fn new() -> RoundRobinBalancer {
        return RoundRobinBalancer{
            servers: vec![],
            cur_idx: 0,
        }
    }

    pub fn insert(&mut self, server: Server) {
        self.servers.push(server);
    }

    pub fn next(&mut self) -> Option<&Server> {
        let s = self.servers.get(self.cur_idx);
        self.cur_idx = (self.cur_idx + 1) % self.servers.len();
        s.clone()
    }

}

mod tests {
    use super::{Server, RoundRobinBalancer};
    #[test]
    fn test_next() {
        let url01 = "http://localhost:8081".to_string();
        let url02 = "http://localhost:8082".to_string();
        let server01 = Server::new(url01.clone());
        let server02 = Server::new(url02.clone());

        let mut rr = RoundRobinBalancer::new();
        rr.insert(server01);
        rr.insert(server02);

        let r1 = rr.next().unwrap();
        assert!(r1.url == url01.clone());

        let r2 = rr.next().unwrap();
        assert!(r2.url == url02.clone());

        let r3 = rr.next().unwrap();
        assert!(r3.url == url01.clone());
    }
}
