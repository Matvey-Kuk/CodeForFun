extern crate gnuplot;
extern crate time;
extern crate scirust;

use std::fs::File;
use std::fs::OpenOptions;
use gnuplot::*;
use time::{Duration, PreciseTime};
use std::io::Write;
use scirust::api::*;
use scirust::linalg::linear_system::*;

fn gauss(matrix:&mut Vec<Vec<f64>>) -> Vec<f64> {
    for iteration_number in 0..(matrix[0].len() - 1) {
       //Achieving zero column by row swapping
       let min_x = iteration_number;
       let max_y = matrix[0].len() - iteration_number;
       for y in (0..max_y).rev() {
           if matrix[min_x][y] == 0.0 {
               for y_to_swap_with in 0..y {
                   if matrix[min_x][y_to_swap_with] != 0.0 {
                       for x in 0..matrix.len() {
                           let buf = matrix[x][y];
                           matrix[x][y] = matrix[x][y_to_swap_with];
                           matrix[x][y_to_swap_with] = buf;
                       }
                   }
               }
           }
       }

       //Achieving zero column by reducing
       for y in 0..(matrix[0].len() - 1 - iteration_number) {
           if matrix[iteration_number][y] != 0.0 {
               let rate:f64 = matrix[iteration_number][y] / matrix[iteration_number][matrix[0].len() - 1 - iteration_number];
               for x in iteration_number..matrix.len() {
                   matrix[x][y] -= rate * matrix[x][matrix[0].len() - 1 - iteration_number];
                   if x == iteration_number {
                       matrix[x][y] = 0.0;
                   }
               }
           }
       }
   }

   let mut result:Vec<f64> = Vec::new();
   for y in 0..matrix[0].len() {
       let mut subtrahend:f64 = 0.0;
       let mut iteration_x = 0;
       for x in ((matrix.len() - 1 - y)..(matrix.len() - 1)).rev() {
           subtrahend += result[iteration_x] * matrix[x][y];
           iteration_x += 1;
       }
       result.push((matrix[matrix.len() - 1][y] - subtrahend) / matrix[matrix.len() - 2 - y][y]);
   }
   result.reverse();
   result
}

fn gauss2(matrix:&mut Vec<Vec<f64>>) -> Vec<f64> {
    let mut result:Vec<f64> = Vec::new();
    let mut linear_matrix:Vec<f64> = Vec::new();
    let mut linear_matrix_right:Vec<f64> = Vec::new();
    for x in 0..matrix.len() - 1 {
        for y in 0..matrix[0].len() {
            linear_matrix.push(matrix[x][y]);
        }
    }
    for y in 0..matrix[0].len() {
        linear_matrix_right.push(matrix[matrix.len() - 1][y]);
    }
    let a = matrix_cw_f64(matrix.len() - 1,matrix[0].len(), &linear_matrix);
    // println!("{}", a);
    let b = vector_f64(&linear_matrix_right);
    let x = GaussElimination::new(&a, &b).solve().unwrap();

    for y in 0..matrix[0].len() {
        result.push(x[y]);
    }

    result
}

fn print_matrix(matrix:&Vec<Vec<f64>>) {
    let mut file = OpenOptions::new().write(true).create(true).append(true).open("foo.txt").unwrap();
    let mut dump = String::new();

    if matrix.len() > 0 {
        for y in (0..matrix[0].len()).rev() {
            for x in 0..matrix.len() {
                dump = dump + &matrix[x][y].to_string();
                dump = dump + "\t";
                // print!("{}\t", matrix[x][y]);
            }
            dump = dump + "\n";
            // print!("\n");
        }
    }
    dump = dump + "\n";
    // print!("\n");

    file.write_all(dump.as_bytes());
}


struct Cell {
	temperature: f64,
    x_size: f64,
    y_size: f64
}


struct Plate {
	cells: Vec<Vec<Cell>>,
    time: f64,
    heat_coefficient_x: f64,
    heat_coefficient_y: f64
}

impl Plate {
    fn new(x_amount_of_cells:i64, y_amount_of_cells:i64, x_size:usize, y_size:usize) -> Plate {
		let mut result = Vec::new();
		let x_size = x_size as f64 / x_amount_of_cells as f64;
		let y_size = y_size as f64 / y_amount_of_cells as f64;
		for x in 0..x_amount_of_cells {
			result.push(Vec::new());
			for _ in 0..y_amount_of_cells {
				result[x as usize].push(
					Cell{
						x_size: x_size,
                        y_size: y_size,
						temperature: 0.0
					}
				);
			}
		}
        Plate {
			cells: result,
            time: 0.0,
            heat_coefficient_x: 1.0,
            heat_coefficient_y: 1.0
		}
    }

	fn apply_start_conditions(&mut self) {
		for x in 0..self.cells.len() {
			for y in 0..self.cells[0].len() {
				self.cells[x as usize][y as usize].temperature = 20.0;
			}
		}
	}

    fn get_linear_repr_for_i_j(&self, i:usize, j:usize) ->usize {
        self.cells.len() * j + i
    }

    fn get_i_j_for_linear_repr(&self, linear:usize) -> (usize,usize) {
        (linear % self.cells.len(), linear / self.cells.len())
    }

    fn apply_border_conditions(&mut self) {
        let y_size = self.cells[0].len();
        let x_size = self.cells.len();
        for x in 0..self.cells.len(){
            self.cells[x][0].temperature = self.cells[x][1].temperature;
            self.cells[x][y_size - 1].temperature = self.cells[x][y_size - 2].temperature;
        }

        for y in 0..y_size {
            self.cells[0][y].temperature = self.cells[1][y].temperature;
            self.cells[x_size - 1][y].temperature = 800.0;
        }
    }

