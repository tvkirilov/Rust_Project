//use ggez::conf::WindowMode;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Drawable};
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
use chicken_invaders::debug;

const MAX_ENEMIES_PER_WAVE:usize=10;
const SCORE_NEED_FOR_NEXT_UPGRADE:u32=10;
const NEXT_WAVE_IN:f32=1.0;
const MAX_ENEMY_SHOTS_AT_A_TIME:usize=20;

const FIRST_ROW_START:Point2<f32>=Point2{
    x:20.0,
    y:20.0,
};

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
    time_until_next_enemy_shot:f32,
    current_wave:i32,
    is_wave_generated:bool,
    screen_width: f32,
    screen_height: f32,
    in_debugmode:bool,
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
            time_until_next_enemy_shot:1.0,
            is_wave_generated:false,
            screen_width: conf.window_mode.width,
            screen_height: conf.window_mode.height,
            in_debugmode:false,
        };

        
        Ok((s))
    }

    fn handle_collisions(&mut self, ctx: &mut Context) {

        for enemy_shot in &mut self.enemy_shots{
            if enemy_shot.bounding_rect(ctx,&self.assets).overlaps(&self.player.bounding_rect(ctx,&self.assets))
                &&!self.player.is_shield_active
            {
                self.game_over=true;
                let _ = self.assets.boom_sound.play(ctx);
                break;//not sure if its needed
            }
        }


        for enemy in &mut self.enemies {
            if enemy.bounding_rect(ctx,&self.assets).overlaps(&self.player.bounding_rect(ctx,&self.assets))
            &&!self.player.is_shield_active
            {
                self.game_over=true;
                let _ = self.assets.boom_sound.play(ctx);
                break;//not sure if its needed
            }
            for shot in &mut self.player_shots {
                if enemy.bounding_rect(ctx,&self.assets).overlaps(&shot.bounding_rect(ctx,&self.assets)) {
                    shot.is_alive = false;
                    enemy.current_health-= shot.damage as i32;
                    if enemy.current_health<=0
                    {
                        enemy.is_alive = false;
                        self.score += 1;
                    }
                    let _ = self.assets.boom_sound.play(ctx);
                }
            }
            
        }
    }

}

impl event::EventHandler for MainState {

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.game_over {
            return Ok(());
        }
        
        const DESIRED_FPS: u32 = 60;


