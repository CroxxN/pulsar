# A GUI volume controller for individual linux processes using Pulse Audio `written in Rust`

# CURRENTLY NOT FUNCTIONAL

- `v.0.1` is yet to be annnounced.

## Progress

- `Project Started (Dec 15 01:29)`
- `Migrated to libpulse-binding (Dec 17 01:53)`
- `Everything *just works* (Dec 17 01:53)`
- `Pulse is ready to be integrated with GUI  (Dec 23 17:43)`
- `A skeleton for GUI has been created (Dec 24 22:44)`
- `Separate threads for the GUI and Pulse client (may be scrapped)`

## ToDo

- ~~Migrate to `libpulse-binding`~~ (done)

- The code is a mess, organize it.

- ~~Use egui for the GUI. Was going to use druid for it but had a change of heart lmao.~~ (Done)(Scrapped)

- On the GUI side, remember what the window size and position the user has set and load from it next time

- Druid looks a lot ergonomic and functions exactly the way I need. So, friendship ended with egui, Druid is my mate now.


## Lessons learnt along the way; The path to `v.0.1`

- The crate `pulsectl-rs` is good for basic volume control but doesn't work for more complex criterion.
   Used it in very early steps of development, and quickly found out it doesn't have as much feature as one would like.
   For example, AFAIK, there is no way to subscribe to events.
   
- Our trusty old friend `libpulse-binding` does a pretty good job tho.
    I was resitant to use it due to it's scary af funciton parameters, but as I learnt more Rust, It's beginning to make sense.
    
- Projects are a nightmare. 0/10 wouldn't recomend

- Basic GUI

- Going insane now

- Designing before hand is a skill

- That's it.

# Notes for Future Self

- A lot of callbacks are going to be used so it may as well be an exercise in Rust callback functions.
- A lot of entailing madness (no cap `Dec 23`)
- I'm going to need to learn more about Rust's `unsafe` blocks.
- Insanity