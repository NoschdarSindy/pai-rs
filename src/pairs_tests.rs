#[cfg(test)]
mod tests {
    use crate::pairs::{Field, Pairs, Vec2D};

    //===Helper===

    fn create_pairs(field_size: usize, player_count: usize) -> Pairs {
        let mut pairs = Pairs::default();
        pairs.create(player_count, field_size);
        pairs
    }

    fn find_matching_card(field: &Vec2D<Field>, x:  usize, y: usize, matching: bool) -> Option<(usize, usize)> {
        let card_to_match = field[x][y].get_symbol();
        for i in 0..field.len() {
            for j in 0..field.len() {
                let f = &field[i][j];
                if (matching == (f.get_symbol() == card_to_match)) && (y != j || x != i) {
                    return Some((i, j));
                }
            }
        }
        None
    }

    //===Tests===

    // Alle Werte und Dimensionen sollen richtig initialisiert werden
    #[test]
    fn test_create_pairs() {
        let field_size = 4;
        let player_count = 2;
        let pairs = create_pairs(field_size, player_count);

        assert_eq!(pairs.get_field_size(), field_size);

        assert_eq!(*pairs.get_player_points(), vec![0; player_count]);

        assert_eq!(pairs.get_field().len(), field_size);
        assert_eq!(pairs.get_field()[0].len(), field_size);
    }

    // Spieler soll nach Aufdecken zwei gleicher Karten einen Punkt bekommen
    #[test]
    fn test_open_matching_pairs() {
        let field_size = 4;
        let player_count = 2;
        let mut pairs = create_pairs(field_size, player_count);

        let (i,j) = find_matching_card(pairs.get_field(), 0,0, true).unwrap();

        pairs.open(0, 0);
        pairs.open(i, j);
        assert_eq!(pairs.get_field()[0][0].is_open(), true);
        assert_eq!(pairs.get_field()[i][j].is_open(), true);

        assert_eq!(pairs.get_player_points()[pairs.get_active_player()], 1);
    }

    // Bei Mismatch soll der nächste Spieler drankommen und beide Karten wieder umgedreht werden
    #[test]
    fn test_open_non_matching_pairs() {
        let field_size = 4;
        let player_count = 2;
        let mut pairs = create_pairs(field_size, player_count);

        let (i,j) = find_matching_card(pairs.get_field(), 0,0, true).unwrap();

        pairs.open(0, 0);
        pairs.open(i, j);

        assert_eq!(pairs.get_field()[x1][y1].is_open(), true);
        assert_eq!(pairs.get_field()[x2][y2].is_open(), true);

        assert_eq!(pairs.get_active_player(), 1);

        pairs.close_all();
        assert_eq!(pairs.get_field()[x1][y1].is_open(), false);
        assert_eq!(pairs.get_field()[x2][y2].is_open(), false);
    }

    // Aktualisierter Punktestand soll korrekt dargestellt werden
    #[test]
    fn test_get_player_points() {
        let field_size = 4;
        let player_count = 3;
        let mut pairs = create_pairs(field_size, player_count);

        let (i,j) = find_matching_card(pairs.get_field(), 0,0, true).unwrap();

        pairs.open(0, 0);
        pairs.open(i, j);

        assert_eq!(pairs.player_points_to_str(), "🟥: 1   🟦: 0   🆒: 0   ");
    }
}