        while ctx.time.check_update_time(DESIRED_FPS) {
            let seconds = ctx.time.delta().as_secs_f32();

            //managing the shield power
            /* 
            if !self.player.is_shield_active
            {
                self.player.shield_power+=seconds;
            }*/

            //for the enemeies
            let enemy_start_pos=Point2{
                x:20.0 ,
                y:20.0,
            };

             
            //povtarqne na kod :( trbva da se izvadi v otdelna funkciq
            match self.current_wave{
                1=>{
                    //wave has 1 health
                    while self.enemies.len() <MAX_ENEMIES_PER_WAVE && !self.is_wave_generated
                    {
                        let index=self.enemies.len();//da ne vikame len poveche puti

                        if !self.enemies.is_empty(){

                        let pos=Point2{
                            
                         x:self.enemies[index-1].pos.x+173.0,//x koordinatite na predhodiq + razmera na kartinkata
                         y:self.enemies[index-1].pos.y,//y koordinatata na predhodiq + razmera na kartinkata

                        };
                        //self.current_wave+self.current_wave/2 so every other wave has 2 more health
                        let enemy = Enemy::new(pos, EnemyType::WAVE1, self.current_wave+self.current_wave/2);
                        self.enemies.push(enemy);
                        }
                        else{
                            let enemy = Enemy::new(enemy_start_pos, EnemyType::WAVE1, self.current_wave+self.current_wave/2);
                            self.enemies.push(enemy);
                        }
                    }
                    //the wave is genereted and no new enemies need to spawn untill the next wave
                    //not working as intended
                    self.time_until_next_wave=NEXT_WAVE_IN;
                    self.is_wave_generated=true;
                    
                }

                2=>{
                    //wave shoud have 3 health
                    while self.enemies.len() <MAX_ENEMIES_PER_WAVE && !self.is_wave_generated
                    {
                        let index=self.enemies.len();

                        if !self.enemies.is_empty(){

                        let pos=Point2{
                         x:self.enemies[index-1].pos.x+173.0,//x koordinatite na predhodiq + razmera na kartinkata
                         y:self.enemies[index-1].pos.y,

                        };
                        
                        let enemy = Enemy::new(pos, EnemyType::WAVE2, self.current_wave+self.current_wave/2);
                        self.enemies.push(enemy);
                        }
                        else{
                            let enemy = Enemy::new(enemy_start_pos, EnemyType::WAVE2, self.current_wave+self.current_wave/2);
                            self.enemies.push(enemy);
                        }
                    }
        
                    //the wave is genereted and no new enemies need to spawn untill the next wave
                    self.time_until_next_wave=NEXT_WAVE_IN;
                    self.is_wave_generated=true;
                    
                }
                3=>{
                    //wave shoud have 5 health
                    while self.enemies.len() <MAX_ENEMIES_PER_WAVE && !self.is_wave_generated
                    {
                        let index=self.enemies.len();

                        if !self.enemies.is_empty(){

                        let pos=Point2{
                         x:self.enemies[index-1].pos.x+173.0,//x koordinatite na predhodiq + razmera na kartinkata
                         y:self.enemies[index-1].pos.y,

                        };
                        
                        let enemy = Enemy::new(pos, EnemyType::WAVE3, self.current_wave+self.current_wave/2);
                        self.enemies.push(enemy);
                        }
                        else{
                            let enemy = Enemy::new(enemy_start_pos, EnemyType::WAVE3, self.current_wave+self.current_wave/2);
                            self.enemies.push(enemy);
                        }
                    }
        
                    //the wave is genereted and no new enemies need to spawn untill the next wave
                    self.time_until_next_wave=NEXT_WAVE_IN;
                    self.is_wave_generated=true;
                }
                4=>{
                    //wave shoud have 6 health
                    while self.enemies.len() <MAX_ENEMIES_PER_WAVE && !self.is_wave_generated
                    {
                        let index=self.enemies.len();

                        if !self.enemies.is_empty(){

                        let pos=Point2{
                         x:self.enemies[index-1].pos.x+173.0,//x koordinatite na predhodiq + razmera na kartinkata
                         y:self.enemies[index-1].pos.y,

                        };
                        
                        let enemy = Enemy::new(pos, EnemyType::WAVE4, self.current_wave+self.current_wave/2);
                        self.enemies.push(enemy);
                        }
                        else{
                            let enemy = Enemy::new(enemy_start_pos, EnemyType::WAVE4, self.current_wave+self.current_wave/2);
                            self.enemies.push(enemy);
                        }
                    }
                    //the wave is genereted and no new enemies need to spawn untill the next wave
                    self.time_until_next_wave=NEXT_WAVE_IN;
                    self.is_wave_generated=true;
                }
                //this part is broken
                5=>{
                    if !self.is_wave_generated{
                        self.bg.state=BackgroundState::BossLevel;
                    //the boss spawns in the middle of the screen
                     let pos=Point2{
                        x:self.screen_width/2.0-350.0,
                        y:self.screen_height/2.0-350.0,
                    };
                        let enemy = Enemy::new(pos, EnemyType::BOSS_WAVE, 40);
                        self.enemies.push(enemy);
                        self.is_wave_generated=true;
                    }
                }
                6=>{
                    self.game_over=true;
                    
                }
                _=>(),
            }
            /* 
            //the whole new wave after curtain concept doesn't work there is something wrong
            if self.time_until_next_wave>0.0
            {
                self.time_until_next_wave -= seconds;
            }
            //check if the wave has been cleared or a new one should come*/
            if self.enemies.is_empty()//||self.time_until_next_wave<=0.0
            {
                self.is_wave_generated=false;
                self.current_wave+=1;
            }

            //for the player

            self.player.update(self.input.movement_x,self.input.movement_y, seconds, self.screen_width,self.screen_height);

            if self.player.time_until_next_shot>0.0
            {
                self.player.time_until_next_shot -= seconds;
            }

            //for the player shots

            if self.input.fire && self.player.time_until_next_shot < 0.0 {
                let shot_pos = Point2 {
                    x: self.player.pos.x - 62.0,
                    y: self.player.pos.y - 200.0,

                };

                //the damage depends on the score thus:1+self.score/SCORE_NEED_FORNEXT_UPGRADE, 1 being the base damage
                let shot = Player_shot::new(shot_pos,1+self.score/SCORE_NEED_FOR_NEXT_UPGRADE);

                self.player_shots.push(shot);

                let _ = self.assets.shot_sound.play(ctx);

                self.player.time_until_next_shot = self.player.attack_speed;
                self.player.state = PlayerState::Shooting;

            } else if !self.input.fire&&self.player.time_until_next_shot>0.25 {
                self.player.state = PlayerState::Normal;
            }

            if self.player.is_shield_active{
                self.player.state = PlayerState::Shielded;
            }
            
            for shot in self.player_shots.iter_mut() {
                shot.update(seconds);
            }


            //for enemy shots
            //they just dissapear I don't know why
            self.time_until_next_enemy_shot -= seconds;
            if self.time_until_next_enemy_shot <= 0.0 && self.enemy_shots.len()<=MAX_ENEMY_SHOTS_AT_A_TIME{
                println!("Vliza 1{}",self.time_until_next_enemy_shot);
                
                let random_point = Point2 {
                    x: self.rng.gen_range(0.0 .. self.screen_width - 100.0),
                    y: self.rng.gen_range(0.0 .. self.screen_height - 600.0),
                };

                let random_speed = self.rng.gen_range(50.0 .. 200.0);
                let enemy_shot=Enemy_shot::new(random_point,random_speed);
                
                self.enemy_shots.push(enemy_shot);
                self.time_until_next_enemy_shot = self.rng.gen_range(0.5 .. 1.8);
                println!("Vliza 2{}, {:?}",self.time_until_next_enemy_shot, self.enemy_shots);
            }

            //DOESNT WORK FOR SOME REASON
            for enemy_shot in self.enemy_shots.iter_mut() {
                enemy_shot.update(seconds);
            }

           
            self.handle_collisions(ctx);

            self.player_shots.retain(|shot| shot.is_alive && shot.pos.y >= 0.0);
            self.enemies.retain(|enemy| enemy.is_alive);

            self.enemy_shots.retain(|shot| shot.is_alive && shot.pos.y >= self.screen_height);//not sure what corresponds to the bottom of the screen

            }

