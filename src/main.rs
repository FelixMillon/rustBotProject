use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::backend::TermionBackend;
use std::io;
use ratatui::text::{Text, Span};
mod map;

use map::{Map, Nature, Entity};

fn main() {
    let stdout = io::stdout();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut map = Map::new(50, 100, 42);
    map.generate_map_obstacles(42);
    let matrix = map.generate_display();

    terminal.clear().unwrap();

    terminal.draw(|f| {
        let size = f.size();

        let max_rows = size.height as usize;
        let max_cols = size.width as usize;

        let matrix_rows = matrix.len();
        let matrix_cols = if matrix_rows > 0 { matrix[0].len() } else { 0 };

        let rows_to_display = std::cmp::min(matrix_rows, max_rows);
        let cols_to_display = std::cmp::min(matrix_cols, max_cols);

        let block = Block::default().borders(Borders::ALL).title("Matrice");
        f.render_widget(block, size);

        for i in 0..rows_to_display {
            for j in 0..cols_to_display {
                let x = j as u16;
                let y = i as u16;
                let text = Text::from(Span::raw(matrix[i][j].to_string()));
                f.render_widget(Paragraph::new(text), Rect::new(x, y, 1, 1));
            }
        }
    }).unwrap();
}
