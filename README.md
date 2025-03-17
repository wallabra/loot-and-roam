# Loot & Roam

> Try as you might, even above the water line there are bigger fish...

![Loot & Roam logo](media/logo/Loot%20&%20Roam%20monochrome.png "Loot & Roam logo")


A physics-based steampunk pirate game. A roguelike action RPG. Some would even
say an immersive sim.

Engage in naval combat, explore a procedurally generated world, and manage a
thriving ship and hungry crew, navigating to far away yet bountiful islands
while evading patrols and warlords - in the pursuit of glory, wealth, or a
new world... or all of the above!

Currently in development by the GameCircular game development cooperative.
Stay tuned! ;)


## Installation

The game is not yet in a playable state. There is a playable prototype, which
is a previous project, which can be played on the Web at
https://wallabra.github.io/proto-lnr - it is compatible with all HTML5 browsers
with HTML5 Canvas support. Note that this prototype is not very optimized for
performance!


## Building

A Rust compiler is required to build this project. It is recommended to use the
Cargo package manager. It makes things really simple!

While there isn't a playable game *yet*, there are a few tech demos available
to try out. Progress in developing engine features can be gauged this way. In
order of release:

* **Cube softbody demo** - a demonstration of the point-spring network based
  soft body physics simulation, which will be fairly universally used (although
  many objects will be springless single points).  
  `cargo run --example soft-cube`

* **Cube softbody collision demo** - a demonstration of the volume primitive
  based collision system.  
  `cargo run --example soft-cube-collision`


## Contributing

If you want to contribute to the project, it might be worth checking out our
shiny new [Contribution Guidelines](CONTRIBUTING.md). :)


## Licensing Information

(c)2025 GameCircular.

All project resources, including source code and assets, are available
under the Cooperative Non-Violent Public License.

Loot & Roam is non-violent software: you can use, redistribute,
and/or modify it under the terms of the CNPLv6+ as found
in the LICENSE file in the source code root directory or
at https://git.pixie.town/thufie/CNPL.

Loot & Roam comes with ABSOLUTELY NO WARRANTY, to the extent
permitted by applicable law.  See the CNPL for details.