        Ok(())
    }
    
    fn key_down_event(&mut self, ctx: &mut Context, input: keyboard::KeyInput, _repeat: bool) -> GameResult<()> {

        match input.keycode {
            Some(keyboard::KeyCode::D) =>self.in_debugmode=true,
            Some(keyboard::KeyCode::F) =>self.in_debugmode=false,
            Some(keyboard::KeyCode::Z) =>{self.player.is_shield_active = true;
                /* 
                while self.player.shield_power>0.0
                {
                    self.player.is_shield_active = true;
                    self.player.shield_power-= 1.0;//Not sure how to solve this problems without having the elapsed seconds
                     
                    //not sure how to stop it once its activated
                    if self.player.shield_power<=0.0
                    {
                        self.player.is_shield_active = false;
                        break;
                    }
                    
                }*/
                
            }
            Some(keyboard::KeyCode::Space) => self.input.fire = true,
            Some(keyboard::KeyCode::Left) => self.input.movement_x = -1.0,
            Some(keyboard::KeyCode::Right) => self.input.movement_x = 1.0,
            Some(keyboard::KeyCode::Up) => self.input.movement_y = -1.0,
            Some(keyboard::KeyCode::Down) => self.input.movement_y = 1.0,
            Some(keyboard::KeyCode::Escape) => ctx.request_quit(),
            _ => (),
        }

        Ok(())
    }
     
    fn key_up_event(&mut self, _ctx: &mut Context, input: keyboard::KeyInput) -> GameResult<()> {
        match input.keycode {
            Some(keyboard::KeyCode::Z) =>{
                self.player.is_shield_active = false;
                self.player.state=PlayerState::Normal;
            }
            Some(keyboard::KeyCode::Space) => self.input.fire = false,
            Some(keyboard::KeyCode::Left | keyboard::KeyCode::Right) => {
                self.input.movement_x = 0.0
            },
            Some(keyboard::KeyCode::Up | keyboard::KeyCode::Down)=>{
            self.input.movement_y=0.0
            },
            _ => (),
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let black = graphics::Color::from_rgb(0, 0, 0);
        let mut canvas = graphics::Canvas::from_frame(ctx, black);

        if self.game_over {
            let mut text = graphics::Text::new(format!("Game Over .\nScore: {}", self.score));
            text.set_scale(graphics::PxScale::from(60.0));

            let top_left = Point2 {
                x: (self.screen_width - text.dimensions(ctx).unwrap().w) / 2.0,
                y: (self.screen_height - text.dimensions(ctx).unwrap().h) / 2.0,
            };
            canvas.draw(&text, graphics::DrawParam::default().dest(top_left));
            canvas.finish(ctx)?;
            return Ok(())
        }
        //drawing the background
        self.bg.draw(&mut canvas, &self.assets);

        //drawing the player
        self.player.draw(&mut canvas, &self.assets);

        //drawing the player shots
        for shot in self.player_shots.iter_mut() {
            shot.draw(&mut canvas, &self.assets);
        }

        //drawing the enemy shots 
        for enemy_shot in self.enemy_shots.iter_mut() {
            enemy_shot.draw(&mut canvas,&self.assets);
        }

        //drawing the enemies
       for enemy in self.enemies.iter_mut() {
            enemy.draw(&mut canvas,&self.assets);
        }
         
         if self.in_debugmode
         {
        //drawing the hitbox of enemies
        for enemy in &mut self.enemies {
            debug::draw_outline(enemy.bounding_rect(ctx,&self.assets), &mut canvas, ctx).unwrap();
        }
        
        //drawing hitboxes of shots
        for shot in self.player_shots.iter_mut() {
            debug::draw_outline(shot.bounding_rect(ctx,&self.assets), &mut canvas, ctx).unwrap();
        }
        //drawing the hitbox of the player

        debug::draw_outline(self.player.bounding_rect(ctx,&self.assets), &mut canvas, ctx).unwrap();
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
