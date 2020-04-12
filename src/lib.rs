extern crate wasm_bindgen;
use serde_wasm_bindgen::*;
#[macro_use]
extern crate serde_derive;
use wasm_bindgen::prelude::*;
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[derive(Serialize, Deserialize)]
enum Tile {
    Wall,
    Floor,
}
#[derive(Serialize, Deserialize)]
pub struct Vector2 {
    x: i32,
    y: i32,
}
impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
}
impl std::ops::Add for Vector2{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign for Vector2{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl Tile {
    fn get_color(&self) -> u32 {
        match self {
            Self::Floor => 0x191919,
            Self::Wall => 0x033499,
        }
    }
}
#[derive(Serialize, Deserialize)]
struct Grid {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl Grid {
    pub fn new(width: u32, height: u32, tiles: Vec<Tile>) -> Grid {
        Grid {
            tiles: tiles,
            width: width,
            height: height,
        }
    }
    pub fn draw(&self) -> Vec<u32> {
        let mut out: Vec<u32> = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                out.append(&mut vec![
                    self.tiles[(x * self.width + y) as usize].get_color(),
                    x * 10,
                    y * 10,
                    10,
                    10,
                ])
            }
        }
        return out;
    }
}
#[derive(Serialize, Deserialize)]
struct Player {
    pos:Vector2,
    health: u32,
}
impl Player{
    pub fn draw(&self)->Vec<u32>{
        vec![0x00ff00,(self.pos.x*10) as u32 ,(self.pos.y*10) as u32,10,10]
    }
}
#[derive(Serialize, Deserialize)]
struct Enemy {
    pos:Vector2,
    health: u32,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entities:Vec<Entity>,

    grid: Grid,
}
#[derive(Serialize)]
pub struct MainOutput {
    pub state: State,
    pub draw_calls: Vec<u32>,
}
#[derive(Serialize, Deserialize)]
struct Entity{
    position:Vector2
}
trait Component{
    fn apply(entity: &mut std::rc::Rc<std::cell::RefCell<Entity>>,world: &mut Grid,input:Vector2);
}
struct InputComponent{

}
impl Component for InputComponent{
    fn apply(entity: &mut std::rc::Rc<std::cell::RefCell<Entity>>,world: &mut Grid,input:Vector2){
        entity.borrow_mut().position+=input;
    }
}
#[wasm_bindgen]
pub fn game_loop_js(input: JsValue, state_in: JsValue) -> JsValue {
    return serde_wasm_bindgen::to_value(&game_loop(
        serde_wasm_bindgen::from_value(input).ok().unwrap(),
        serde_wasm_bindgen::from_value(state_in).ok().unwrap(),
    ))
    .ok()
    .unwrap();
}

pub fn game_loop(input:Vector2,state: State) -> MainOutput {
    let mut state_m = state;
    let mut input_m = input;
    input_m.y*=-1;
    state_m.player.pos+=input_m;
    let mut draw_calls = state_m.grid.draw();
    draw_calls.append(&mut state_m.player.draw());

    MainOutput {
        draw_calls: draw_calls,
        state: state_m,
    }
}

pub fn init_state() -> State {
    State {
        entities:vec![],
        grid: Grid::new(
            5,
            5,
            vec![
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Floor,
                Tile::Floor,
                Tile::Floor,
                Tile::Wall,
                Tile::Wall,
                Tile::Floor,
                Tile::Floor,
                Tile::Floor,
                Tile::Wall,
                Tile::Wall,
                Tile::Floor,
                Tile::Floor,
                Tile::Floor,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
            ],
        ),
    }
}
#[wasm_bindgen]
pub fn init_state_js() -> JsValue {
    serde_wasm_bindgen::to_value(&init_state()).ok().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_grid() {
        let v: Vec<Tile> = vec![];
        let g = Grid::new(0, 0, v);
        assert!(g.draw().len() == 0)
    }
    #[test]
    fn one_by_one_grid() {
        let g = Grid::new(1, 1, vec![Tile::Wall]);
        assert!(g.draw() == vec![Tile::Wall.get_color(), 0, 0, 10, 10])
    }
    #[test]
    fn run_frame() {
        let s = init_state();
        game_loop(Vector2::new(0, 0),s);
    }
    #[test]
    fn add_vec(){
        let v1 = Vector2::new(0, 0)+Vector2::new(0, 0);
        assert_eq!(v1.x,0);
        assert_eq!(v1.y,0);
        let v2 = Vector2::new(1, 1)+Vector2::new(1, 1);
        assert_eq!(v2.x,2);
        assert_eq!(v2.y,2);
    }
    #[test]
    fn add_vec_assign(){
        let mut v1 = Vector2::new(0, 0);
        v1+=Vector2::new(1, 1);
        assert_eq!(v1.x,1);
        assert_eq!(v1.y,1);

    }


}
