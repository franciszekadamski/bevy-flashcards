use bevy::prelude::*;
use::polars::prelude::*;
use std::env;

mod flashcard;
use crate::flashcard::properties::{Flashcard, Deck, Holder};

#[derive(Resource, Debug)]
struct SourceCSVFilePath(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Pass one input .csv file");
        std::process::exit(1);    
    }
    let file_path = SourceCSVFilePath(args[1].clone());

    App::new()
        .insert_resource(file_path)
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                load_flashcards,
                setup_camera,
                setup_text
            ).chain()
        )
        .add_systems(
            Update,
            control_flashcard_holder
        )
        .run();
}


fn wrap_text(text: String, width: usize) -> String {
    let mut wrapped_text = String::new();
    let mut line_length = 0;

    for word in text.split_whitespace() {
        let word_length = word.len();
        
        if line_length + word_length + 1 > width {
            wrapped_text.push('\n');
            line_length = 0;
        } else if line_length > 0 {
            wrapped_text.push(' ');
            line_length += 1;
        }
        
        wrapped_text.push_str(word);
        line_length += word_length;
    }

    wrapped_text.replace("\"", "")
}

fn load_flashcards(
    mut commands: Commands,
    source_path: Res<SourceCSVFilePath>
) {
    let mut df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some(source_path.0.clone().into()))
        .unwrap()
        .finish()
        .unwrap();

    let front_column = df.column("front").expect("Couldn't find column front");
    let back_column = df.column("back").expect("Couldn't find column back");

    let mut deck = Vec::new();
    
    for row_index in 0..df.height() {
        let front_text = front_column.get(row_index)
            .expect("couldn't access data at given index {row_index}")
            .to_string();
        let back_text = back_column.get(row_index)
            .expect("couldn't access data at given index {row_index}")
            .to_string();
        let flashcard = Flashcard::new(
            wrap_text(front_text, 18),
            wrap_text(back_text, 18),
        );
        deck.push(flashcard);
    }
    
    let deck = Deck::new(deck);
    commands.insert_resource(deck);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    deck: Res<Deck>
) {
    let deck = deck.0.clone();

    commands.spawn(
        (
            TextBundle::from_section(
                deck[0].front_text.clone(),
                TextStyle {
                    font: asset_server.load("cjk.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..default()
                }
            )
            .with_text_justify(JustifyText::Center)
            .with_style(
                Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(38.0),
                    left: Val::Percent(38.0),
                    ..default()
                }
            ),
            Holder::new(deck),
        )
    );
}

fn control_flashcard_holder(
    keycode: Res<ButtonInput<KeyCode>>,
    mut text_and_holder_query: Query<(&mut Text, &mut Holder)>
) {
    let (mut text, mut holder) = text_and_holder_query.single_mut();
    let text = &mut text.sections[0].value;
    
    if keycode.just_pressed(KeyCode::ArrowLeft) {
        holder.prev();
        *text = holder.text.clone();
    } else if keycode.just_pressed(KeyCode::ArrowRight) {
        holder.next();
        *text = holder.text.clone();
    } else if keycode.just_pressed(KeyCode::Space) {
        holder.flip();
        *text = holder.text.clone();
    }
}
