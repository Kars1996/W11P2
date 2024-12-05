class Enemy {
    maxHP: number;
    currentHP: number;
    level: number;
    readonly name: string;

    constructor(name: string, level: number, maxHp: number) {
        this.name = name;
        this.level = level;
        this.maxHP = maxHp;
        this.currentHP = maxHp;
    }

    Attack(): void {
        console.log(`${this.name} attacks!`);
    }

    Block(): void {
        console.log(`${this.name} blocks the attack!`);
    }

    Move(): void {
        console.log(`${this.name} moves swiftly!`);
    }
}

class Bandit extends Enemy {
    gang: string;
    wealth: number;

    constructor(
        name: string,
        level: number,
        maxHp: number,
        gang: string,
        wealth: number
    ) {
        super(name, level, maxHp);
        this.gang = gang;
        this.wealth = wealth;
    }

    Talk(): void {
        console.log(`${this.name} says, gimme me all of your stuff!'`);
    }
}

class Goblin extends Enemy {
    climb(): void {
        console.log(`${this.name} climbs a wall with ease.`);
    }

    eat(): void {
        console.log(`${this.name} chomps noisily on some food.`);
    }

    screech(): void {
        console.log(`${this.name} lets out a piercing screech!`);
    }
}

const bandit = new Bandit("Bandit Lord", 5, 100, "Set 2b", 500);
console.log(
    `${bandit.name} (Level ${bandit.level}): ${bandit.currentHP}/${bandit.maxHP} HP`
);
bandit.Attack();
bandit.Talk();

const goblin = new Goblin("Suspicious Goblin", 2, 50);
console.log(
    `${goblin.name} (Level ${goblin.level}): ${goblin.currentHP}/${goblin.maxHP} HP`
);
goblin.Move();
goblin.screech();
goblin.climb();
