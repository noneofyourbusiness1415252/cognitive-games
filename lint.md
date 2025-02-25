warning: method `angle` is never used
  --> src/games/mental_rotation/tile.rs:50:12
   |
23 | impl Direction {
   | -------------- method in this implementation
...
50 |     pub fn angle(self) -> f64 {
   |            ^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: function `random_rotation` is never used
 --> src/games/mental_rotation/level_generator.rs:4:4
  |
4 | fn random_rotation() -> i32 {
  |    ^^^^^^^^^^^^^^^

warning: function `random_bool` is never used
 --> src/games/mental_rotation/level_generator.rs:8:4
  |
8 | fn random_bool() -> bool {
  |    ^^^^^^^^^^^

warning: struct `GridPos` is never constructed
 --> src/games/mental_rotation/grid.rs:2:12
  |
2 | pub struct GridPos {
  |            ^^^^^^^
  |
  = note: `GridPos` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: associated items `new` and `to_pair` are never used
  --> src/games/mental_rotation/grid.rs:8:12
   |
7  | impl GridPos {
   | ------------ associated items in this implementation
8  |     pub fn new(x: usize, y: usize) -> Self {
   |            ^^^
...
12 |     pub fn to_pair(&self) -> (usize, usize) {
   |            ^^^^^^^

warning: function `rotate_coordinates` is never used
 --> src/games/mental_rotation/rotation.rs:1:8
  |
1 | pub fn rotate_coordinates(cells: &[(usize, usize)], rotation: i32) -> Vec<(usize, usize)> {
  |        ^^^^^^^^^^^^^^^^^^

warning: lint group `pedantic` has the same priority (0) as a lint
  --> Cargo.toml:65:1
   |
65 | pedantic = "warn"
   | ^^^^^^^^   ------ has an implicit priority of 0
...
77 | unsafe_derive_deserialize = "allow"
   | ------------------------- has the same priority as this lint
   |
   = note: the order of the lints in the table is ignored by Cargo
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#lint_groups_priority
   = note: `#[warn(clippy::lint_groups_priority)]` on by default
help: to have lints override the group set `pedantic` to a lower priority
   |
65 - pedantic = "warn"
65 + pedantic = { level = "warn", priority = -1 }
   |

warning: this argument is passed by value, but not consumed in the function body
 --> src/games/perception/input.rs:7:51
  |
7 |     pub(super) fn setup_click_handler(game_state: Rc<RefCell<Self>>) -> Result<(), JsValue> {
  |                                                   ^^^^^^^^^^^^^^^^^ help: consider taking a reference instead: `&Rc<RefCell<Self>>`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
  = note: `-W clippy::needless-pass-by-value` implied by `-W clippy::pedantic`
  = help: to override `-W clippy::pedantic` add `#[allow(clippy::needless_pass_by_value)]`

warning: this function has too many lines (121/100)
   --> src/games/perception/maze.rs:8:5
    |
8   | /     pub(super) fn create_maze(size: usize, document: Document) -> Self {
9   | |         // Total cells and walls per cell (top, right, bottom, left)
10  | |         let total_cells = size * size;
11  | |         let wall_per_cell = 4;
...   |
165 | |     }
    | |_____^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_lines
    = note: `-W clippy::too-many-lines` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::too_many_lines)]`

warning: this function's return value is unnecessary
  --> src/games/perception/timer.rs:7:5
   |
7  | /     pub(super) fn setup_timer(game_state: &Rc<RefCell<Perception>>) -> Result<(), JsValue> {
8  | |         let timer_callback = {
9  | |             let game_state = game_state.clone();
10 | |             Closure::wrap(Box::new(move || {
...  |
35 | |         Ok(())
36 | |     }
   | |_____^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
   = note: `-W clippy::unnecessary-wraps` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::unnecessary_wraps)]`
help: remove the return type...
   |
7  -     pub(super) fn setup_timer(game_state: &Rc<RefCell<Perception>>) -> Result<(), JsValue> {
7  +     pub(super) fn setup_timer(game_state: &Rc<RefCell<Perception>>) -> () {
   |
help: ...and then remove returned values
   |
35 -         Ok(())
   |

warning: you are deriving `serde::Deserialize` on a type that has methods using `unsafe`
  --> src/games/perception/mod.rs:21:28
   |
21 | #[derive(Clone, Serialize, Deserialize)]
   |                            ^^^^^^^^^^^
   |
   = help: consider implementing `serde::Deserialize` manually. See https://serde.rs/impl-deserialize.html
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unsafe_derive_deserialize
   = note: `-W clippy::unsafe-derive-deserialize` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::unsafe_derive_deserialize)]`
   = note: this warning originates in the derive macro `Deserialize` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: the loop variable `x` is used to index `occupied`
  --> src/games/mental_rotation/level_generator.rs:42:14
   |
42 |     for x in 1..size-1 {
   |              ^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
   = note: `#[warn(clippy::needless_range_loop)]` on by default
help: consider using an iterator and enumerate()
   |
42 -     for x in 1..size-1 {
42 +     for (x, <item>) in occupied.iter_mut().enumerate().take(size-1).skip(1) {
   |

warning: methods with the following characteristics: (`to_*` and `self` type is `Copy`) usually take `self` by value
  --> src/games/mental_rotation/grid.rs:12:20
   |
12 |     pub fn to_pair(&self) -> (usize, usize) {
   |                    ^^^^^
   |
   = help: consider choosing a less ambiguous name
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#wrong_self_convention
   = note: `#[warn(clippy::wrong_self_convention)]` on by default

warning: this macro has been superceded by `std::sync::LazyLock`
  --> src/games/mental_rotation/timer.rs:8:1
   |
8  | / lazy_static! {
9  | |     static ref CURRENT_TIMER: Mutex<Option<i32>> = Mutex::new(None);
10 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#non_std_lazy_statics
   = note: `-W clippy::non-std-lazy-statics` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::non_std_lazy_statics)]`

warning: this macro has been superceded by `std::sync::LazyLock`
  --> src/games/mental_rotation/mod.rs:16:1
   |
16 | / lazy_static! {
17 | |     static ref GAME_INSTANCE: Mutex<Option<MentalRotation>> = Mutex::new(None);
18 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#non_std_lazy_statics

warning: you are deriving `serde::Deserialize` on a type that has methods using `unsafe`
  --> src/games/mental_rotation/mod.rs:21:28
   |
21 | #[derive(Clone, Serialize, Deserialize)]
   |                            ^^^^^^^^^^^
   |
   = help: consider implementing `serde::Deserialize` manually. See https://serde.rs/impl-deserialize.html
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unsafe_derive_deserialize
   = note: this warning originates in the derive macro `Deserialize` (in Nightly builds, run with -Z macro-backtrace for more info)

