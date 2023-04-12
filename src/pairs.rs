use std::fmt::Display;

pub type Vec2D<T> = Vec<Vec<T>>;

#[derive(Clone, Debug)]
struct Vec2<T> {
    x: T,
    y: T,
}
type Position = Vec2<usize>;

const PLAYER_COLORS: [char; 8] = ['🟥', '🟦', '🆒', '🟧', '🟪', '🟨', '🟩', '🟫'];

#[derive(Clone, Debug, Default)]
pub struct Field {
    symbol: usize,
    open: bool,
}

impl Field {
    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn get_symbol(&self) -> usize {
        self.symbol
    }
}

#[derive(Debug, Default)]
pub struct Pairs {
    open: (Option<Position>, Option<Position>),
    field: Vec2D<Field>,
    field_size: usize,
    active_player: usize,
    player_points: Vec<usize>,
}

impl Pairs {
    pub fn create(&mut self, player_count: usize, field_size: usize) {
        self.field_size = field_size;
        self.field = Pairs::init_random_field(field_size);
        self.player_points = vec![0; player_count];
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

    pub fn open(&mut self, x: usize, y: usize) -> bool {
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
                self.close_all();
                self.open.0 = Some(Position { x, y });
            }
            _ => {}
        }
        return false;
    }

    pub fn close_all(&mut self) {
        if let (Some(first), Some(second)) = &self.open {
            self.field[first.x][first.y].open = false;
            self.field[second.x][second.y].open = false;
            self.open = (None, None);
        }
    }

    pub fn player_points_to_str(&self) -> String {
        let mut final_string = String::new();
        for (index, points) in self.player_points.iter().enumerate() {
            final_string += &format!("{}: {}   ", PLAYER_COLORS[index], points);
        }
        final_string
    }

    pub fn get_field_size(&self) -> usize {
        self.field_size
    }

    pub fn get_field(&self) -> &Vec2D<Field> {
        &self.field
    }

    pub fn get_active_player(&self) -> usize {
        self.active_player
    }

    pub fn get_player_points(&self) -> &Vec<usize> {
        &self.player_points
    }
}

impl Display for Pairs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.field_size {
            for y in 0..self.field_size {
                let field = &self.field[x][y];
                let symbol = match field.open {
                    true => char::from_u32(128053 + field.symbol as u32).unwrap(),
                    false => PLAYER_COLORS[self.active_player],
                };
                f.write_str(&format!("{} ", symbol)).unwrap();
            }
            f.write_str("\n").unwrap();
        }
        Ok(())
    }
}
