use borrow_box::*;

fn main() {
    let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
    println!("{:?}", game.read_winner());

    game.update_score("Joao");
    game.update_score("Joao");
    game.update_score("Susana");
    game.update_score("Susana");
    println!("{:?}", game.read_winner());

    game.update_score("Joao");
    game.update_score("Susana");

    println!("{:?}", game.read_winner());

    println!("{:?}", game.delete());

    
}