#[derive(Debug)]
pub struct Server {
    weight: i32,
    effect_weight: i32,
    cur_weight: i32,
    pub url: String,
}

impl Server {
    pub fn new(url: String, weight: i32) -> Server {
        return Server {
            effect_weight: weight,
            cur_weight: 0,
            weight,
            url,
        };
    }
}

#[derive(Debug)]
pub struct WeightedRoundRobinBalancer {
    pub servers: Vec<Server>,
}

impl WeightedRoundRobinBalancer {
    pub fn new() -> WeightedRoundRobinBalancer {
        return WeightedRoundRobinBalancer { servers: vec![] };
    }

    pub fn insert_server(&mut self, server: Server) {
        self.servers.push(server);
    }

    pub fn insert_url(&mut self, url: String, weight: i32) {
        let server = Server::new(url, weight);
        self.insert_server(server);
    }

    pub fn next(&mut self) -> Option<&Server> {
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
        let mut best = self.servers.get_mut(best_idx).unwrap();
        best.cur_weight = best.cur_weight - total;

        Some(best)
    }
}
