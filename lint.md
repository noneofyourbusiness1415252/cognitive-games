error[E0599]: no method named `unwrap` found for unit type `()` in the current scope
   --> src/games/numeracy/mod.rs:145:33
    |
145 |             this.update_timer().unwrap();
    |                                 ^^^^^^ method not found in `()`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/games/numeracy/mod.rs:173:9
    |
173 |         self.update_stats()?;
    |         ^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `()`
    |
    = help: the trait `std::ops::Try` is not implemented for `()`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/games/numeracy/mod.rs:182:9
    |
182 |         self.update_stats()?;
    |         ^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `()`
    |
    = help: the trait `std::ops::Try` is not implemented for `()`

error[E0308]: mismatched types
  --> src/games/perception/mod.rs:96:27
   |
96 |         Self::setup_timer(game_state.clone())?;
   |         ----------------- ^^^^^^^^^^^^^^^^^^ expected `&Rc<RefCell<Perception>>`, found `Rc<RefCell<Perception>>`
   |         |
   |         arguments to this function are incorrect
   |
   = note: expected reference `&std::rc::Rc<_>`
                 found struct `std::rc::Rc<_>`
note: associated function defined here
  --> src/games/perception/timer.rs:7:19
   |
7  |     pub(super) fn setup_timer(game_state: &Rc<RefCell<Perception>>) -> Result<(), JsValue> {
   |                   ^^^^^^^^^^^ ------------------------------------
help: consider borrowing here
   |
96 |         Self::setup_timer(&game_state.clone())?;
   |                           +

error[E0599]: no method named `get_arrow_classes` found for reference `&games::mental_rotation::MentalRotation` in the current scope
   --> src/games/mental_rotation/mod.rs:214:52
    |
214 |                         arrow.set_class_name(&self.get_arrow_classes(tile));
    |                                               -----^^^^^^^^^^^^^^^^^------
    |                                               |    |
    |                                               |    this is an associated function, not a method
    |                                               help: use associated function syntax instead: `games::mental_rotation::MentalRotation::get_arrow_classes(tile)`
    |
    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: the candidate is defined in an impl for the type `games::mental_rotation::MentalRotation`
   --> src/games/mental_rotation/mod.rs:51:5
    |
51  |     fn get_arrow_classes(tile: &tile::Tile) -> String {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `get_arrow_classes` found for struct `games::mental_rotation::MentalRotation` in the current scope
   --> src/games/mental_rotation/mod.rs:285:68
    |
22  | pub struct MentalRotation {
    | ------------------------- method `get_arrow_classes` not found for this struct
...
285 |                                         arrow.set_class_name(&game.get_arrow_classes(tile));
    |                                                               -----^^^^^^^^^^^^^^^^^------
    |                                                               |    |
    |                                                               |    this is an associated function, not a method
    |                                                               help: use associated function syntax instead: `games::mental_rotation::MentalRotation::get_arrow_classes(tile)`
    |
    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: the candidate is defined in an impl for the type `games::mental_rotation::MentalRotation`
   --> src/games/mental_rotation/mod.rs:51:5
    |
51  |     fn get_arrow_classes(tile: &tile::Tile) -> String {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Some errors have detailed explanations: E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `cognitive-games` (lib) due to 6 previous errors
error: could not compile `cognitive-games` (lib test) due to 6 previous errors
