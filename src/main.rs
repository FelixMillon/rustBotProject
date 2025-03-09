use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::backend::CrosstermBackend;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use std::io::{self, stdout};
use rand::prelude::*;  // Pour test A EFFACER
mod map;

use map::Map;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut map = Map::new(20, 40, 42);
    map.generate_map_obstacles(42);

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
                    KeyCode::Char('m') => {
                        place_random_star(&mut map);

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
                    KeyCode::Esc => {
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

fn place_random_star(map: &mut Map) {
    let mut rng = rand::thread_rng();
    let mut empty_positions: Vec<(usize, usize)> = Vec::new();

    for (i, row) in map.map_matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == ' ' {
                empty_positions.push((i, j));
            }
        }
    }

    if !empty_positions.is_empty() {
        let (x, y) = empty_positions.choose(&mut rng).unwrap();
        map.map_matrix[*x][*y] = '*';
    }
}
