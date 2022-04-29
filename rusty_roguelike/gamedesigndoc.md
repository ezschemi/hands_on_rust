# Game Design Doc

## Name
Stick with "Rusty Roguelike".

## Short Description
This game is a dungeon crawler, with an adventurer arriving in a randomly generated dungeon. The player guides the adventurer around the dungeon, defeating monsters and collecting power-ups until they find the Amulet of Yala ("Yet Another Lost Amulet") and win the game. 

## Story
The hero's hometown is suffering from a plague of monsters. Welling up from the deep, they seem unstoppable. Legend tells of the Amulet of Yala that can be used to stem the tide. After a long night at the tavern, the hero promises to save the day - and sets forth into the dungeon.

## Basic Game Loops
1. Enter the dungeon level
2. Explore, revealing the map.
3. Encounter enemies whom the player fights or flees from.
4. Find power-ups and use them to strengthen the player.
5. Locate the exit to the level - to to 1.

## Minimum Viable Product
1. Create a basic dungeon map
2. Place the player and let them walk around
3. Spawn monsters, draw them, and let the player kill them by walking into them.
4. Add health and a combat system that uses it.
5. Add healing potions.
6. Display a "Game Over"-screen when the player dies.
7. Add the Amulet of Yala to the level and let the player win by reaching it.

## Stretch Goals
1. Add field of view.
2. Add more interesting dungeon designs.
3. Add some dungeon themes.
4. Add multiple layers to a dungeon, with the Amulet on the last one.
5. Add varied weapons to the game.
6. Move to a data-driven design for spawning enemies.
7. Consider some visual effects to make combat more visceral.
8. Consider keeping score.
9. Add gamepad support.

