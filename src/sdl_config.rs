extern crate sdl2;
use crate::characters::player::Direction;
use crate::{ Player, Enemy };
use crate::World;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{ WindowCanvas, Texture };
use sdl2::image::{ self, LoadTexture, InitFlag };
use std::collections::VecDeque;
use std::time::Duration;
use sdl2::rect::{ Point, Rect };

const PLAYER_MOVEMENT_SPEED: i32 = 20;

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
    // make center of screen as (0, 0)
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 /2);
    // create rectangle in center width the width of sprite fragment
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());
    // get the texture, copy the fragment of original sprice and put it in created rectangle
    canvas.copy(texture, player.sprite, screen_rect)?;
    canvas.present();
    Ok(())
}

fn update_player(player: &mut Player, arrow_keys: &VecDeque<Keycode>) {

    if arrow_keys.len() > 1 {
        let vertical_keys = [Keycode::Up, Keycode::Down];
        let horizontal_keys = [Keycode::Left, Keycode::Right];

        let has_vertical_keys = arrow_keys.iter()
            .all(|&key| vertical_keys.contains(&key));
        let has_horizontal_keys = arrow_keys.iter()
            .all(|&key| horizontal_keys.contains(&key));

        if (has_horizontal_keys || has_vertical_keys) && arrow_keys.len() == 2 || arrow_keys.len() == 4 {
            player.speed = 0;
            return
        } 
    }
    if let Some(last_key) = arrow_keys.back() {
        player.direction = match *last_key {
            Keycode::Up => Direction::Up,
            Keycode::Down => Direction::Down,
            Keycode::Right => Direction::Right,
            Keycode::Left => Direction::Left,
            _ => player.direction,
        };
    }

    player.speed = if arrow_keys.is_empty() {
        0
    } else {
        PLAYER_MOVEMENT_SPEED
    };

    match player.direction {
        Direction::Up => {
            player.position = player.position.offset(0, -player.speed)
        },
        Direction::Down => {
            player.position = player.position.offset(0, player.speed)
        },
        Direction::Right => {
            player.position = player.position.offset(player.speed, 0)
        },
        Direction::Left => {
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
                        Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => {
                            arrow_keys.push_back(key);
                            println!("pushing {:?}", key);
                        },
                        _ => {}
                    };
                },
                Event::KeyUp { keycode: Some(key), repeat: false, ..} => {
                    match key {
                        Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => {
                            if let Some(index) = arrow_keys.iter().position(|&k| k == key){
                                arrow_keys.remove(index);
                            };
                        },
                        _ => {}
                    };
                },
                _ => {println!("{:?}", event)}
            };
        };
        i = (i + 1) % 255;

        update_player(&mut world.player, &arrow_keys);

        render(&mut canvas, Color::RGB(i, 64, 255-i), &texture, &world.player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }
    Ok(())
}
