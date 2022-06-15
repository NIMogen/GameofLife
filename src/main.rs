use rand::distributions::{Distribution, Uniform};
use std::thread::sleep;
use std::time;

/*
If a neighbor cell is alive, but has one or no neighboring cells alive, then the cell dies in the next pass

If a cell is alive, but has four or more neighboring cells alive, then the cell dies in the next pass

A living cell with 2 or 3 living neighbors will stay alive

If a cell is dead, it will become alive in the next pass if it has 3 neighboring cells

Births and deaths occur at the same pass, so dying cells can contribute to the creation of new cells
*/


struct Grid {
    grid: Vec<Vec<i32>>,
}


impl Grid {
    fn new() -> Grid {
        Grid {
            grid: vec![vec![0; 40]; 40],
        }
    }
    
    fn count_neighbors(&self, row: usize, column: usize) -> i32 {
        let mut i;
        let mut j;
        if row == 0 { i = 0;}
        else { i = row - 1; };

        if column == 0 {j = 0;}
        else {j = column - 1;};

        let mut count = 0;
        while i <= row + 1 {

            while j <= column + 1 {


                if i < self.grid.len() {
                if (i == row && j == column) ||
                ({i as i32}  < 0 || {j as i32} < 0) ||
                (j >= self.grid[i].len() || j >= self.grid.len()) {
                    j += 1;
                    continue;
                }
                if self.grid[i][j] == 1 {
                    count += 1;
                }
                }

                j += 1;
            }
            if column == 0 {j = 0;}
            else {j = column - 1;}
            i += 1;
        }

    	count
    }

    fn init_grid(&mut self) {
        let mut i = 0;
        let mut j = 0;
        let mut rng = rand::thread_rng();
        let flip = Uniform::from(0..2);
        while i < self.grid.len() {
            while j < self.grid[i].len() {
                self.grid[i][j] = flip.sample(&mut rng);
                j += 1;
            }
            j = 0; 
            i += 1;
        }
        
        for i in self.grid.iter() {
            for j in i.iter() {
                print!("{} ", j);
            }
            println!("");
        }
    }

    fn pass_generation(&mut self) {
        let mut newgrid = Grid::new();
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < self.grid.len() {
            while j < self.grid[i].len() {
                let live_neighbors = self.count_neighbors(i, j);            
                
                if (self.grid[i][j] == 1 && (live_neighbors == 2 || live_neighbors == 3)) || 
                   (self.grid[i][j] == 0 && live_neighbors == 3) {
                    newgrid.grid[i][j] = 1;
                }

                j += 1;
            }
            j = 0;
            i += 1;
        }
        self.grid = newgrid.grid;
        self.display();
    }

    fn display(&self) {
        print!("\x1B[2J");
        for i in self.grid.iter() {
            for j in i.iter() {
                let txt: String;
                if *j == 1 {
                    txt = String::from("@");
                }
                else {
                    txt = String::from(" ");
                }
                print!("{} ", txt);
            }
            println!("");
        }
    }
}


fn main() {
    let mut v = Grid::new();
    v.init_grid();

    let t = time::Duration::from_millis(100);
    loop {
        v.pass_generation();
        sleep(t);
  }
}





