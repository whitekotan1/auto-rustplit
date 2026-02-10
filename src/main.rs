
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::io::stdout;
use std::process::Command;
use std::str;
use crossterm::execute;
use crossterm::event::KeyEventKind;


fn main() -> Result<()> {
   execute!(stdout(), EnterAlternateScreen)?;

    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut output: String = String::new();

    loop {
       
        terminal.draw(|frame| {
            


             let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(frame.area());

             frame.render_widget(
                Paragraph::new(input.as_str())
                
                .style(
                    Style::default().fg(Color::White).bg(Color::Rgb(55, 60, 87))
                )
                .block(Block::new().title("input").borders(Borders::ALL).border_style(Style::default().fg(Color::Green))),
                layout[0],
             );

             frame.render_widget(
                Paragraph::new(output.as_str())
                .style(
                    Style::default().fg(Color::White).bg(Color::Rgb(55, 60, 87))
                )
                .block(Block::new().title("output").borders(Borders::ALL)),
                layout[1],
           );



        })?;
        

    

        

        

        if let Event::Key(key) = event::read()? {
         if key.kind == KeyEventKind::Press {

                 match key.code {
                KeyCode::Char(c) => input.push(c),      
                KeyCode::Backspace => {
                    input.pop();                        
                }
                KeyCode::Enter => {
                    input.push('\n');
                }
                KeyCode::Tab  =>  {
                    output = send_input(&input)?;
                }
                 KeyCode::Esc => break,                 
                _ => {}
            }
           
           }
        }
    
    }
    disable_raw_mode()?;
   execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}






   fn send_input(input: &str) -> Result<String> {
    let output = if input.trim() != "" {
        Some(Command::new("ollama")
            .arg("run")
            .arg("gemma3:1b")
            .arg(input)
            .output()?)
    } else {
        println!("error: empty input");
        None
    };

    let s = match output {
        Some(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        None => String::new(),
    };

    Ok(s)
}
/* 
fn run_model() {

}*/