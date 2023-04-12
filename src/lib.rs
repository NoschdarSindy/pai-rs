use wasm_bindgen::prelude::*;

type Vec2D<T> = Vec<Vec<T>>;

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
}

const PLAYER_COLORS: [char; 8] = ['🟥', '🟦', '🆒', '🟧', '🟪', '🟨', '🟩', '🟫'];

#[derive(Clone, Default)]
struct Field {
    symbol: usize,
    open: bool,
}


#[wasm_bindgen]
#[derive(Default)]
pub struct Pairs {
    open: (Option<Position>, Option<Position>),
    field: Vec2D<Field>,
    field_size: usize,
    active_player: usize,
    player_points: Vec<usize>,
}

#[wasm_bindgen]
impl Pairs {

    #[wasm_bindgen(constructor)]
    pub fn create_grid(player_count: usize, field_size: usize) -> Self {
        let mut pairs = Self::default();
        pairs.field_size = field_size;
        pairs.field = Pairs::init_random_field(field_size);
        pairs.player_points = vec![0; player_count];
        pairs
    }

    fn init_random_field(field_size: usize) -> Vec2D<Field> {
        let mut pairs = Vec::new();
        for i in 0..((field_size * field_size) / 2) {
            let field = Field {
                symbol: i,
                open: false,
            };
            pairs.push(field.clone());
            pairs.push(field);
        }
        fastrand::shuffle(&mut pairs);

        let mut field: Vec2D<Field> = vec![vec![Field::default(); field_size]; field_size];
        for f in field.iter_mut().flatten() {
            *f = pairs.pop().unwrap();
        }
        field
    }

    #[wasm_bindgen(js_name = openField)]
    pub fn open_field(&mut self, x: usize, y: usize) -> bool {
        let field = &mut self.field[x][y];

        field.open = match field.open {
            true => {
                return false;
            }
            false => true,
        };

        match &self.open {
            (None, None) => self.open.0 = Some(Position { x, y }),
            (Some(other), None) => {
                if field.symbol == self.field[other.x][other.y].symbol {
                    self.player_points[self.active_player] += 1;
                    self.open = (None, None);
                } else {
                    self.open.1 = Some(Position { x, y });
                    self.active_player = (self.active_player + 1) % self.player_points.len();
                    return true;
                }
            }
            (Some(_), Some(_)) => {
                self.close_fields();
                self.open.0 = Some(Position { x, y });
            }
            _ => {}
        }
        return false;
    }

    #[wasm_bindgen(js_name = closeFields)]
    pub fn close_fields(&mut self) {
        if let (Some(first), Some(second)) = &self.open {
            self.field[first.x][first.y].open = false;
            self.field[second.x][second.y].open = false;
            self.open = (None, None);
        }
    }

    #[wasm_bindgen(js_name = getPoints)]
    pub fn get_points(&self) -> String {
        let mut final_string = String::new();
        for (index, points) in self.player_points.iter().enumerate() {
            final_string += &format!("{}: {}   ", PLAYER_COLORS[index], points);
        }
        final_string
    }

    #[wasm_bindgen(js_name = getState)]
    pub fn get_state(&self) -> String {
        let mut state = String::new();
        for x in 0..self.field_size {
            for y in 0..self.field_size {
                let field = &self.field[x][y];
                let symbol = match field.open {
                    true => char::from_u32(128053 + field.symbol as u32).unwrap(),
                    false => PLAYER_COLORS[self.active_player],
                };
                state += &format!("{} ", symbol);
            }
            state += "\n";
        }
        state
    }
}