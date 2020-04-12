extern crate wasm_bindgen;
#[allow(unused_imports)]
use serde_wasm_bindgen::*;
#[macro_use]
extern crate serde_derive;
use wasm_bindgen::prelude::*;
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
static TILE_SIZE:u32=20;
#[derive(Serialize, Deserialize,Clone,std::cmp::PartialEq,Debug)]
enum Tile {
    Wall,
    Floor,
}
#[derive(Serialize, Deserialize,Clone,std::cmp::PartialEq,Debug)]
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
#[derive(Serialize, Deserialize,Clone)]
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
    pub fn get_tile(&self, position:Vector2)->Option<Tile>{
        let index = (position.x as u32 * self.width + position.y as u32) as usize;
        if index<self.tiles.len(){
            return Some(self.tiles[index].clone());
        }
        None
    }
    pub fn draw(&self) -> Vec<u32> {
        let mut out: Vec<u32> = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                out.append(&mut vec![
                    self.tiles[(x * self.width + y) as usize].get_color(),
                    x * TILE_SIZE,
                    y * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                ])
            }
        }
        return out;
    }
}
#[derive(Serialize, Deserialize)]
pub struct State {
    entities:Vec<Entity>,
    grid: Grid,
    input:InputComponent,
    grid_component:GridComponent,
}
impl State{
    pub fn process(&mut self, input:Vector2){
        let mut entities_v = vec![];
        for ent in self.entities.iter(){
            let test = self.input.apply(ent.clone(),self.grid.clone(),input.clone());
            entities_v.push(test.0);
            self.grid=test.1;
        }
        self.entities=entities_v;
        entities_v = vec![];
        for ent in self.entities.iter(){
            let test = self.grid_component.apply(ent.clone(),self.grid.clone(),input.clone());
            entities_v.push(test.0);
            self.grid=test.1;
        }
        self.entities=entities_v;
    }
    #[allow(dead_code)]
    fn get_tile(&self,position:Vector2)->Option<Tile>{
        return self.grid.get_tile(position)
    }
    pub fn draw(&self)->Vec<u32>{
        let mut draws = self.grid.draw();
        for ent in self.entities.iter(){
            draws.append(&mut vec![0x00ff00,(ent.position.x as u32 * TILE_SIZE) as u32,(ent.position.y as u32 * TILE_SIZE) as u32,TILE_SIZE,TILE_SIZE]);
        }
        return draws;
    }
    #[allow(dead_code)]
    fn get_entities(&self)->&Vec<Entity>{
        &self.entities
    }
}
#[derive(Serialize)]
pub struct MainOutput {
    pub state: State,
    pub draw_calls: Vec<u32>,
}
#[derive(Serialize, Deserialize,Clone)]
struct Entity{
    position:Vector2,
    delta_position:Vector2,
}
impl Entity{
    pub fn new(pos:Vector2)->Entity{
        Entity{
            position:pos,
            delta_position:Vector2::new(0,0),
        }
    }
}
trait Component{
    fn apply(&self,entity: Entity,world: Grid,input:Vector2)->(Entity,Grid);
}

#[derive(Serialize, Deserialize)]
struct InputComponent{

}
impl Component for InputComponent{
    fn apply(&self,entity: Entity,world: Grid,input:Vector2)->(Entity,Grid){
        let mut entity_n = entity;
        entity_n.delta_position=input;
        return (entity_n,world);
    }
}
#[derive(Serialize, Deserialize)]
struct GridComponent{

}
impl Component for GridComponent{
    fn apply(&self,entity: Entity,world: Grid,_input:Vector2)->(Entity,Grid){
        if let Some(tile) = world.get_tile(entity.position.clone()+entity.delta_position.clone()){
            let mut entity_m = entity;
            if tile!=Tile::Wall{
                entity_m.position+=entity_m.delta_position;
            }
            entity_m.delta_position=Vector2::new(0, 0);
            return (entity_m,world)
        }
        let mut entity_m = entity;
        entity_m.delta_position=Vector2::new(0, 0);
        return (entity_m,world);
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
    state_m.process(input);

    MainOutput {
        draw_calls: state_m.draw(),
        state: state_m,
    }
}

pub fn init_state() -> State {
    State {
        entities:vec![Entity::new(Vector2::new(1, 1))],
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
        input:InputComponent{},
        grid_component:GridComponent{}
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
    #[test]
    fn run_frame_input(){
        let mut s = init_state();
        s = game_loop(Vector2::new(1, 0),s).state;
        assert_eq!(s.get_tile(Vector2::new(2,1)),Some(Tile::Floor));
        let e = &s.get_entities()[0];
        assert_eq!(e.position,Vector2::new(2, 1));
    }


}
