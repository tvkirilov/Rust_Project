//use ggez::conf::WindowMode;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::conf::{Conf, WindowMode};
use ggez::mint::Point2;
use std::path;
use std::env;
use chicken_invaders::assets::Assets;
use chicken_invaders::entities::{Background,BackgroundState,PlayerState,Player, Player_shot, Enemy_shot,Enemy,EnemyType};
use rand::Rng;
use rand::rngs::ThreadRng;
use ggez::audio::SoundSource;
use ggez::input::keyboard;
#[derive(Debug, Default)]
struct InputState {
    movement_x: f32,
    movement_y: f32,
    fire: bool,
    shielding: bool,
}

struct MainState {
    // Your state here...
    rng:ThreadRng,
    game_over: bool,
    bg:Background,
    assets: Assets,
    score: u32,
    input :InputState,
    player:Player,
    player_shots:Vec<Player_shot>,
    enemy:Enemy,//just for testing something remove later
    enemies: Vec<Enemy>,
    enemy_shots:Vec<Enemy_shot>,
    time_until_next_wave: f32,
    current_wave:u8,
    screen_width: f32,
    screen_height: f32,

}

impl MainState {
    pub fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        // Load/create resources such as images here.
        let assets = Assets::new(ctx)?;
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;

        //setting the background
        let background_pos = Point2 {
            x:0.0, 
            y:0.0, 
        };

        let current_background=Background{
             state:BackgroundState::Normal,
            //state:BackgroundState::BossLevel,
             pos:background_pos,

        };

        //setting the starting position of the player
        let player_start_pos=Point2{
            x:screen_width / 2.0,
            y:screen_height,
        };

         //setting the starting position of one enemy
        let enemy_start_pos=Point2{
            x:500.0,
            y:500.0,
        };
            // ...
        let s = MainState {
            rng: rand::thread_rng(),
            game_over: false,
            bg:current_background,
            assets: assets,
            score: 0,
            input: InputState::default(),
            player: Player::new(player_start_pos),
            player_shots: Vec::new(),
            enemies:Vec::new(),
            enemy:Enemy::new(enemy_start_pos, EnemyType::WAVE1, 1),//just for testing something remove later
            enemy_shots: Vec::new(),
            time_until_next_wave:15.0,//probbably will need adjustment
            current_wave:1,
            screen_width: conf.window_mode.width,
            screen_height: conf.window_mode.height,
        };

        
        Ok((s))
    }
