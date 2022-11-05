fn main() {
    let grid = find_routes(vec![
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ]);

    println!("found {} possible routes", &grid.routes.len());
    for route in &grid.routes {
        println!("{:?}", route);
    }
}

fn find_routes(g: Vec<Vec<u8>>) -> Grid {
    let mut grid = Grid {
        size: [g.len() as u8, g[0].len() as u8],
        routes: vec![],
        grid: g,
    };

    let start_points = &grid.initial();

    let target: usize = start_points.len().try_into().unwrap();

    for point in start_points.iter() {
        grid.recursion(point, vec![], target);
    }
    grid
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
    size: [u8; 2],
    routes: Vec<Vec<[u8; 2]>>,
}

impl Grid {
    fn access(&self, &pos: &[u8; 2]) -> u8 {
        self.grid[pos[1] as usize][pos[0] as usize]
    }

    fn initial(&self) -> Vec<[u8; 2]> {
        let mut initial: Vec<[u8; 2]> = vec![];

        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                if self.access(&[x, y]) == 1 {
                    continue;
                }

                initial.push([x, y])
            }
        }
        initial
    }

    fn connected(&self, &pos: &[u8; 2]) -> Vec<[u8; 2]> {
        let mut connected: Vec<[u8; 2]> = vec![];

        if pos[1] >= 1 {
            let con = [pos[0], pos[1] - 1];
            if self.access(&con) == 0 {
                connected.push(con);
            }
        }
        if pos[0] >= 1 {
            let con = [pos[0] - 1, pos[1]];
            if self.access(&con) == 0 {
                connected.push(con);
            }
        }
        if pos[0] < self.size[0] - 1 {
            let con = [pos[0] + 1, pos[1]];
            if self.access(&con) == 0 {
                connected.push(con);
            }
        }
        if pos[1] < self.size[1] - 1 {
            let con = [pos[0], pos[1] + 1];
            if self.access(&con) == 0 {
                connected.push(con);
            }
        }
        connected
    }

    fn recursion(&mut self, current: &[u8; 2], route: Vec<[u8; 2]>, target: usize) {
        if route.len() == target {
            if self.routes.iter().any(|a| a == &route) {
            } else {
                self.routes.push(route);
            }
        } else {
            let current_available = &self.connected(&current);

            for i in 0..current_available.len() {
                let c_avl = current_available[i];
                if route.iter().any(|&a| a == c_avl) {
                    continue;
                }
                let next_available = &self.connected(&c_avl);
                if next_available.len() > 0 {
                    let mut update_route = route.clone();
                    update_route.push(c_avl.clone());
                    self.recursion(&c_avl, update_route, target);
                }
            }
        }
    }
}
