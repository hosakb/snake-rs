use std::io::Stdout;
use std::io::Write;
use std::io::stdout;

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize, size
};
use crossterm::{ExecutableCommand, queue};

use crate::point::Point;
use crate::snake::Snake;

pub struct Terminal{
    stdout: Stdout,
    pub width: u16,
    pub height: u16,
    original_size: (u16, u16),
    pub boarder: Vec<Point>,
}

impl Terminal {
    pub fn new(width: u16, height: u16, boarder: Vec<Point>) -> Terminal{
        Terminal{
            stdout: stdout(),
            width,
            height,
            original_size: size().unwrap(),
            boarder,
        }
    }

    pub fn setup_ui(&mut self) {

        let width = self.width;
        let height = self.height;

        enable_raw_mode().unwrap();

        queue!(self.stdout, EnterAlternateScreen, SetSize(width, height), Clear(ClearType::All), Hide).unwrap();
        self.stdout.flush().unwrap();
    } 

    pub fn clean_up_ui(&mut self) {

        let width = self.original_size.0;
        let height = self.original_size.1;

        disable_raw_mode().unwrap();
        
        queue!(self.stdout, LeaveAlternateScreen, SetSize(width, height), Show).unwrap();
        self.stdout.flush().unwrap();
    }
    
    fn draw_borders(&mut self) {
        for x in 0..self.width {
            queue!(self.stdout, MoveTo(x, 0), Print("█".to_string())).unwrap();
            self.boarder.push(Point::new(x, 0));

            let h = self.height;
            queue!(self.stdout, MoveTo(x, h - 1), Print("█".to_string())).unwrap();
            self.boarder.push(Point::new(x, self.height - 1));
        }
    
        for y in 0..self.height {

            queue!(self.stdout, MoveTo(0, y), Print("█".to_string())).unwrap();
            queue!(self.stdout, MoveTo(1, y), Print("█".to_string())).unwrap();
            self.boarder.push(Point::new(1, y));

            let w = self.width;
            queue!(self.stdout, MoveTo(w - 2, y), Print("█".to_string())).unwrap();
            queue!(self.stdout, MoveTo(w - 1, y), Print("█".to_string())).unwrap();
            self.boarder.push(Point::new(self.width - 2, y));
        }

        self.stdout.flush().unwrap();
    }

    fn draw_snake(&mut self, snake: Snake){

        snake.body.into_iter().enumerate().for_each(|(i, p)|{
            if i == 0{
                queue!(self.stdout, MoveTo::from(p), Print("▢".to_string())).unwrap();
                return
            }

            queue!(self.stdout, MoveTo::from(p), Print("o".to_string())).unwrap();
        });
    }
    
    fn draw_food(&mut self, food: Point) {
        queue!(self.stdout, MoveTo::from(food), Print("🍒".to_string())).unwrap();
    }

    pub fn render(&mut self, snake: Snake, food: Point){
        queue!(self.stdout, Clear(ClearType::All)).unwrap();
        self.draw_borders();
        self.draw_snake(snake);
        self.draw_food(food);
        self.stdout.flush().unwrap();
        
    }
}
