use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::mint::{Vector2, Point2};

use crate::assets::Assets;
#[derive(Debug)]
pub enum BackgroundState {
    //There can be multiple backgrounds depending on the stage of the game
    Normal,
    BossLevel,
}
#[derive(Debug)]
pub struct Background {
    pub state: BackgroundState,
    pub pos: Point2<f32>,
}

impl Background {
    pub fn new(pos: Point2<f32>) -> Self {
        Background {
            state: BackgroundState::Normal,
            //state: BackgroundState::BossLevel,
            pos,
        }
    }
    /* 
    //idk what is happening here
    pub fn update(&mut self, amount: f32, seconds: f32, max_right: f32) {
    
        self.pos.x = nalgebra::clamp(self.pos.x, 0.0, max_right);
    }*/
    
    pub fn draw(&self, canvas: &mut graphics::Canvas, assets: &Assets) {  
            //Risuvame background-a spored nivoto na koeto sme
            //println!(self.state);
            match self.state{
                BackgroundState::Normal=>{
                    let draw_params=graphics::DrawParam::default().
                    dest(self.pos);
                    canvas.draw(&assets.background_normal_image, draw_params);
                },
                BackgroundState::BossLevel=>{
                    let draw_params=graphics::DrawParam::default().
                    dest(self.pos).
                    scale(Vector2 { x: 0.60, y: 0.75 });
                    canvas.draw(&assets.background_boss_image, draw_params);
                },
            }
    }
}

#[derive(Debug)]
pub enum PlayerState {
    Normal,
    Shooting,
    Shielded,
}

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
    pub pos: Point2<f32>,
    pub attack_speed:f32,
    pub shield_duration: f32,
    pub shield_cooldown:f32,
    pub is_shield_charged: bool,
    pub is_shield_active: bool,
    pub time_until_next_shot : f32,
    pub time_until_shield_is_charged:f32,
    pub move_speed:f32,
}

impl Player {
    //pub const SHOT_TIMEOUT: f32 = 1.0;
    //pub const SPEED: f32 = 500.0;

    pub fn new(pos: Point2<f32>) -> Self {
        Player {
            state: PlayerState::Normal,
            pos,
            attack_speed:0.5,
            shield_duration:5.0,//adjust as needed
            shield_cooldown:10.0,//adjust as need.
            is_shield_charged:false,//maybe should start with charged shield?
            is_shield_active:false,
            time_until_next_shot :1.0,//adjust as needed
            time_until_shield_is_charged:20.0,//adjust as needed
            move_speed:500.0,//adjust as needed
        }
    }

    pub fn update(&mut self, amount_x: f32, amount_y: f32, seconds: f32, max_right: f32,max_up:f32) {
       // let new_pos_x = self.pos.x + self.move_speed * seconds * amount;//GIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIGA VAJNO
        let new_pos=Point2{
            x:self.pos.x + self.move_speed * seconds * amount_x,
            y:self.pos.y + self.move_speed * seconds * amount_y,
        };
        self.pos.x = nalgebra::clamp(new_pos.x, 0.0, max_right);
        self.pos.y = nalgebra::clamp(new_pos.y, 50.0, max_up);//min is a magic number idk why 0.0 is not working good enough
        
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, assets: &Assets) {
        let draw_params = graphics::DrawParam::default().
                    dest(self.pos).
                    scale(Vector2 { x: 0.26, y: 0.26 }).
                    offset(Point2 { x: 0.5, y: 1.0 });

        match self.state {
            PlayerState::Normal => {

                canvas.draw(&assets.ship_normal_image, draw_params);
            },

            PlayerState::Shooting => {

                    if self.is_shield_active {
                        canvas.draw(&assets.ship_shooting_shilded_image, draw_params);
                    }
                    else{
                        canvas.draw(&assets.ship_shooting_image, draw_params); 
                    }
                
            },
            PlayerState::Shielded =>{
               // let draw_params = graphics::DrawParam::default().
                  //  dest(self.pos);

                canvas.draw(&assets.ship_shielded_image, draw_params);
            }
        }
    }
}

//could just add damage field to the player and combine Player_shot and Enemy_Shot structures
//can the player collide with their own projectile ? Theoretically yes, practically maybe not
#[derive(Debug, Clone)]
pub struct Player_shot {
    pub pos: Point2<f32>,
    pub is_alive: bool,
    velocity: Vector2<f32>,
    pub damage: u8,
}

