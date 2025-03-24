use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::backend::CrosstermBackend;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use std::io::{self, stdout};
mod map;
mod robots;
mod resources;
mod id_generator;
mod events;
use events::EventType;
use map::Map;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut id_generator = id_generator::IDGenerator::new();
    let mut stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut map = Map::new(20, 40, 44);
    map.generate_map_obstacles();
    map.generate_resources(&mut id_generator, 10);
    map.add_bot(10,20,"scout",&mut id_generator);
    map.add_bot(10,20,"scout",&mut id_generator);
    map.add_bot(10,20,"scout",&mut id_generator);
    map.add_bot(10,20,"scout",&mut id_generator);
    map.add_bot(10,20,"scout",&mut id_generator);
    map.add_bot(10,20,"scout",&mut id_generator);
    map.add_bot(10,20,"scout",&mut id_generator);
    map.add_bot(9,21,"scout",&mut id_generator);  // TEST TO DELETE
    map.add_bot(11,19,"scout",&mut id_generator);  // TEST TO DELETE
    terminal.clear().unwrap();

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().borders(Borders::ALL).title("Map");
        f.render_widget(block, size);

        for (i, row) in map.generate_display().iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let x = j as u16;
                let y = i as u16;
                let text = Text::from(Span::raw(cell.display.to_string()));
                f.render_widget(Paragraph::new(text), Rect::new(x, y, 1, 1));
            }
        }
    }).unwrap();
    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('m') => { // reafficher la carte pour evolution. A remplacer par tic.
                        map.handle_event(EventType::Tick);
                        terminal.draw(|f| {
                            let size = f.size();
                            let block = Block::default().borders(Borders::ALL).title("Map");
                            f.render_widget(block, size);

                            for (i, row) in map.generate_display().iter().enumerate() {
                                for (j, &cell) in row.iter().enumerate() {
                                    let x = j as u16;
                                    let y = i as u16;
                                    let explore_value = map.map_matrix[i][j].explore;
                                    let mut gray_value = 0 as u8;
                                    if explore_value < 0 {
                                        gray_value = 0 as u8;
                                    }else if explore_value > 30{
                                        gray_value = 30 as u8;
                                    }else{
                                        gray_value = explore_value as u8;
                                    }
                                    gray_value = gray_value * 8;
                                    let mut color = Color::Rgb(gray_value, gray_value, gray_value);
                                    match cell.display {
                                        'C' => {
                                            color = Color::Rgb(0, gray_value, gray_value);
                                        }
                                        'E' => {
                                            color = Color::Rgb(gray_value, gray_value, 0);
                                        }
                                        'S' => {
                                            color = Color::Rgb(0,155, 0);
                                        }
                                        'G' => {
                                            color = Color::Rgb(255,0, 0);
                                        }
                                        _ => {}
                                    }
                                    let cell_style = Style::default().fg(color);

                                    let text = Text::from(Span::raw(cell.display.to_string()));
                                    f.render_widget(
                                        Paragraph::new(text).style(cell_style),
                                        Rect::new(x, y, 1, 1),
                                    );
                                }
                            }
                        }).unwrap();
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
