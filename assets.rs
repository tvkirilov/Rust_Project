use ggez::audio::{self, SoundSource};
use ggez::graphics::{self, Drawable};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use std::fmt::Debug;

pub struct Assets {

    pub background_normal_image: graphics::Image,
    pub background_boss_image:   graphics::Image,

    pub ship_normal_image:       graphics::Image,
    pub ship_shooting_image:     graphics::Image,
    pub ship_shielded_image:     graphics::Image,
    pub ship_shooting_shilded_image: graphics::Image,

    pub enemy_1_image:           graphics::Image,
    pub enemy_2_image:           graphics::Image,
    pub enemy_3_image:           graphics::Image,
    pub enemy_4_image:           graphics::Image,

    pub boss_image:              graphics::Image,

    pub projectile_lvl_1_image:   graphics::Image,
    pub projectile_lvl_2_image:   graphics::Image,
    pub projectile_lvl_3_image:   graphics::Image,
    pub projectile_lvl_4_image:   graphics::Image,

    pub enemy_projectile_image:   graphics::Image,
    
    pub shot_sound: audio::Source,
    pub boom_sound: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {

        let background_normal_image=graphics::Image::from_path(ctx, "/background.png")?;
        let background_boss_image=graphics::Image::from_path(ctx, "/boss_background.png")?;

 
        let ship_normal_image=graphics::Image::from_path(ctx, "/Normal.png")?;
        let ship_shooting_image=graphics::Image::from_path(ctx, "/ship_shooting.png")?;
        let ship_shielded_image=graphics::Image::from_path(ctx, "/Shielded.png")?;
        let ship_shooting_shilded_image=graphics::Image::from_path(ctx, "/ship_shooting_sheilded.png")?;

        let enemy_1_image=graphics::Image::from_path(ctx, "/enemy_1.png")?;
        let enemy_2_image=graphics::Image::from_path(ctx, "/enemy_2.png")?;
        let enemy_3_image=graphics::Image::from_path(ctx, "/enemy_3.png")?;
        let enemy_4_image=graphics::Image::from_path(ctx, "/enemy_4.png")?;


        let boss_image=graphics::Image::from_path(ctx, "/Boss.png")?;


        let projectile_lvl_1_image=graphics::Image::from_path(ctx, "/bullet_lvl1.png")?;
        let projectile_lvl_2_image=graphics::Image::from_path(ctx, "/bullet_lvl2.png")?;
        let projectile_lvl_3_image=graphics::Image::from_path(ctx, "/bullet_lvl3.png")?;
        let projectile_lvl_4_image=graphics::Image::from_path(ctx, "/bullet_lvl4.png")?;
        
        let enemy_projectile_image=graphics::Image::from_path(ctx, "/enemy_projectile_down.png")?;
        
        let mut shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        shot_sound.set_volume(0.5);

        let mut boom_sound = audio::Source::new(ctx, "/boom.ogg")?;
        boom_sound.set_volume(0.3);

        Ok(Assets{
            background_normal_image, background_boss_image,
            ship_normal_image, ship_shooting_image, ship_shielded_image,ship_shooting_shilded_image,
            enemy_1_image, enemy_2_image,enemy_3_image,enemy_4_image,
            boss_image,
            projectile_lvl_1_image,projectile_lvl_2_image,projectile_lvl_3_image,projectile_lvl_4_image,
            enemy_projectile_image,
            shot_sound,boom_sound,

        })
    }
}
