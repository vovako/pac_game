use rusty_engine::prelude::*;

struct GameState {
	lives: u32,
	score: u32,
	move_dir: char,
}

fn main() {
	let mut game = Game::new();

	game.window_settings( WindowDescriptor {
    title: "My Awesome Game".into(),
    width: 700.0,
    height: 700.0,
	..Default::default()
	});

	let player = game.add_sprite("player", "pacman.png");
	player.scale = 0.1;

	game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.3);

	let game_state = GameState{
		lives: 3,
		score: 0,
		move_dir: 'r'
	};

	game.add_logic(game_logic);
	game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState){
	let player = engine.sprites.get_mut("player").unwrap();
	
	match game_state.move_dir {
		'u' => {
			player.translation.y += 100.0 * engine.delta_f32;
			player.rotation = std::f32::consts::PI / 2.0;
		},
		'r' => {
			player.translation.x += 100.0 * engine.delta_f32;
			player.rotation = 0.0;
		},
		'd' => {
			player.translation.y -= 100.0 * engine.delta_f32;
			player.rotation = std::f32::consts::PI * 3.0 / 2.0;
		},
		'l' => {
			player.translation.x -= 100.0 * engine.delta_f32;
			player.rotation = std::f32::consts::PI;
		},
		_ => {}
	}

	if engine.keyboard_state.pressed_any(&[KeyCode::D, KeyCode::Right]) {
		game_state.move_dir = 'r';
	} else if engine.keyboard_state.pressed_any(&[KeyCode::S, KeyCode::Down]) {
		game_state.move_dir = 'd';
	} else if engine.keyboard_state.pressed_any(&[KeyCode::A, KeyCode::Left]) {
		game_state.move_dir = 'l';
	} else if engine.keyboard_state.pressed_any(&[KeyCode::W, KeyCode::Up]) {
		game_state.move_dir = 'u';
	}

	println!("Жизней: {}", game_state.lives);
	println!("Текущий счет: {}", game_state.score);
}