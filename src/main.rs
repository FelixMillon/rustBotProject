use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::backend::CrosstermBackend;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use std::io::{self, stdout};
mod map;
mod entities;
mod id_generator;

use map::Map;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut id_generator = id_generator::IDGenerator::new();
    let mut stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut map = Map::new(20, 40, 43);
    map.generate_map_obstacles();
    map.add_bot(3,3,"scout",&mut id_generator);

    terminal.clear().unwrap();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().borders(Borders::ALL).title("Map");
            f.render_widget(block, size);

            for (i, row) in map.map_matrix.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    let x = j as u16;
                    let y = i as u16;
                    let text = Text::from(Span::raw(cell.to_string()));
                    f.render_widget(Paragraph::new(text), Rect::new(x, y, 1, 1));
                }
            }
        }).unwrap();
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('m') => { // reafficher la carte pour evolution. A remplacer par tic.
                        terminal.draw(|f| {
                            let size = f.size();
                            let block = Block::default().borders(Borders::ALL).title("Map");
                            f.render_widget(block, size);

                            for (i, row) in map.map_matrix.iter().enumerate() {
                                for (j, &cell) in row.iter().enumerate() {
                                    let x = j as u16;
                                    let y = i as u16;
                                    let text = Text::from(Span::raw(cell.to_string()));
                                    f.render_widget(Paragraph::new(text), Rect::new(x, y, 1, 1));
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
