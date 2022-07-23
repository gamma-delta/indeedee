# Indeedee

Utilities for making progress bars and distributing work across frames. The target use case is for things
like world generation or loading assets.

The core of this crate is the `ProgressiveWaiter` struct, which takes an iterator and a state running over it.
Then, you query the waiter with `query`, passing a maximum amount of time you allow the iterator to run.
The `ProgressiveWaiter` will consume as many elements out of the iterator as it can into the state, and
return whether it's done or not.

---

This crate is built alongside and for my W.I.P. roguelike [Foxfire](https://www.petra-k.at/foxfire).
