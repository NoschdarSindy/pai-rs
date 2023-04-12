#[cfg(test)]
mod tests {
    use crate::*;

    //===Helper===

    fn create_pairs(field_size: usize, player_count: usize) -> Pairs {
        Pairs::create_grid(player_count, field_size)
    }

    fn find_card(field: &Vec2D<Field>, x:  usize, y: usize, matching: bool) -> Option<(usize, usize)> {
        let given_card = field[x][y].symbol;
        for i in 0..field.len() {
            for j in 0..field.len() {
                let f = &field[i][j];
                if (matching == (f.symbol == given_card)) && (y != j || x != i) {
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

        assert_eq!(pairs.field_size, field_size);

        assert_eq!(*pairs.player_points, vec![0; player_count]);

        assert_eq!(pairs.field.len(), field_size);
        assert_eq!(pairs.field.len(), field_size);
    }

    // Spieler soll nach Aufdecken zwei gleicher Karten einen Punkt bekommen
    #[test]
    fn test_open_matching_pairs() {
        let field_size = 4;
        let player_count = 2;
        let mut pairs = create_pairs(field_size, player_count);

        let (i,j) = find_card(&pairs.field, 0,0, true).unwrap();

        pairs.open_field(0, 0);
        pairs.open_field(i, j);
        assert_eq!(pairs.field[0][0].open, true);
        assert_eq!(pairs.field[i][j].open, true);

        assert_eq!(pairs.player_points[pairs.active_player], 1);
    }

    // Bei Mismatch soll der nächste Spieler drankommen und beide Karten wieder umgedreht werden
    #[test]
    fn test_open_non_matching_pairs() {
        let field_size = 4;
        let player_count = 2;
        let mut pairs = create_pairs(field_size, player_count);

        let (i,j) = find_card(&pairs.field, 0,0, false).unwrap();

        pairs.open_field(0, 0);
        pairs.open_field(i, j);

        assert_eq!(pairs.field[0][0].open, true);
        assert_eq!(pairs.field[i][j].open, true);

        assert_eq!(pairs.active_player, 1);

        pairs.close_fields();
        assert_eq!(pairs.field[0][0].open, false);
        assert_eq!(pairs.field[i][j].open, false);
    }

    // Aktualisierter Punktestand soll korrekt dargestellt werden
    #[test]
    fn test_get_player_points() {
        let field_size = 4;
        let player_count = 3;
        let mut pairs = create_pairs(field_size, player_count);

        let (i,j) = find_card(&pairs.field, 0,0, true).unwrap();

        pairs.open_field(0, 0);
        pairs.open_field(i, j);

        assert_eq!(pairs.get_points(), "🟥: 1   🟦: 0   🆒: 0   ");
    }
}