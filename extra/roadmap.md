# Roadmap

This is an extremely broad development roadmap. Currently, no timeline
has been set, until a more concerted and sustained development effort can
be assembled.

Suggestions and feedback are welcome; this is by no means a definitive or
final roadmap!

## Current Plans

The top level items are in **bold**. They themselves can be fulfilled in any
order. Their sub-items are only loosely ordered; that they be implemented in
that order is not a requirement per se.

### **Physics**
  * [x] Physics points and springs
  * [ ] Volumes, for collision and volume phenomena
  * [ ] Air and fluid drag
  * [ ] Water, and water buoyancy
  * [ ] Utility methods for projectile physics (air time and hit location predictors)
  
### **Terrain**
  * [ ] Define terrain nodes
  * [ ] Heightmap, signed distance field
  
### **Rendering**
  * [ ] Terrain renderer (raymarcher)
  * [ ] Command-oriented UI renderer
  
### **Game**
  * [ ] Non-player ship AI with states
    * [ ] Collision avoidance of some sort
    * Note: possibly consider Rain World's NPC AI for study?
      It is more organic than the NPC AI in our Loot & Roam prototype,
      but still clearly has some notion of state and state transition.
      Unlike the prototype, we do this time want to have some degree of
      multi-tasking, for instance, an NPC might want to swoop in for items
      laying around even while fleeing. This is not possible in a rigid
      state machine based NPC system.
  * [ ] Ship part code
  * [ ] Island archetypes, with their own props and themes
  * [ ] Stationary props like turrets, buildings that drop loot when destroyed,
        or island decor
  * [ ] Ship crew, food consumption, strikes when hungry/unpaid
  * [ ] Definition/design system for parts, makes, items, etc
    * This allows design to be decoupled from the programming itself.
      How this will be done is yet unclear.
  * [ ] Internationalization system
  * [ ] Superstates, aka top-level game states (menu, play, etc)
  * [ ] Purely immediate-mode reactive UI API

The current plans focus on catching up with the prorotype. You can see the
prototype here for comparison:
https://wallabra.github.io/proto-lnr

For more details, there are notes scattered across the source code. In
paricular, look for comments marked with  [TODO]'.

There are also other plans in the `design/` subdirectory. They are, however,
merely tentative. Not only may they be changed at a future date; it MUST be
assumed that they WILL.

## Long-Term Plans

The above roadmap is not comprehensive. It is, mostly, merely
catching up with progress on the prototype. Directions in which to expand
beyond are, as of yet, sadly unclear.

However, there are a few potential directions to discuss thoroughly before
setting in stone. Not at all a comprehensive list!

* **Game Modes** - besides the prototype's "Free Play", we could have a "Pirate
                   Mode" where stealth is more important and there is a quest
                   line.

  * Maybe a campaign mode too?

* **Rumor System** - despite being a roguelike, storing 'rumors' about previous
                     runs that can be overheard at a 'bar' in a town mode
                     island could be nice!

* **Modding** - almost certainly; important for community building

* **Multiplayer?** - will be hard due to the highly inertial nature of naval
                     physics. We might do something P2P / federated, with
                     basic routing. Maybe there's already a library for that!
