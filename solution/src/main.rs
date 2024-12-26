mod player;
mod solver;
use player::{ Player};
fn main() {
    let mut player = Player::new();
    loop {
        {
            player.add_id();
            player.add_anfield();
            player.add_piece();
            player.solve();
        }
        
    }
}