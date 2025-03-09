use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Row, Table},
    style::{Style, Color},
    Terminal,
};
use ratatui::prelude::Constraint;
use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};
mod map; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let rows = 10;
    let cols = 10;
    let seed = 42;
    let mut game_map = map::Map::new(rows, cols, seed);

    let bot = map::Entity {
        id: 1,
        loc: map::Localization { x: 5, y: 5 },
        nature: map::Nature::Bot {
            function: 1,
            display: 'B',
        },
    };
    game_map.entities.insert(bot.id, bot);

    game_map.generate_map_obstacles(seed);

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let map_matrix = game_map.generate_display();
            let rows = map_matrix.iter().map(|row| {
                Row::new(row.iter().map(|&c| c.to_string()).collect::<Vec<String>>())  
            });

            // Création de la table avec les dimensions des colonnes
            let table = Table::new(
                rows,
                [
                    // Largeur des colonnes
                    Constraint::Length(4),
                    Constraint::Length(5),
                    Constraint::Length(7),
                ],
            )
            .block(Block::default().borders(Borders::ALL).title("Matrice"))
            .style(Style::default().fg(Color::White));

            // Rendu de la table sur le terminal
            f.render_widget(table, size);
        })?;

        // Gestion des événements (sortir si l'utilisateur appuie sur 'q')
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break; // Quitter la boucle si 'q' est pressé
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}

