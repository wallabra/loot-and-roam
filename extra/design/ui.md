[//]: # ( (c) 2025 GameCircular. )
[//]: # ( Written by: )
[//]: # ( * Gustavo Ramos Rehermann (rehermann6046@gmail.com) )

[//]: # ( For license details, please see the bottom of the file. )


# UI Engine

No UI library in Rust meets our goals of being simple and immediate mode,
while still providing a multi-pass layout engine, and being overlayable over
an existing game. For that reason, we will write our own and eventually provide
it as a separate crate. We are inspired by [Clay](https://github.com/nicbarker/clay).

We have considered the following solutions, rejected for the attributed reasons:

* **egui** - has a layout engine (morphorn), but it is clunky and hard to style
* **vello** - is retained-mode, rather than immediate-mode
* **clay-rs** - requires C bindings

Retained-mode UIs are naturally harder to decouple from the broader codebase, and
end up actually being more effort for little gain.

The prototype also has its own UI engine, which is a bit limited (and messy),
retained mode, and object oriented.

## Principles

These are the goals our UI engine seeks to achieve:

* **Immediate-mode**, to allow decuopled integration
* **Multi-pass**, to allow flexible layouts, and adjust around constraints
* **Event based**, following the principles of the Elm Architecture
* **Data driven**, for performance (We are immediate mode, we need to squeeze every drop of performance we can get!)
* **Asynchronous**

### Multi-pass

Our UI engine will have multiple phases:

* **Generation** - the instructions for populating the UI are generated from the current client state
* **Layouting** - the constraints around every element are resolved with a layout engine
* **Rendering** - a list of primitive instructions are generated, to be consumed by a backend (a la Clay)

### Data driven UI

UIs are naturally hierarchical, but the way this is done is traditionally with object oriented pointers. This hinders
our goals of being data-driven.

The API will, on every render, instead flatten a tree of closures (egui-style) into a list of instructions,
so-called *high level instructions*, which perform incremental operations on UI elements in a stack.
When an element is ready, it is popped from the stack, pushed to a pool (based on slotmaps), and its key assigned to
the list of children in its parent element, which is now atop the stack.

A stack of style builders is also kept alongside the element stack; when a new element is initialized, the style
atop the stack is cloned back onto the stack, to maintain inheritance; before an element is popped, the style is
popped and assigned to it.

The pool of elements, each of which keeps a list of children as indexes into the pool itself, is then passed to the
layout engine, which works from the root elememnt to solve constraints and define the rectangular boundaries for every
elemenet. In the process, it generates the *low level instructions*, which are the sequential rendering instructions
that must be implemented by the backend.

### Event based

Our UI engine seeks to implement the Elm Architecture. That is, user code wil define UI using two processes:
**display logic**, which uses the UI engine API to build the UI for the current state, and **update logic**, which
updates the current state on UI events.

The UI is only rendered when the state is updated. For this reason, the rendering output must be cached, likely in
an internal buffer. and that must be drawn by Bevy on each frame.

### Asynchronous

Using an asynchronous runtime such as Tokio, UI updates need to be resolved concurrently with the Bevy tick loop.

For this reason, both the UI engine and Bevy must be run asynchronously. This must be possible with Tokio's
work-stealing (`rt-multi-thread`) runtime.

[//]: # (  (c)2025 GameCircular. )
[//]: # (  All Loot & Roam documentation and support material is marked with CC0 1.0 Universal: https://creativecommons.org/publicdomain/zero/1.0/ )
