use crossterm::{
    cursor, style::{Color, SetBackgroundColor, SetForegroundColor}, terminal::{Clear, ClearType}, ExecutableCommand
};
use std::io::{stdout, Write};
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cella {
    Safe(usize, bool, bool), //count, hidden, flag
    Bomb(bool) // flag
}

pub struct Game {
    cells: Vec<Vec<Cella>>,
    x: usize,
    y: usize,
    lost: bool,
    won: bool,
    n_bombs: usize,
    n_flags: usize,
}

impl Game {
    pub fn new(w: usize, h: usize, bomb_prob: f32) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells: Vec<Vec<Cella>> = Vec::new();
        let mut n_bombs = 0;
        for _ in 0..h {
            let mut new_row: Vec<Cella> = Vec::new();
            for _ in 0..w {
                if rng.gen_range(0..100) > (bomb_prob*100.0) as i32 {
                    new_row.push(Cella::Safe(0, true, false));
                }else{
                    new_row.push(Cella::Bomb(false));
                    n_bombs+=1;
                }
            }
            cells.push(new_row);
        }

        let mut ret = Game {
            cells,
            x: w/2,
            y: h/2,
            lost: false,
            won: false,
            n_bombs,
            n_flags: 0,
        };

        for y in 0..h {
            for x in 0..w {
                if *ret.get_cell(x, y).unwrap() != Cella::Bomb(false) {
                    let n_bombe = ret.get_neighbors(x, y).iter().fold(0, |c:usize, n:&&Cella| {
                        c + match n {
                            Cella::Safe(..) => 0,
                            Cella::Bomb(..) => 1
                        }
                    });
                    ret.cells.get_mut(y).unwrap()[x] = Cella::Safe(n_bombe, true, false);
                }
            }
        }

