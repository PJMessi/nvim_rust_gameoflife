use crate::internal::app_handler::AppHandler;
use rmpv::Value;

pub struct GameHandler {
    generation: u16,
    population: Vec<Vec<char>>,
    population_next_gen: Vec<Vec<char>>,
    default_block: char,
    cell_size: (usize, usize),
}

impl GameHandler {
    pub fn new() -> Self {
        let mut ins = Self {
            population: Vec::new(),
            population_next_gen: Vec::new(),
            generation: 0,
            default_block: '█',
            cell_size: (80, 120),
        };
        ins.initialize_grid();
        ins
    }
}

impl GameHandler {
    fn initialize_grid(&mut self) {
        let row: Vec<char> = vec![self.default_block; self.cell_size.1];
        self.population = (0..self.cell_size.0).map(|_| row.clone()).collect();
        self.population_next_gen = self.population.clone();

        let initial_live_cells = vec![
            (10, 10),
            (10, 11),
            (11, 10),
            (11, 11),
            (10, 20),
            (11, 20),
            (12, 20),
            (9, 21),
            (13, 21),
            (8, 22),
            (14, 22),
            (8, 23),
            (14, 23),
            (11, 24),
            (9, 25),
            (13, 25),
            (10, 26),
            (11, 26),
            (12, 26),
            (11, 27),
            (8, 30),
            (9, 30),
            (10, 30),
            (8, 31),
            (9, 31),
            (10, 31),
            (7, 32),
            (11, 32),
            (6, 34),
            (7, 34),
            (11, 34),
            (12, 34),
            (8, 44),
            (9, 44),
            (8, 45),
            (9, 45),
        ];
        for (x, y) in initial_live_cells {
            self.revive(x, y);
        }

        eprintln!("Population = {:?}", self.population);
    }

    fn vec_to_rmpv(&self, data: &Vec<Vec<char>>) -> Value {
        Value::Array(
            data.iter()
                .map(|row| Value::from(row.iter().collect::<String>()))
                .collect(),
        )
    }

    fn is_alive(&self, x: usize, y: usize) -> bool {
        self.population[x][y] == ' '
    }

    fn get_live_neighbour_count(&self, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;

        let initial_row = x.saturating_sub(1);
        let initial_col = y.saturating_sub(1);

        for i in initial_row..(initial_row + 3) {
            // Handle grid overflow.
            if i >= self.cell_size.0 {
                continue;
            }

            for j in initial_col..(initial_col + 3) {
                // Handle grid overflow.
                if j >= self.cell_size.1 {
                    continue;
                }

                // Ignore self.
                if i == x && j == y {
                    continue;
                }

                if self.is_alive(i, j) {
                    count += 1;
                }
            }
        }

        count
    }

    fn revive(&mut self, x: usize, y: usize) {
        self.population_next_gen[x][y] = ' ';
    }

    fn kill(&mut self, x: usize, y: usize) {
        self.population_next_gen[x][y] = self.default_block;
    }

    fn pass_generation(&mut self) {
        self.generation += 1;

        if self.generation == 1 {
            return;
        }

        let (x_len, y_len) = self.cell_size;

        for i in 0..x_len {
            for j in 0..y_len {
                if self.is_alive(i, j) {
                    let neighbour_count = self.get_live_neighbour_count(i, j);
                    if neighbour_count < 2 {
                        // Dies by lonliness.
                        self.kill(i, j);
                    } else if neighbour_count > 3 {
                        // Dies by overpopulation.
                        self.kill(i, j);
                    }
                } else {
                    let neighbour_count = self.get_live_neighbour_count(i, j);
                    if neighbour_count == 3 {
                        // Reproduction.
                        self.revive(i, j);
                    }
                }
            }
        }

        self.population = self.population_next_gen.clone();
    }
}

impl AppHandler for GameHandler {
    fn handle(&mut self) -> Result<rmpv::Value, rmpv::Value> {
        self.pass_generation();
        let grid_value = self.vec_to_rmpv(&self.population);
        Ok(grid_value)
    }
}