    fn go_to_time(&mut self, to_time:f64, time_delta:f64) {

        while self.time < to_time {
            let start = PreciseTime::now();

            self.apply_border_conditions();
            let old_plate = self.clone();
            let mut _linear_eq_system = self.get_linear_eq_system(old_plate, 1.0);

            // print_matrix(&_linear_eq_system);

            let _new_temperatures = gauss2(&mut _linear_eq_system);

            // print_matrix(&_linear_eq_system);

            for i in 0.._new_temperatures.len() {
                let (x, y) = self.get_i_j_for_linear_repr(i);
                self.cells[x][y].temperature = _new_temperatures[i];
            }
            //
            println!("{}", start.to(PreciseTime::now()));
            self.time += time_delta;
        }

    }

    fn paint(&self) {
        let mut blocks = Vec::new();

        for y in 0..self.cells[0].len()
        {
            for x in 0..self.cells.len()
            {
                blocks.push(self.cells[x][y].temperature);
            }
        }

        let mut fg = Figure::new();

        fg.axes2d()
        .set_title("Surface", &[])
        .image(blocks.iter(), self.cells[0].len(), self.cells.len(), Some((0.0, 0.0, 5.0, 5.0)), &[])
        .set_x_label("X", &[])
        .set_y_label("Y", &[]);

        fg.show();
    }

    fn get_linear_eq_system(&self, old_plate: Plate, time_delta:f64) -> Vec<Vec<f64>> {
        let mut matrix = Vec::new();
        for x in 0..self.cells.len() * self.cells[0].len() + 1 {
            matrix.push(Vec::new());
            for y in 0..self.cells.len() * self.cells[0].len() {
                matrix[x].push(0.0);
            }
        }
        for y in 0..self.cells.len() * self.cells[0].len() {
            let (cell_x, cell_y) = self.get_i_j_for_linear_repr(y);
            let x_matrix_size = matrix.len();
            if cell_x == self.cells.len() - 1 || cell_x == 0 || cell_y == self.cells[0].len() - 1 || cell_y == 0 {
                matrix[x_matrix_size - 1][y] = old_plate.cells[cell_x][cell_y].temperature;
                matrix[y][y] = 1.0;
            } else {
                let cell_x_size = self.cells[cell_x][cell_y].x_size;
                let cell_y_size = self.cells[cell_x][cell_y].y_size;
                matrix[x_matrix_size - 1][y] = - old_plate.cells[cell_x][cell_y].temperature / time_delta;

                matrix[y][y] = -1.0 / time_delta - 2.0 * self.heat_coefficient_x / (cell_x_size * cell_x_size) - 2.0 * self.heat_coefficient_y / (cell_y_size * cell_y_size);

                matrix[self.get_linear_repr_for_i_j(cell_x - 1, cell_y)][y] =  self.heat_coefficient_x / (cell_x_size * cell_x_size);
                matrix[self.get_linear_repr_for_i_j(cell_x + 1, cell_y)][y] =  self.heat_coefficient_x / (cell_x_size * cell_x_size);

                matrix[self.get_linear_repr_for_i_j(cell_x, cell_y - 1)][y] =  self.heat_coefficient_y / (cell_y_size * cell_y_size);
                matrix[self.get_linear_repr_for_i_j(cell_x, cell_y + 1)][y] =  self.heat_coefficient_y / (cell_y_size * cell_y_size);
            }
        }
        matrix
    }
}


impl Clone for Plate {
    fn clone(&self) -> Plate {
        let mut cells = Vec::new();

        for x in 0..self.cells.len() {
            cells.push(Vec::new());
            for y in 0..self.cells[0].len() {
                cells[x].push(Cell {
                    temperature: self.cells[x][y].temperature,
                    x_size: self.cells[x][y].x_size,
                    y_size: self.cells[x][y].y_size
                });
            }
        }

        Plate {
            cells: cells,
            time: self.time,
            heat_coefficient_x: self.heat_coefficient_x,
            heat_coefficient_y: self.heat_coefficient_y
        }
    }
}

fn main() {
    let mut plate:Plate = Plate::new(50, 50, 5, 5);
    plate.apply_start_conditions();
    plate.go_to_time(5.0, 1.0);
    plate.paint();
}

#[cfg(test)]
mod test {

    extern crate rand;
    use self::rand::Rng;

    #[test]
    fn gauss_test(){
        let mut matrix:Vec<Vec<f64>> = Vec::new();
        let test_matrix_size = 40;

        let mut rng = self::rand::thread_rng();

        for x in 0..(test_matrix_size + 1) {
            matrix.push(Vec::new());
            for _ in 0..test_matrix_size {
                matrix[x].push(rng.gen::<f64>());
            }
        }

        let result:Vec<f64> = super::gauss(&mut matrix);

        for y in 0..matrix[0].len() {
            let mut left_summ = 0.0;
            for x in 0..(matrix.len() - 1) {
                left_summ += result[x] * matrix[x][y];
            }
            assert_eq!((matrix[matrix.len() - 1][y] * 100000.0) as i64, (100000.0 * left_summ) as i64);
        }
    }

    #[test]
    fn test_get_linear_i_j_representation(){
        let plate = super::Plate::new(10,15, 5, 5);
        for i in 0..plate.cells.len() {
            for j in 0..plate.cells[0].len()  {
                assert_eq!(
                    plate.get_i_j_for_linear_repr(plate.get_linear_repr_for_i_j(i, j)),
                    (i, j)
                );
            }
        }
    }
}
