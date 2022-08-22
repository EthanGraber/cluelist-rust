use std::io;

enum Characters {
    Mustard,
    Plum,
    Scarlet,
    Peacock,
    Green,
    White,
}

enum Rooms {
    Hall,
    DiningRoom,
    Kitchen,
    Patio,
    Observatory,
    Theater,
    LivingRoom,
    Spa,
    GuestHouse,
}

enum Weapons {
    Knife,
    Candlestick,
    Pistol,
    Poison,
    Trophy,
    Rope,
    Bat,
    Ax,
    Dumbbell,
}

enum Card {
    Character(Characters),
    Room(Rooms),
    Weapon(Weapons),
}

struct Player {
    name: String,
    cards: Vec<Card>,
}

struct Guess {
    asked_by: String,
    character: Characters,
    room: Rooms,
    weapon: Weapons,
    disproven: bool,
    disprover: String,
}

fn main() {
    println!("Welcome to cluelist v0.0.1!");
    
    let mut players: Vec<Player> = Vec::new();
    let mut guesses: Vec<Guess> = Vec::new();
    setup(&mut players);
    
    println!("To see available commands, type 'help' or 'h'.");
    
    loop {
        let cmd = get_input_wrapper("-> ");
        
        match &cmd[..] {
            "help" | "h" => help(),
            "guess" | "g" => guess(&mut guesses),
            "quit" | "q" => break,
            _ => (),
        }
    }
}

fn help() {
    println!("todo: help content");
}

fn setup(players: &mut Vec<Player>) {
    let num_players = get_input_wrapper("Enter the number of players: ");
        
    let num_players: u8 = num_players
        .parse()
        .expect("Could not parse the number of players");
        
    let cards_per_player: u8 = 24 / num_players;
    
    for i in 0..num_players {
    
        let name = get_input_wrapper(&format!("Enter player {} name: ",i)[..]);
    
        for j in 0..cards_per_player {
            
        }
    }
}

fn get_input_wrapper(prompt: &str) -> String {
    loop {
        let mut out = String::new();
    
        println!("{}", prompt);
        match io::stdin().read_line(&mut out) {
            Ok(_) => return out.trim().to_string(),
            Err(_) => (),
        }
    }
} 

fn get_card_from_str(input: &str) -> Card {
    
    match &input.trim().to_lowercase()[..] {
        "mustard" => Card::Character(Characters::Mustard),
        "plum" => Card::Character(Characters::Plum),
        "scarlet" => Card::Character(Characters::Scarlet),
        "peacock" => Card::Character(Characters::Peacock),
        "green" => Card::Character(Characters::Green),
        "white" => Card::Character(Characters::White),
        "hall" => Card::Room(Rooms::Hall),
        "diningroom" => Card::Room(Rooms::DiningRoom),
        "kitchen" => Card::Room(Rooms::Kitchen),
        "observatory" => Card::Room(Rooms::Observatory),
        "theater" | "theatre" => Card::Room(Rooms::Theater),
        "livingroom" => Card::Room(Rooms::LivingRoom),
        "spa" => Card::Room(Rooms::Spa),
        "guesthouse" => Card::Room(Rooms::GuestHouse),
        "knife" => Card::Weapon(Weapons::Knife),
        "candlestick" => Card::Weapon(Weapons::Candlestick),
        "pistol" => Card::Weapon(Weapons::Pistol),
        "poison" => Card::Weapon(Weapons::Poison),
        "trophy" => Card::Weapon(Weapons::Trophy),
        "rope" => Card::Weapon(Weapons::Rope),
        "bat" => Card::Weapon(Weapons::Bat),
        "ax" | "axe" => Card::Weapon(Weapons::Ax),
        "dumbbell" => Card::Weapon(Weapons::Dumbbell),
        _ => panic!("not a card"), //todo: use option
    }
}

fn guess(guesses: &mut Vec<Guess>) {
    loop {
        let to_add = Guess {
            asked_by: get_input_wrapper("Which player is guessing? (Name or Index): "),
            character: get_card_from_str(get_input_wrapper("Character: ")),
            room: get_card_from_str(get_input_wrapper("Room: ")),
            weapon: get_card_from_str(get_input_wrapper("Weapon: ")),
            disproven: get_input_wrapper("Was the guess disproven? (y/n): ")
                            .to_lowercase() == "y".to_string() ? true : false,
            disprover: self.disproven ? String::new() : get_input_wrapper("Which player disproved? (Name or Index): "),
        }
    }
}