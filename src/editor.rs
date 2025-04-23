use std::io::{self, Error, ErrorKind, Write};
use std::time::{Duration, Instant};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{self, Stylize},
    terminal::{self, ClearType},
};
use crate::storage::{load_notes, save_notes};

pub struct Editor {
    content: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    last_save: Instant,
    save_interval: Duration,
    modified: bool,
}

impl Editor {
    pub fn new() -> io::Result<Self> {
        // Enter alternate screen and enable raw mode
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), terminal::EnterAlternateScreen)?;
        
        // Load existing notes
        let content = match load_notes() {
            Ok(text) => {
                // Split content into lines
                text.lines().map(|line| line.to_string()).collect()
            }
            Err(_) => {
                // Start with an empty document
                vec![String::new()]
            }
        };
        
        Ok(Editor {
            content,
            cursor_x: 0,
            cursor_y: 0,
            last_save: Instant::now(),
            save_interval: Duration::from_secs(5), // Auto-save every 5 seconds
            modified: false,
        })
    }
    
    pub fn run(&mut self) -> io::Result<()> {
        self.draw_screen()?;
        
        loop {
            // Check if we should auto-save
            if self.modified && self.last_save.elapsed() >= self.save_interval {
                self.save()?;
            }
            
            // Handle input
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) && key_event.code == KeyCode::Char('q') {
                        break; // Exit on Ctrl+Q
                    }
                    
                    if self.handle_keypress(key_event.code, key_event.modifiers)? {
                        self.modified = true;
                    }
                    
                    self.draw_screen()?;
                }
            }
        }
        
        // Save before exiting
        if self.modified {
            self.save()?;
        }
        
        // Cleanup: leave alternate screen and disable raw mode
        execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        
        Ok(())
    }
    
    fn handle_keypress(&mut self, key: KeyCode, modifiers: KeyModifiers) -> io::Result<bool> {
        let mut content_modified = false;
        
        match key {
            KeyCode::Char(c) => {
                if modifiers.contains(KeyModifiers::CONTROL) {
                    if c == 's' {
                        self.save()?;
                    }
                } else {
                    // Insert character at cursor position
                    if self.cursor_y >= self.content.len() {
                        self.content.push(String::new());
                    }
                    
                    let line = &mut self.content[self.cursor_y];
                    if self.cursor_x >= line.len() {
                        line.push(c);
                    } else {
                        line.insert(self.cursor_x, c);
                    }
                    
                    self.cursor_x += 1;
                    content_modified = true;
                }
            }
            KeyCode::Backspace => {
                if self.cursor_x > 0 {
                    let line = &mut self.content[self.cursor_y];
                    line.remove(self.cursor_x - 1);
                    self.cursor_x -= 1;
                    content_modified = true;
                } else if self.cursor_y > 0 {
                    // Merge with previous line
                    let current_line = self.content.remove(self.cursor_y);
                    self.cursor_y -= 1;
                    self.cursor_x = self.content[self.cursor_y].len();
                    self.content[self.cursor_y].push_str(&current_line);
                    content_modified = true;
                }
            }
            KeyCode::Delete => {
                if self.cursor_y < self.content.len() {
                    let line = &mut self.content[self.cursor_y];
                    if self.cursor_x < line.len() {
                        line.remove(self.cursor_x);
                        content_modified = true;
                    } else if self.cursor_y < self.content.len() - 1 {
                        // Merge with next line
                        let next_line = self.content.remove(self.cursor_y + 1);
                        self.content[self.cursor_y].push_str(&next_line);
                        content_modified = true;
                    }
                }
            }
            KeyCode::Enter => {
                // Split line at cursor
                let current_line = &mut self.content[self.cursor_y];
                let new_line: String;
                
                if self.cursor_x < current_line.len() {
                    new_line = current_line.split_off(self.cursor_x);
                } else {
                    new_line = String::new();
                }
                
                self.content.insert(self.cursor_y + 1, new_line);
                self.cursor_y += 1;
                self.cursor_x = 0;
                content_modified = true;
            }
            KeyCode::Up => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
                }
            }
            KeyCode::Down => {
                if self.cursor_y < self.content.len() - 1 {
                    self.cursor_y += 1;
                    self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
                }
            }
            KeyCode::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.content[self.cursor_y].len();
                }
            }
            KeyCode::Right => {
                if self.cursor_x < self.content[self.cursor_y].len() {
                    self.cursor_x += 1;
                } else if self.cursor_y < self.content.len() - 1 {
                    self.cursor_y += 1;
                    self.cursor_x = 0;
                }
            }
            _ => {}
        }
        
        Ok(content_modified)
    }
    
    fn draw_screen(&self) -> io::Result<()> {
        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        
        // Get terminal size
        let (width, height) = terminal::size()?;
        
        // Draw header
        let title = " term-securepad ";
        let left_pad = (width as usize - title.len()) / 2;
        execute!(
            io::stdout(),
            style::PrintStyledContent("=".repeat(left_pad).dark_cyan()),
            style::PrintStyledContent(title.bold().dark_cyan()),
            style::PrintStyledContent("=".repeat(width as usize - left_pad - title.len()).dark_cyan()),
            cursor::MoveToNextLine(1),
        )?;
        
        // Display content
        for (_, line) in self.content.iter().enumerate().take(height as usize - 3) {
            let display_line = if line.is_empty() { " " } else { line };
            execute!(io::stdout(), style::Print(display_line), cursor::MoveToNextLine(1))?;
        }
        
        // Draw footer
        let footer_y = height - 2;
        let status = format!("Line: {}/{} | Col: {} | [Ctrl+Q] Quit | [Ctrl+S] Save", 
                            self.cursor_y + 1, self.content.len(), self.cursor_x + 1);
        
        execute!(
            io::stdout(),
            cursor::MoveTo(0, footer_y),
            style::PrintStyledContent("=".repeat(width as usize).dark_cyan()),
            cursor::MoveTo(0, footer_y + 1),
            style::PrintStyledContent(status.dark_grey())
        )?;
        
        // Position cursor
        execute!(io::stdout(), cursor::MoveTo(self.cursor_x as u16, self.cursor_y as u16 + 2))?;
        
        io::stdout().flush()?;
        Ok(())
    }
    
    fn save(&mut self) -> io::Result<()> {
        let text = self.content.join("\n");
        match save_notes(&text) {
            Ok(_) => {
                self.last_save = Instant::now();
                self.modified = false;
                Ok(())
            }
            Err(e) => Err(Error::new(ErrorKind::Other, format!("Failed to save notes: {}", e)))
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        // Ensure we exit terminal raw mode even if there's a panic
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), terminal::LeaveAlternateScreen);
    }
}
