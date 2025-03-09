use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::backend::TermionBackend;
use termion::input::TermRead;
use termion::event::{Key};
use std::io::{self, Write};
use ratatui::text::{Text, Span};
use rand::prelude::*;  // Pour test A EFFACER
mod map;

use map::Map;

fn main() {
    let stdout = io::stdout();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut map = Map::new(20, 40, 42);
    map.generate_map_obstacles(42);

    terminal.clear().unwrap();

    let stdin = io::stdin();

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
        for c in stdin.lock().keys() {
            match c.unwrap() {
                Key::Char('m') => {
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

                },
                Key::Esc => {
                    return;
                },
                _ => {}
            }
        }
    }
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
