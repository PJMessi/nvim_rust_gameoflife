use crate::internal::app_handler::AppHandler;
use rand::Rng;
use rmpv::Value;

pub struct GameHandler {
    generation: u16,
    data: Vec<Vec<char>>,
    default_block: char,
}

impl GameHandler {
    pub fn new() -> Self {
        let mut ins = Self {
            data: Vec::new(),
            generation: 0,
            default_block: '█',
        };
        ins.initialize_grid();
        ins
    }
}

impl GameHandler {
    fn initialize_grid(&mut self) {
        let row: Vec<char> = vec![self.default_block; 100];
        self.data = (0..50).map(|_| row.clone()).collect();
    }

    fn vec_to_rmpv(data: &Vec<Vec<char>>) -> Value {
        Value::Array(
            data.iter()
                .map(|row| Value::from(row.iter().collect::<String>()))
                .collect(),
        )
    }

    fn get_random_coordinates(&self) -> (usize, usize)  {
        let mut rng = rand::rng();
        return (rng.random_range(0..50), rng.random_range(0..100))
    }

    fn pass_generation(&mut self) {
        let (x, y) = self.get_random_coordinates();
        self.data[x][y] = ' ';
    }
}

impl AppHandler for GameHandler {
    fn handle(&mut self) -> Result<rmpv::Value, rmpv::Value> {
        eprintln!("generation {}", self.generation);
        self.generation += 1;
        self.pass_generation();
        let grid_value = GameHandler::vec_to_rmpv(&self.data);
        Ok(grid_value)
    }
}
