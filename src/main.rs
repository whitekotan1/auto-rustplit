use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;
use std::io::stdout;



fn main() -> Result<()> {
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut output = String::new();

    loop {
       
        terminal.draw(|f| {
            let text = format!("Input: {}", input);
            let p = Paragraph::new(text);
            f.render_widget(p, f.size());
        })?;
        

        

       
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => input.push(c),      
                KeyCode::Backspace => {
                    input.pop();                        
                }
            
                 KeyCode::Esc => break,                 
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}




