#include <iostream>
#include <string>
using namespace std;

class Enemy {
protected:
    int Current_HP, Max_HP, Level;
    string Name;

public:
    Enemy(string name, int level, int maxHp)
        : Name(name), Level(level), Max_HP(maxHp), Current_HP(maxHp) {}

    virtual void Attack() = 0; 
    virtual void Block() = 0;
    virtual void Move() = 0;

    virtual ~Enemy() {}
};

class Bandit : public Enemy {
private:
    string Gang;
    int Wealth;

public:
    Bandit(string name, int level, int maxHp, string gang, int wealth)
        : Enemy(name, level, maxHp), Gang(gang), Wealth(wealth) {}

    void Attack() override {
        cout << Name << " from " << Gang << " attacks with ferocity!\n";
    }

    void Block() override {
        cout << Name << " blocks with agility.\n";
    }

    void Move() override {
        cout << Name << " moves to a new position stealthily.\n";
    }

    void Talk() {
        cout << Name << " says, 'This is my turf!'\n";
    }
};

class Goblin : public Enemy {
public:
    Goblin(string name, int level, int maxHp) : Enemy(name, level, maxHp) {}

    void Attack() override {
        cout << Name << " lunges forward and attacks!\n";
    }

    void Block() override {
        cout << Name << " hides behind a rock to block.\n";
    }

    void Move() override {
        cout << Name << " scuttles around quickly.\n";
    }

    void Climb() {
        cout << Name << " climbs a tree with ease.\n";
    }

    void Eat() {
        cout << Name << " chomps noisily on some food.\n";
    }

    void Screech() {
        cout << Name << " lets out a piercing screech!\n";
    }
};

int main() {
    Bandit bandit("Bandit King", 5, 100, "Black Scorpions", 500);
    bandit.Attack();
    bandit.Talk();

    Goblin goblin("Sneaky Goblin", 2, 50);
    goblin.Move();
    goblin.Screech();
    goblin.Climb();

    return 0;
}
