use crate::player::Player;
pub fn valid_position(player: &Player) -> (u32, u32) {
    let enemy_positions = find_enemy_positions(&player.anfield, player.id);
    let mut possible_moves = Vec::new();
    for x in 0..player.anfield.len() {
        for y in 0..player.anfield[0].len() {
            let safe = can_place(x, y, &player.anfield, &player.piece, player.id);
            if safe {
                possible_moves.push((x as i32, y as i32));
            }
        }
    }
    if !possible_moves.is_empty() && !enemy_positions.is_empty() {
        let result = best_move(possible_moves, &enemy_positions);
        return (result.1 as u32, result.0 as u32);
    }
    (0, 0)
}
fn best_move(possible_moves: Vec<(i32, i32)>, enemy_positions: &[(usize, usize)]) -> (i32, i32) {
    let mut best_move = possible_moves[0];
    for &coord in possible_moves.iter() {
        let min_distance = enemy_positions.iter()
            .map(|&enemy_pos| distance(coord, (enemy_pos.0 as i32, enemy_pos.1 as i32)))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let current_min_distance = enemy_positions.iter()
            .map(|&enemy_pos| distance(best_move, (enemy_pos.0 as i32, enemy_pos.1 as i32)))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        if min_distance < current_min_distance {
            best_move = coord;
        }
    }
    best_move
}
pub fn distance(current: (i32, i32), enemy: (i32, i32)) -> f32 {
    let dx = (enemy.0 - current.0) as f32;
    let dy = (enemy.1 - current.1) as f32;
    (dx * dx + dy * dy).sqrt()
}
pub fn can_place(x: usize, y: usize, anfield: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, player_symbol: (char, char)) -> bool {
    let mut overlap = 0;
    let piece_height = piece.len();
    let piece_width = piece[0].len();
    let anfield_height = anfield.len();
    let anfield_width = anfield[0].len();
    for i in 0..piece_height {
        for j in 0..piece_width {
            let anfield_x = x + i;
            let anfield_y = y + j;
            if anfield_x >= anfield_height || anfield_y >= anfield_width {
                return false;
            }
            let anfield_char = anfield[anfield_x][anfield_y];
            if piece[i][j] != '.' {
                if anfield_char == player_symbol.0 || anfield_char == player_symbol.1 {
                    overlap += 1;
                } else if anfield_char != '.' {
                    return false;
                }
            }
        }
    }
    overlap == 1
}
pub fn find_enemy_positions(anfield: &Vec<Vec<char>>, player_symbol: (char, char)) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for x in 0..anfield.len() {
        for y in 0..anfield[0].len() {
            let cell = anfield[x][y];
            if cell != '.' && cell != player_symbol.0 && cell != player_symbol.1 {
                positions.push((x, y));
            }
        }
    }
    positions
}

