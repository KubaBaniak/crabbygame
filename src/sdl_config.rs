extern crate sdl2;
use crate::characters::player::Direction;
use crate::characters::player::*;
use crate::characters::enemy::*;
use crate::World;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{ WindowCanvas, Texture };
use sdl2::image::{ self, LoadTexture, InitFlag };
use sdl2::rect::{ Point, Rect };

use std::collections::VecDeque;
use std::time::Duration;

const PLAYER_MOVEMENT_SPEED: i32 = 20;

fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        J => 0,
        K => 1,
        H => 2,
        L => 3,
    }
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player
    ) 
    -> Result<(), String>
{
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (frame_width, frame_height) = player.sprite.size();

    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y() + frame_height as i32 * direction_spritesheet_row(player.direction),
        frame_width,
        frame_height
        );
    // make center of screen as (0, 0)
    let screen_position = player.position + Point::new(width as i32 / 2,
                                                       height as i32 / 2);
    // create rectangle in center width the width of sprite fragment
    let screen_rect = Rect::from_center(screen_position,
                                        frame_width,
                                        frame_height);
    // get the texture, copy the fragment of original sprice and put it in created rectangle
    canvas.copy(texture, current_frame, screen_rect)?;
    canvas.present();
    Ok(())
}

fn update_player(player: &mut Player, arrow_keys: &VecDeque<Keycode>) {

    // check if two opposite keys are pressed
    if arrow_keys.len() > 1 {
        let vertical_keys = [Keycode::K, Keycode::J];
        let horizontal_keys = [Keycode::H, Keycode::L];

        let has_vertical_keys = arrow_keys.iter()
            .all(|&key| vertical_keys.contains(&key));
        let has_horizontal_keys = arrow_keys.iter()
            .all(|&key| horizontal_keys.contains(&key));

        if (has_horizontal_keys || has_vertical_keys) && arrow_keys.len() == 2 || arrow_keys.len() == 4 {
            player.speed = 0;
            return
        } 
    }
    // Set direction to last pressed key
    if let Some(last_key) = arrow_keys.back() {
        player.direction = match *last_key {
            Keycode::K => Direction::K,
            Keycode::J => Direction::J,
            Keycode::L => Direction::L,
            Keycode::H => Direction::H,
            _ => player.direction,
        };
    }

    player.speed = if arrow_keys.is_empty() {
        0
    } else {
        PLAYER_MOVEMENT_SPEED
    };

    if player.speed != 0 {
        player.current_frame = (player.current_frame + 1) % 2;
    }

    match player.direction {
        Direction::K => {
            player.position = player.position.offset(0, -player.speed)
        },
        Direction::J => {
            player.position = player.position.offset(0, player.speed)
        },
        Direction::L => {
            player.position = player.position.offset(player.speed, 0)
        },
        Direction::H => {
            player.position = player.position.offset(-player.speed, 0)
        },
    }
}

pub fn initalize_sdl() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("Game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initalize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/character.png")?;

    // init the world and players

    let mut world = World {
        player: Player::new(Point::new(0, 0), Rect::new(0, 0, 59, 88)),
        enemy: Enemy::new()
    };
    let mut arrow_keys: VecDeque<Keycode> = VecDeque::new();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                        break 'running;
                }
                Event::KeyDown { keycode: Some(key), repeat: false, ..} => {
                    match key {
                        Keycode::K | Keycode::J | Keycode::H | Keycode::L => {
                            arrow_keys.push_back(key);
                            println!("pushing {:?}", key);
                        },
                        _ => {}
                    };
                },
                Event::KeyUp { keycode: Some(key), repeat: false, ..} => {
                    match key {
                        Keycode::K | Keycode::J | Keycode::H | Keycode::L => {
                            if let Some(index) = arrow_keys.iter()
                                .position(|&k| k == key){
                                arrow_keys.remove(index);
                            };
                        },
                        _ => {}
                    };
                },
                _ => {}
            };
        };
        i = (i + 1) % 255;

        update_player(&mut world.player, &arrow_keys);

        render(&mut canvas, Color::RGB(i, 64, 255-i), &texture, &world.player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }
    Ok(())
}
