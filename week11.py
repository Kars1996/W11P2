class Enemy:
    def __init__(self, name: str, level: int, max_hp: int) -> None:
        self.Name = name
        self.Level = level
        self.Max_HP = max_hp
        self.Current_HP = max_hp

    def Attack(self) -> None:
        print(f"{self.Name} attacks!")

    def Block(self) -> None:
        print(f"{self.Name} blocks the attack!")

    def Move(self) -> None:
        print(f"{self.Name} moves with haste!")


class Bandit(Enemy):
    def __init__(
        self, name: str, level: int, max_hp: int, gang: str, wealth: int
    ) -> None:
        super().__init__(name, level, max_hp)
        self.Gang = gang
        self.Wealth = wealth

    def Talk(self) -> None:
        print(f"{self.Name} says, 'gimme me all of your stuff!'")


class Goblin(Enemy):
    def climb(self) -> None:
        print(f"{self.Name} climbs a wall.")

    def eat(self) -> None:
        print(f"{self.Name} chomps loudly on some food.")

    def screech(self) -> None:
        print(f"{self.Name} lets out a loud annoying screech!")


if __name__ == "__main__":
    bandit = Bandit(name="Bandit Lord", level=5, max_hp=100, gang="Set 2b", wealth=500)
    print(
        f"{bandit.Name} (Level {bandit.Level}): {bandit.Current_HP}/{bandit.Max_HP} HP"
    )
    bandit.Attack()
    bandit.Talk()

    goblin = Goblin(name="Suspicious Goblin", level=2, max_hp=50)
    print(
        f"{goblin.Name} (Level {goblin.Level}): {goblin.Current_HP}/{goblin.Max_HP} HP"
    )
    goblin.Move()
    goblin.screech()
    goblin.climb()
