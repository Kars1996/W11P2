abstract class Enemy(val name: String, val level: Int, val maxHp: Int) {
    var currentHp: Int = maxHp

    abstract fun attack()
    abstract fun block()
    abstract fun move()
}

class Bandit(
    name: String, level: Int, maxHp: Int,
    private val gang: String, private val wealth: Int
) : Enemy(name, level, maxHp) {

    override fun attack() {
        println("$name from $gang attacks with ferocity!")
    }

    override fun block() {
        println("$name blocks with agility.")
    }

    override fun move() {
        println("$name moves to a new position stealthily.")
    }

    fun talk() {
        println("$name says, 'This is my turf!'")
    }
}

class Goblin(name: String, level: Int, maxHp: Int) : Enemy(name, level, maxHp) {
    override fun attack() {
        println("$name lunges forward and attacks!")
    }

    override fun block() {
        println("$name hides behind a rock to block.")
    }

    override fun move() {
        println("$name scuttles around quickly.")
    }

    fun climb() {
        println("$name climbs a tree with ease.")
    }

    fun eat() {
        println("$name chomps noisily on some food.")
    }

    fun screech() {
        println("$name lets out a piercing screech!")
    }
}

fun main() {
    val bandit = Bandit("Bandit King", 5, 100, "Black Scorpions", 500)
    bandit.attack()
    bandit.talk()

    val goblin = Goblin("Sneaky Goblin", 2, 50)
    goblin.move()
    goblin.screech()
    goblin.climb()
}
