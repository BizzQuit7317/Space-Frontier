# Space-Frontier
A game made in gadot game engine, the style and visual are ripped from SPACEPLAN on steam.
Please check it out!

Here is a rough repo to hold thoughts and general structure throughout development, it will also hold code and other parts that may or may not be used

# Game Description
Captain a galactic planet destoryer as you traverse the infinite expanse of the universe exploring and destroying all planets you come across.
See how many planets you eliminate before you get destroyed yourself!!!

# Concept
- A rougue like clicker game
- You run a space ship exploring galaxies
- When you start you get a random galaxy
- Each galaxy has a sun in the center with 6 planets
- Each planet and sun are randomly generated
- You will pick a planet and fly to it
- While orbiting the planet you can drop probes and other items
- You will progress through the planet essentially by destroying it
- you get points/score for destroying planets
- You use points to upgrade your space ship
- thats the game loop

  # Finer Details
  - Each sun will have a different size and no gravity
  - Each planet will have potential for astroids that can kill you
  - Some planets will also have inhabitant that can fight back
  - Each planet will be wrapped with a perlin noise generator for terrain
  - Each planet will have key facotrs like mass, rotation speed
  - The space ship will also have mass and speed
  - The game should use physics to move things, doesnt need to be accurate
  - For example before entering a planet you need to ensure you have enough speed so you'll send a probe or something
  - The planets random values should scale with your ship
