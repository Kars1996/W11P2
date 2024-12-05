using System;

class Enemy
{
    public int currentHP { get; set; }
    public int maxHP { get; set; }
    public int level { get; set; }
    public string name { get; set; }

    public Enemy(string name, int level, int maxHp)
    {
        name = name;
        level = level;
        maxHP = maxHp;
        currentHP = maxHp;
    }

    public void Attack()
    {
        Console.WriteLine($"{name} attacks!");
    }
    public void Block()
    {
        Console.WriteLine($"{name} blocks the attack!");
    }
    public void Move()
    {
        Console.WriteLine($"{name} moves swiftly.");
    }
}

class Bandit : Enemy
{
    public string gang { get; set; }
    public int wealth { get; set; }

    public Bandit(string name, int level, int maxHp, string gang, int wealth)
        : base(name, level, maxHp)
    {
        this.gang = gang;
        this.wealth = wealth;
    }

    public void Talk()
    {
        Console.WriteLine($"{name} says, 'This is my turf!'");
    }
}

class Goblin : Enemy
{
    public Goblin(string name, int level, int maxHp) : base(name, level, maxHp) { }

    public void Climb()
    {
        Console.WriteLine($"{name} climbs a wall with ease.");
    }

    public void Eat()
    {
        Console.WriteLine($"{name} chomps noisily on some food.");
    }

    public void Screech()
    {
        Console.WriteLine($"{name} lets out a piercing screech!");
    }
}

class Program
{
    public static void Main(string[] args)
    {
        Bandit bandit = new Bandit("Bandit King", 5, 100, "Black Scorpions", 500);
        Console.WriteLine($"{bandit.name} (Level {bandit.level}): {bandit.currentHP}/{bandit.maxHP} HP");
        bandit.Attack();
        bandit.Talk();

        Goblin goblin = new Goblin("Sneaky Goblin", 2, 50);
        Console.WriteLine($"{goblin.name} (Level {goblin.level}): {goblin.currentHP}/{goblin.maxHP} HP");
        goblin.Move();
        goblin.Screech();
        goblin.Climb();
    }
}