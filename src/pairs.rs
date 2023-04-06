#![allow(unused)]

use std::fmt::Display;

type Vec2D<T> = Vec<Vec<T>>;

#[derive(Clone, Debug)]
struct Vec2<T> {
    x: T,
    y: T,
}
type Position = Vec2<usize>;

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
        }
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

    pub fn open(&mut self, x: usize, y: usize) {
        if let None = self.field[x][y] {
            return;
        }

        match &self.open {
            // Bisher ist noch kein Feld aufgedeckt
            (None, None) => {
                self.field[x][y].as_mut().unwrap().open = true;
                self.open.0 = Some(Position { x, y });
            }

            // Bisher ist nur eine Karte aufgedeckt
            (Some(first), None) => {
                // Die gleiche Karte wurde ausgewählt
                if first.x == x && first.y == y {
                    return;
                }
                // Wurde ein Paar gefunden?
                let first_symbol = self.field[first.x][first.y].as_ref().unwrap().symbol;
                let second_symbol = self.field[x][y].as_ref().unwrap().symbol;
                if first_symbol == second_symbol {
                    self.field[first.x][first.y] = None;
                    self.field[x][y] = None;
                    self.open = (None, None);
                } else {
                    self.field[x][y].as_mut().unwrap().open = true;
                    self.open.1 = Some(Position { x, y });
                }
            }

            // Es sind bereits zwei Karten aufgedeckt
            (Some(first), Some(second)) => {
                // Die alten Karten wieder verdecken
                self.field[first.x][first.y].as_mut().unwrap().open = false;
                self.field[second.x][second.y].as_mut().unwrap().open = false;
                // Die neue Karte aufdecken
                self.field[x][y].as_mut().unwrap().open = true;
                self.open = (Some(Position { x, y }), None);
            }
            _ => {}
        }
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
                            f.write_str("🟪 ");
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
