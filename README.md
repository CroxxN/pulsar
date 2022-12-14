# A GUI volume controller for individual linux processes using Pulse Audio `written in Rust`

# CURRENTLY NOT FUNCTIONAL

- v.0.1 is yet to be annnounced.

## Progress

- `Project Started (Dec 15 01:29)`

## ToDo

- Migrate to `libpulse-binding`

## Lessons learnt along the way; The path to v.0.1

- The crate `pulsectl-rs` is good for basic volume control but doesn't work for more complex criteria.
   Used it in very early steps of development, and quickly found out it doesn't have as much feature as one would like.
   For example, AFAIK, there is no way to subscribe to events.
   
- Our trusty old friend `libpulse-bindings` does a pretty good job tho.
    I was resitant to use it due to it's scary af funciton parameters, but as I learnt more Rust, I's beginning to make sense.

- That's it.

# Notes for Future Self

- A lot of callbacks are going to be used so it may as well be an exercise in Rust callback functions.
- A lot of entailing madness