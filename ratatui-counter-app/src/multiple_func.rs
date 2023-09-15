// // organizing imports
// use crossterm::{
//     event::{self, Event::Key, KeyCode::Char},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use ratatui::{
//     prelude::{CrosstermBackend, Terminal},
//     widgets::Paragraph,
// };
// // Typedefs and Type Aliases
// type Err = Box<dyn std::error::Error>;
// type Result<T> = std::result::Result<T, Err>;
// pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;
// // Define struct to encapsulate our app
// struct App {
//     counter: i64,
//     should_quit: bool,
// }
// // initializing the terminal
// fn startup() -> Result<()> {
//     enable_raw_mode()?;
//     // arg1: if error occurs pass it to stderr
//     execute!(std::io::stderr(), EnterAlternateScreen)?;
//     Ok(())
// }
// // cleans up the terminal
// fn shutdown() -> Result<()> {
//     execute!(std::io::stderr(), LeaveAlternateScreen)?;
//     disable_raw_mode()?;
//     Ok(())
// }

// // handles rendering of our app state
// fn ui(app: &App, f: &mut Frame<'_>) {
//     f.render_widget(
//         Paragraph::new(format!("Counter: {}", app.counter)),
//         f.size(),
//     )
// }

// // process user input and updates our app state
// fn update(app: &mut App) -> Result<()> {
//     if event::poll(std::time::Duration::from_millis(250))? {
//         if let Key(key) = event::read()? {
//             match key.code {
//                 Char('j') => app.counter += 1,
//                 Char('k') => app.counter -= 1,
//                 Char('q') => app.should_quit = true,
//                 _ => (),
//             }
//         }
//     }
//     Ok(())
// }

// fn run() -> Result<()> {
//     // ratatui terminal
//     let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

//     // app state
//     let mut app = App {
//         counter: 0,
//         should_quit: false,
//     };

//     loop {
//         // render app, arg is a closure
//         t.draw(|f| {
//             ui(&app, f);
//         })?;
//         // update state
//         update(&mut app)?;
//         // exit?
//         if app.should_quit {
//             break;
//         }
//     }
//     Ok(())
// }
// fn main() -> Result<()> {
//     startup()?;
//     let status = run();
//     shutdown()?;
//     status?;
//     Ok(())
// }
