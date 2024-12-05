trait Enemy {
    fn attack(&self);
    fn block(&self);
    fn move_character(&self);
}

struct Bandit {
    name: String,
    level: u32,
    max_hp: u32,
    current_hp: u32,
    gang: String,
    wealth: u32,
}

impl Enemy for Bandit {
    fn attack(&self) {
        println!("{} from {} attacks with ferocity!", self.name, self.gang);
    }

    fn block(&self) {
        println!("{} blocks with agility.", self.name);
    }

    fn move_character(&self) {
        println!("{} moves to a new position stealthily.", self.name);
    }
}

impl Bandit {
    fn new(name: &str, level: u32, max_hp: u32, gang: &str, wealth: u32) -> Self {
        Bandit {
            name: name.to_string(),
            level,
            max_hp,
            current_hp: max_hp,
            gang: gang.to_string(),
            wealth,
        }
    }

    fn talk(&self) {
        println!("{} says, 'This is my turf!'", self.name);
    }
}

struct Goblin {
    name: String,
    level: u32,
    max_hp: u32,
    current_hp: u32,
}

impl Enemy for Goblin {
    fn attack(&self) {
        println!("{} lunges forward and attacks!", self.name);
    }

    fn block(&self) {
        println!("{} hides behind a rock to block.", self.name);
    }

    fn move_character(&self) {
        println!("{} scuttles around quickly.", self.name);
    }
}

impl Goblin {
    fn new(name: &str, level: u32, max_hp: u32) -> Self {
        Goblin {
            name: name.to_string(),
            level,
            max_hp,
            current_hp: max_hp,
        }
    }

    fn climb(&self) {
        println!("{} climbs a tree with ease.", self.name);
    }

    fn eat(&self) {
        println!("{} chomps noisily on some food.", self.name);
    }

    fn screech(&self) {
        println!("{} lets out a piercing screech!", self.name);
    }
}

fn main() {
    let bandit = Bandit::new("Bandit King", 5, 100, "Black Scorpions", 500);
    bandit.attack();
    bandit.talk();

    let goblin = Goblin::new("Sneaky Goblin", 2, 50);
    goblin.move_character();
    goblin.screech();
    goblin.climb();
}