impl Player_shot {
    pub fn new(pos: Point2<f32>) -> Self {
        Player_shot {
            pos,
            is_alive: true,
            velocity: Vector2 { x: 0.0, y: -750.0 },
            damage:1,
        }
    }

    pub fn update(&mut self, seconds: f32) {
        self.pos.x += self.velocity.x * seconds;
        self.pos.y += self.velocity.y * seconds;
    }

    pub fn draw(&mut self, canvas: &mut graphics::Canvas, assets: &Assets) {
        

        match self.damage {
            1 =>{
                let draw_params = graphics::DrawParam::default().
                    dest(self.pos).
                    scale(Vector2 { x: 0.7, y: 0.7 }).
                    offset(Point2 { x: -0.25, y: -0.25 });
                canvas.draw(&assets.projectile_lvl_1_image, draw_params);
            },
            2=>{
                let draw_params = graphics::DrawParam::default().
                    dest(self.pos).
                    scale(Vector2 { x: 0.3, y: 0.3 }).
                    offset(Point2 { x: -0.95, y: -0.35 });
                canvas.draw(&assets.projectile_lvl_2_image,draw_params);
            }
            3=>{
                let draw_params = graphics::DrawParam::default().
                    dest(self.pos).
                    scale(Vector2 { x: 0.3, y: 0.3 }).
                    offset(Point2 { x: -1.00, y: -0.35 });
                canvas.draw(&assets.projectile_lvl_3_image, draw_params);
            }
            4=>
            {
                let draw_params = graphics::DrawParam::default().
                    dest(self.pos).
                    scale(Vector2 { x: 0.3, y: 0.3 }).
                    offset(Point2 { x: -1.05, y: -0.50 });
                canvas.draw(&assets.projectile_lvl_4_image,draw_params);// graphics::DrawParam::default().dest(self.pos));
            }
            _=>(),
            
        }
        //canvas.draw(&assets.shot_image, graphics::DrawParam::default().dest(self.pos));
    }
}
pub struct Enemy_shot {
    pub pos: Point2<f32>,
    pub is_alive: bool,
    velocity: Vector2<f32>,
   // pub damage: u8,
}

pub enum EnemyType {
    WAVE1,
    WAVE2,
    WAVE3,
    WAVE4,
    BOSS_WAVE,
}
pub struct Enemy {
    pub pos: Point2<f32>,
    pub is_alive: bool,
    pub enemy_type:EnemyType,
    pub current_health: u8,
    velocity: Vector2<f32>,
}

impl Enemy{
    pub fn new(pos: Point2<f32>,enemy_type:EnemyType,current_health:u8) -> Self {
        Enemy {
            pos,
            is_alive:true,
            enemy_type,
            current_health,
            velocity: Vector2 { x: 0.0, y: 200.0 },
        }
    }
    pub fn update(&mut self, amount: f32, seconds: f32, max_down: f32) {
        
        self.pos.x += self.velocity.x * seconds;
        self.pos.y += self.velocity.y * seconds;
        //self.pos.y += self.velocity.y * seconds;
        //let new_pos = self.pos.y + Self::SPEED * seconds * amount;
       // self.pos.x = nalgebra::clamp(new_pos, 0.0, max_down);
    }


    pub fn draw(&mut self, canvas: &mut graphics::Canvas, assets: &Assets){

    let draw_params = graphics::DrawParam::default().dest(self.pos).scale(Vector2 { x: 0.3, y: 0.3 });
            match self.enemy_type{
                EnemyType::WAVE1=>{
                    canvas.draw(&assets.enemy_1_image, draw_params);
                },
                EnemyType::WAVE2=>{
                    canvas.draw(&assets.enemy_2_image, draw_params);
                },
                EnemyType::WAVE3=>{
                    canvas.draw(&assets.enemy_3_image, draw_params);
                },
                EnemyType::WAVE4=>{
                    canvas.draw(&assets.enemy_4_image, draw_params);
                }
                EnemyType::BOSS_WAVE=>{
                    canvas.draw(&assets.boss_image, draw_params);
                }
            }
    }
}