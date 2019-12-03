#[derive(Debug)]
pub struct Server {
    url: String,
}

impl Server {
    pub fn new(url: String) -> Server {
        return Server { url };
    }
}

#[derive(Debug)]
pub struct RoundRobinBalancer {
    pub servers: Vec<Server>,
    cur_idx: usize,
}

impl RoundRobinBalancer {
    pub fn new() -> RoundRobinBalancer {
        return RoundRobinBalancer {
            servers: vec![],
            cur_idx: 0,
        };
    }

    pub fn insert_server(&mut self, server: Server) {
        self.servers.push(server);
    }

    pub fn insert_url(&mut self, url: String) {
        let server = Server::new(url);
        self.insert_server(server);
    }

    pub fn next(&mut self) -> Option<&Server> {
        let s = self.servers.get(self.cur_idx);
        self.cur_idx = (self.cur_idx + 1) % self.servers.len();
        s.clone()
    }
}

mod tests {
    use super::{RoundRobinBalancer, Server};
    #[test]
    fn test_simple_next() {
        let url01 = "http://localhost:8081".to_string();
        let url02 = "http://localhost:8082".to_string();
        let server01 = Server::new(url01.clone());

        let mut rr = RoundRobinBalancer::new();
        rr.insert_server(server01);
        rr.insert_url(url02.clone());

        let r1 = rr.next().unwrap();
        assert!(r1.url == url01.clone());

        let r2 = rr.next().unwrap();
        assert!(r2.url == url02.clone());

        let r3 = rr.next().unwrap();
        assert!(r3.url == url01.clone());
    }
}
