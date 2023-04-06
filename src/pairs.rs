#![allow(unused)]

use std::fmt::Display;

type Vec2D<T> = Vec<Vec<T>>;

#[derive(Clone, Debug)]
struct Vec2<T> {
    x: T,
    y: T,
}
type Position = Vec2<usize>;


const PLAYER_COLORS: [char; 8] = ['🟥', '🟦', '🆒', '🟧', '🟪', '🟨', '🟩', '🟫'];


#[derive(Clone, Debug)] 
struct Field {
    symbol: usize,
    open: bool,
}

#[derive(Debug)]
pub struct Pairs {
    cols: usize,
    rows: usize,
    open: (Option<Position>, Option<Position>),
    field: Vec2D<Option<Field>>,
    player_count: usize,
    active_player: usize,
    player_points: Vec<usize>,
}

impl Pairs {
    pub fn new(cols: usize, rows: usize) -> Self {
        if (cols * rows) % 2 != 0 {
            panic!("Even number of cards is needed!");
        }
        Self {
            cols,
            rows,
            open: (None, None),
            field: Pairs::init_random_field(cols, rows),
            player_count: 1,
            active_player: 0,
            player_points: Vec::new(),
        }
    }


    pub fn create(&mut self, player_count: usize, field_size: usize) {
        self.cols = field_size;
        self.rows = field_size;
        self.player_count = player_count;
        self.field = Pairs::init_random_field(field_size, field_size);
        self.player_points = vec![0; player_count];
    }


    fn init_random_field(cols: usize, rows: usize) -> Vec2D<Option<Field>> {
        let mut pairs = Vec::new();
        for i in 0..((cols * rows) / 2) {
            let field = Field {
                symbol: i,
                open: false,
            };
            pairs.push(field.clone());
            pairs.push(field);
        }
        fastrand::shuffle(&mut pairs);

        let mut field: Vec2D<Option<Field>> = vec![vec![None; rows]; cols];
        for f in field.iter_mut().flatten() {
            *f = pairs.pop();
        }
        field
    }


    pub fn open(&mut self, x: usize, y: usize) -> bool {

        if let Some(field) = &mut self.field[x][y] {
            // Karte aufdecken, falls sie es noch nicht ist
            field.open = match field.open {
                true => { return false; },
                false => true,
            };
        } else {
            return false;
        }

        match &self.open {
            // Bisher ist noch kein Feld aufgedeckt
            (None, None) => {
                self.open.0 = Some(Position { x, y });
            }

            // Bisher ist nur eine Karte aufgedeckt
            (Some(first), None) => {
                // Die gleiche Karte wurde ausgewählt
                if first.x == x && first.y == y {
                    return false;
                }
                // Wurde ein Paar gefunden?
                let first_symbol = self.field[first.x][first.y].as_ref().unwrap().symbol;
                let second_symbol = self.field[x][y].as_ref().unwrap().symbol;
                if first_symbol == second_symbol {
                    self.player_points[self.active_player] += 1;
                    self.open = (None, None);
                } else {
                    self.open.1 = Some(Position { x, y });
                    self.active_player = (self.active_player + 1) % self.player_count;
                    return true;
                }
            }

            // Es sind bereits zwei Karten aufgedeckt
            (Some(first), Some(second)) => {
                // Die alten Karten wieder verdecken
                self.field[first.x][first.y].as_mut().unwrap().open = false;
                self.field[second.x][second.y].as_mut().unwrap().open = false;
                // Die neue Karte als aufgedeckt markieren
                self.open = (Some(Position { x, y }), None);
            }
            _ => {}
        }
        return false;
    }


    pub fn close_all(&mut self) {
        if let (Some(first), Some(second)) = &self.open {
            self.field[first.x][first.y].as_mut().unwrap().open = false;
            self.field[second.x][second.y].as_mut().unwrap().open = false;
            self.open = (None, None);
        }
    }


    pub fn get_player_points(&mut self) -> String { 
        let mut final_string = String::new();
        for (index, points) in self.player_points.iter().enumerate() {
            final_string += &format!("{}: {}   ", PLAYER_COLORS[index], points);
        }
        final_string
    }


}


impl Display for Pairs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.cols {
            for y in 0..self.rows {
                if let Some(field) = &self.field[x][y] {
                    match field.open {
                        true => {
                            let symbol = char::from_u32(128053 + field.symbol as u32).unwrap();
                            f.write_str(&format!("{} ", symbol));
                        }
                        false => {
                            f.write_str(&format!("{} ", PLAYER_COLORS[self.active_player]));
                        }
                    }
                } else {
                    f.write_str("⬜ ");
                }
            }
            f.write_str("\n");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_random_field() {
        let pairs = Pairs::new(8, 8);
        println!("{:?}", pairs.field);
        println!("{}", pairs);
    }
}