/* 
    fn handle_collisions(&mut self, ctx: &mut Context) {
        for enemy in &mut self.enemies {
            for shot in &mut self.shots {
                if enemy.bounding_rect(ctx).contains(shot.pos) {
                    shot.is_alive = false;
                    enemy.is_alive = false;
                    self.score += 1;
                    let _ = self.assets.boom_sound.play(ctx);
                }
            }
        }
    }*/
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.game_over {
            return Ok(());
        }

        const DESIRED_FPS: u32 = 60;

        while ctx.time.check_update_time(DESIRED_FPS) {
            let seconds = ctx.time.delta().as_secs_f32();

            //for the enemeies
            let enemy_start_pos=Point2{
                x:500.0 ,
                y:500.0,
            };

            let enemy = Enemy::new(enemy_start_pos, EnemyType::WAVE1, 1);
            //let mut enemy_vec:Vec<Enemy>=Vec::new();
            //enemy_vec.push(enemy);
            self.enemies.push(enemy);


            //for the player=========================
            self.player.update(self.input.movement_x,self.input.movement_y, seconds, self.screen_width,self.screen_height);

            if self.player.time_until_next_shot>0.0
            {
                self.player.time_until_next_shot -= seconds;
            }
            //self.player.time_until_next_shot -= seconds;

            //ne mi haresva neshto tuka
            /* 
            self.player.shield_duration-=seconds;
            self.player.shield_cooldown=self.player.shield_cooldown+self.player.shield_duration-seconds;
*/
            //for the player shots
            if self.input.fire && self.player.time_until_next_shot < 0.0 {
                let shot_pos = Point2 {
                    x: self.player.pos.x - 62.0,
                    y: self.player.pos.y - 200.0,
                   //x:500.0,
                  // y:500.0,
                };
                let shot = Player_shot::new(shot_pos);
                self.player_shots.push(shot);

                let _ = self.assets.shot_sound.play(ctx);

                self.player.time_until_next_shot = self.player.attack_speed;
                self.player.state = PlayerState::Shooting;

            } else if !self.input.fire/*||self.player.time_until_next_shot>0.25 */{
                self.player.state = PlayerState::Normal;
            }

            if self.player.is_shield_active{
                self.player.state = PlayerState::Shielded;
            }

            for shot in self.player_shots.iter_mut() {
                shot.update(seconds);
            }

           // self.handle_collisions(ctx);
            self.player_shots.retain(|shot| shot.is_alive && shot.pos.y >= 0.0);
            //self.enemies.retain(|enemy| enemy.is_alive);

            }

        Ok(())
    }
 
    fn key_down_event(&mut self, ctx: &mut Context, input: keyboard::KeyInput, _repeat: bool) -> GameResult<()> {
        match input.keycode {
            Some(keyboard::KeyCode::Z) => self.player.is_shield_active = true,
            Some(keyboard::KeyCode::Space) => self.input.fire = true,
            Some(keyboard::KeyCode::Left) => self.input.movement_x = -1.0,
            Some(keyboard::KeyCode::Right) => self.input.movement_x = 1.0,
            Some(keyboard::KeyCode::Up) => self.input.movement_y = -1.0,
            Some(keyboard::KeyCode::Down) => self.input.movement_y = 1.0,
            Some(keyboard::KeyCode::Escape) => ctx.request_quit(),
            _ => (), // Do nothing
        }

        Ok(())
    }
     
    fn key_up_event(&mut self, _ctx: &mut Context, input: keyboard::KeyInput) -> GameResult<()> {
        match input.keycode {
            Some(keyboard::KeyCode::Z) => self.player.is_shield_active = false,
            Some(keyboard::KeyCode::Space) => self.input.fire = false,
            Some(keyboard::KeyCode::Left | keyboard::KeyCode::Right) => {
                self.input.movement_x = 0.0
            },
            Some(keyboard::KeyCode::Up | keyboard::KeyCode::Down)=>{
            self.input.movement_y=0.0
            },
            _ => (), // Do nothing
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let black = graphics::Color::from_rgb(0, 0, 0);
        let mut canvas = graphics::Canvas::from_frame(ctx, black);

        //drawing the background
        self.bg.draw(&mut canvas, &self.assets);

        //drawing the player
        self.player.draw(&mut canvas, &self.assets);

        //drawing the shots
        for shot in self.player_shots.iter_mut() {
            shot.draw(&mut canvas, &self.assets);
        }


        //self.enemy.draw(&mut canvas,&self.assets);
        //self.enemies.draw


        //drawing the enemies
        //self.enemies[0].draw(&mut canvas,&self.assets);
        /* 
        let mut i=0;
        while i<self.enemies.len()
        {
            self.enemies[i].draw(&mut canvas,&self.assets);
            i+=1;
        }*/
           
       for enemy in self.enemies.iter_mut() {
            enemy.draw(&mut canvas,&self.assets);
        }

        canvas.finish(ctx)?;
        Ok(())
    }

}


fn main() {

    // Make a Context.
    let conf = Conf::new().
    window_mode(WindowMode {
        width: 1750.0,
        height: 950.0,
        ..Default::default()
    });

    let (mut ctx, event_loop) = ContextBuilder::new("Chicken Invaders", "Teodor").
        default_conf(conf.clone()).
        build().
        unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.fs.mount(&path, true);
    }

    let my_game = MainState::new(&mut ctx, &conf).unwrap();

    //setting the window name
    ctx.gfx.set_window_title("Chicken Invaders");

    // Run!
    event::run(ctx, event_loop, my_game);
    
}