        ret
    }

    pub fn is_won(&self) -> bool {
        self.won
    }
    pub fn is_lost(&self) -> bool {
        self.lost
    }
    pub fn get_h(&self) -> usize {
        self.cells.len()
    }
    pub fn get_w(&self) -> usize {
        match self.cells.get(0) {
            Some(x) => x.len(),
            None => 0,
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<&Cella> {
        let mut ret: Vec<&Cella> = Vec::new();
        let w = self.get_w() as i32;
        let h = self.get_h() as i32;
        for i in -1..=1 {
            for j in -1..=1 {
                let off_x: i32 = x as i32 + i;
                let off_y: i32 = y as i32 + j;
                if off_y>=0 && off_y < h && off_x>=0 && off_x < w {
                    ret.push(self.cells.get(off_y as usize).unwrap().get(off_x as usize).unwrap());
                }
            }
        }
        return ret;
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cella> {
        self.cells.get(y)?.get(x)
    }
    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cella> {
        self.cells.get_mut(y)?.get_mut(x)
    }


    pub fn move_up(&mut self) {
        let old_y = self.y;
        self.y = self.y.saturating_sub(1);
        self.draw_cell(self.x, old_y);
        self.draw_cell(self.x, self.y);
    }
    pub fn move_left(&mut self) {
        let old_x = self.x;
        self.x = self.x.saturating_sub(1);
        self.draw_cell(old_x, self.y);
        self.draw_cell(self.x, self.y);
    }
    pub fn move_down(&mut self) {
        let old_y = self.y;
        if self.y+1 < self.get_h() {
            self.y += 1;
        }
        self.draw_cell(self.x, old_y);
        self.draw_cell(self.x, self.y);
    }
    pub fn move_right(&mut self) {
        let old_x = self.x;
        if self.x+1 < self.get_w() {
            self.x += 1;
        }
        self.draw_cell(old_x, self.y);
        self.draw_cell(self.x, self.y);
    }

    pub fn long_up(&mut self) {
        /*
        let x = self.x;
        let mut y = self.y;
        let old_cell = self.get_cell(x,y).unwrap();
        loop {
            y = y.saturating_sub(0);
            if y==0 {break;}
            let new_cell = self.get_cell(x,y).unwrap();
            if new_cell == old_cell {continue}
            if match new_cell {
                Cella::Safe(_,false,false) => {
                    match old_cell {
                        Cella::Safe(_,false,_) => false,
                        _ => true
                    }
                }
                _ => true
            } {break};
        }

        self.y = y;
        */
    }
    pub fn long_down(&mut self) {
    }
    pub fn long_left(&mut self) {
    }
    pub fn long_right(&mut self) {
    }

    fn click_coords(&mut self, x:usize, y:usize) {
        if self.lost {return;}
        let cel = self.get_cell_mut(x,y).unwrap().clone();

        if match cel {
            Cella::Bomb(_) => {
                self.lost = true;
                false
            }
            Cella::Safe(_, false, _) => {
                true
            }
            _ => {false}
        }{return}

        self.cells.get_mut(y).unwrap()[x] = match cel {
            Cella::Safe(n, true, f) => {
                if f {
                    self.n_flags-=1;
                }
                Cella::Safe(n, false, false)
            }
            _ => cel
        };

        let h = self.get_h();
        let w = self.get_w();
        match self.cells.get(y).unwrap().get(x).unwrap() {
            Cella::Safe(0, false, _) => {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i==0 && j==0 {continue}
                        let off_y = y as i32 + i;
                        let off_x = x as i32 + j;
                        if off_y >= 0 && off_x >= 0 && off_y < h as i32 && off_x < w as i32 {
                            self.click_coords(off_x as usize, off_y as usize);
                        }
                    }
                }
            }
            _ => {}
        }
        
        self.draw_cell(x,y);
    }

    fn check_win(&mut self) {
        self.won = self.cells.iter().all(|row|
            row.iter().all(|c| match c {
                Cella::Bomb(_) => true,
                Cella::Safe(_, false, _) => true,
                _ => false
            })
        );
    }

    pub fn click(&mut self) {
        let x = self.x;
        let y = self.y;
        self.click_coords(x,y);

        self.check_win();
    }
    pub fn flag(&mut self) {
        let x = self.x;
        let y = self.y;

        let old = self.get_cell(x,y).unwrap().clone();
        self.cells.get_mut(y).unwrap()[x] = match old {
            Cella::Safe(a,b,f) => Cella::Safe(a,b,!f),
            Cella::Bomb(f) => Cella::Bomb(!f)
        };
        match old {
            Cella::Safe(_, true, false) | Cella::Bomb(false) => {
                self.n_flags+=1;
            }
            Cella::Safe(_, true, true) | Cella::Bomb(true) => {
                self.n_flags-=1;
            }
            _ => {}
        }
        
        self.update_bomb_count();
        self.draw_cell(x,y);
    }

    fn update_bomb_count(&self) {
        let mut stdout = stdout();

        stdout.execute(cursor::MoveTo(0, self.get_h() as u16)).unwrap();
        stdout.execute(SetBackgroundColor(Color::Reset)).unwrap();
        stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
        write!(stdout, "{}/{}", self.n_flags, self.n_bombs).unwrap();
    }

    fn draw_cell(&self, x:usize, y:usize) {
        let mut stdout = stdout();

        stdout.execute(cursor::MoveTo(x as u16 * 2, y as u16)).unwrap();
        stdout.execute(SetBackgroundColor(Color::Black)).unwrap();
        stdout.execute(SetForegroundColor(Color::White)).unwrap();

        let hovering = x == self.x && y == self.y;
        if hovering {
            stdout.execute(SetBackgroundColor(Color::White)).unwrap();
            stdout.execute(SetForegroundColor(Color::Black)).unwrap();
        }
        let cella: &Cella = self.get_cell(x, y).unwrap();
        match cella {
            Cella::Bomb(false) | Cella::Safe(_, true, false) => {
                // ▓░▒█
                if hovering {
                    stdout.execute(SetForegroundColor(Color::White)).unwrap();
                } else {
                    stdout.execute(SetForegroundColor(Color::Rgb{r:150,g:150,b:150})).unwrap();
                }
                write!(stdout, "██").unwrap();
            },
            Cella::Safe(0, false, _) => {
                write!(stdout, "  ").unwrap();
            }
            Cella::Safe(n, false, _) => {
                if hovering {
                    stdout.execute(SetForegroundColor(Color::Black)).unwrap();
                } else {
                    match n {
                        0 => {stdout.execute(SetForegroundColor(Color::Black)).unwrap();}
                        1 => {stdout.execute(SetForegroundColor(Color::Blue)).unwrap();}
                        2 => {stdout.execute(SetForegroundColor(Color::Green)).unwrap();}
                        3 => {stdout.execute(SetForegroundColor(Color::Red)).unwrap();}
                        4 => {stdout.execute(SetForegroundColor(Color::Magenta)).unwrap();}
                        5 => {stdout.execute(SetForegroundColor(Color::DarkYellow)).unwrap();}
                        6 => {stdout.execute(SetForegroundColor(Color::Blue)).unwrap();}
                        7 => {stdout.execute(SetForegroundColor(Color::Green)).unwrap();}
                        _ => {stdout.execute(SetForegroundColor(Color::Red)).unwrap();}
                    }
                }
                write!(stdout, " {}", n).unwrap();
            }
            Cella::Safe(_, true, true) | Cella::Bomb(true) => {
                stdout.execute(SetBackgroundColor(Color::DarkRed)).unwrap();
                stdout.execute(SetForegroundColor(Color::Black)).unwrap();
                if hovering {
                    stdout.execute(SetBackgroundColor(Color::White)).unwrap();
                }
                write!(stdout, "FF").unwrap();
            }
        }
        stdout.flush().unwrap();
    }

    pub fn draw(&self) {
        if self.lost {
            self.draw_dead()
        } else {
            self.draw_alive()
        }
    }

    pub fn refresh(&self) {
        let mut stdout = stdout();
        stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
        stdout.execute(SetBackgroundColor(Color::Reset)).unwrap();
        stdout.execute(Clear(ClearType::All)).unwrap();
        self.draw();
        self.update_bomb_count();
    }

    fn draw_alive(&self) {
        let h = self.get_h();
        let w = self.get_w();

        // Draw the map with a blinking cursor at the player's position
        for y in 0..h {
            for x in 0..w {
                self.draw_cell(x,y);
            }
        }
    }

    fn draw_dead(&self) {
        let mut stdout = stdout();
        let h = self.get_h();
        let w = self.get_w();

        // Draw the map with a blinking cursor at the player's position
        for y in 0..h {
            stdout.execute(cursor::MoveTo(0, y as u16)).unwrap();
            for x in 0..w {
                stdout.execute(SetBackgroundColor(Color::Black)).unwrap();
                stdout.execute(SetForegroundColor(Color::White)).unwrap();

                let cella: &Cella = self.get_cell(x, y).unwrap();
                match cella {
                    Cella::Safe(_, true, false) => {
                        // ▓░▒█
                        stdout.execute(SetForegroundColor(Color::Rgb{r:150,g:150,b:150})).unwrap();
                        write!(stdout, "██").unwrap();
                    },
                    Cella::Safe(0, false, _) => {
                        write!(stdout, "  ").unwrap();
                    }
                    Cella::Safe(n, false, _) => {
                        match n {
                            0 => {stdout.execute(SetForegroundColor(Color::Black)).unwrap();}
                            1 => {stdout.execute(SetForegroundColor(Color::Blue)).unwrap();}
                            2 => {stdout.execute(SetForegroundColor(Color::Green)).unwrap();}
                            3 => {stdout.execute(SetForegroundColor(Color::Red)).unwrap();}
                            4 => {stdout.execute(SetForegroundColor(Color::Magenta)).unwrap();}
                            5 => {stdout.execute(SetForegroundColor(Color::DarkYellow)).unwrap();}
                            6 => {stdout.execute(SetForegroundColor(Color::Blue)).unwrap();}
                            7 => {stdout.execute(SetForegroundColor(Color::Green)).unwrap();}
                            _ => {stdout.execute(SetForegroundColor(Color::Red)).unwrap();}
                        }
                        write!(stdout, " {}", n).unwrap();
                    }
                    Cella::Safe(_, true, true) => {
                        stdout.execute(SetBackgroundColor(Color::Rgb{r:150,g:150,b:150})).unwrap();
                        stdout.execute(SetForegroundColor(Color::Black)).unwrap();
                        write!(stdout, "FF").unwrap();
                    }
                    Cella::Bomb(true) => {
                        stdout.execute(SetBackgroundColor(Color::DarkRed)).unwrap();
                        stdout.execute(SetForegroundColor(Color::Black)).unwrap();
                        write!(stdout, "FF").unwrap();
                    }
                    Cella::Bomb(false) => {
                        stdout.execute(SetBackgroundColor(Color::DarkRed)).unwrap();
                        stdout.execute(SetForegroundColor(Color::Black)).unwrap();
                        write!(stdout, "XX").unwrap();
                    }
                }
                stdout.flush().unwrap();
            }
        }
        stdout.flush().unwrap();
    }
}
