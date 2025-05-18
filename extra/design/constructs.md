# Constructs

Objects like ships, attack towers, et cetera, can have 'parts' installed into
them, according to their part slot definitions. Those parts come with many
capabilities; cannons fire cannonballs, engines propel propellable (Propulsion)
objects, etc. Objects which can have parts assigned (installed) to them are
known as **Constructs.**

The humble _ship_ is the most quintessential example of a construct. Mobile,
propellable, fitted with weapons, item vacuums, and even armor, there are
many things that ships are capable of doing only because of the parts it can
have.

A construct will always have:

  * Children entities to represent its part slots (with the `PartSlot`
    component).

  * A `PartUsageStrategy` component, wrapping a generic logic component which
    defines how these parts are operated, via polling every tick for
    `PartOperationRequest`s (a
    [command pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/command.html))
    for things like engine throttle & steering, weapon firing, etc. Examples:

    * Player ships have the `PlayerOperator` component, which, if the player ID
      matches that of the client's, produce operation requests based on the
      input state as provided by Bevy.

    * NPC ships are controlled by the `ShipNpcOperator`. These in themselves
      actually wrap around a fairly complex NPC AI system, which is still in
      the whiteboard stage, but interacts with an internal model of behavior,
      which is separately implemented. The `ShipNpcOperator` simply provides
      part commands based on the 'state of mind' of the NPC, which itself
      varies with time and might use a task system under the hood.
