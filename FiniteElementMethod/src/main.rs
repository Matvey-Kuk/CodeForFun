extern crate gnuplot;
extern crate time;
extern crate scirust;

use std::num;
use std::fs::File;
use std::fs::OpenOptions;
use gnuplot::*;
use time::{Duration, PreciseTime};
use std::io::Write;
use scirust::api::*;
use scirust::linalg::linear_system::*;


fn gauss(matrix:&mut Vec<Vec<f64>>) -> Vec<f64> {
    for iteration_number in 0..(matrix[0].len() - 1) {
        // println!("{}", iteration_number / (matrix[0].len() - 1);
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
            // if y == 10*21 + 10 {
            dump = dump + &y.to_string();
            dump = dump + "|\t";
            // }
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

fn solve_analytical(start_x:f64, l:f64, amount_of_elements: usize) -> Vec<f64> {
    let mut result = Vec::new();
    for inc in 0..amount_of_elements + 1 {
        let x = start_x + inc as f64 * l;
        result.push(
            1.0/(11.0 * (1.0 + (31.0 * (11.0 as f64).powf(0.5) as f64).exp()))
            *
            (-0.5 * (11.0 as f64).powf(0.5) * (x + 1.0) as f64).exp()
            *
            (
                (-117.0 * (
                    (11.0 as f64).powf(0.5) * x as f64
                    ).exp())
                +
                (7.0 * (
                    0.5 * (11.0 as f64).powf(0.5) * (x + 1.0) as f64
                    ).exp())
                +
                (7.0 * (
                    0.5 * (11.0 as f64).powf(0.5) * (x + 63.0) as f64
                    ).exp())
                +
                (10.0 * (11.0 as f64).powf(0.5) * (
                    0.5 * (11.0 as f64).powf(0.5) * (2.0 * x + 31.0) as f64
                    ).exp())
                -
                (10.0 * (11.0 as f64).powf(0.5) * (
                    33.0 * (11.0 as f64).powf(0.5) / 2.0 as f64
                    ).exp())
                -
                (117.0 * (
                    32.0 * (11.0 as f64).powf(0.5) as f64
                    ).exp())
            )
            );
    }
    result
}

struct FEM {
    start_x:f64,
    end_x:f64,
    amount_of_elements:usize,
    right_matrix:Vec<f64>,
    form_matrix:Vec<Vec<f64>>,
    matrix:Vec<Vec<f64>>
}

impl FEM {
    fn new_linear_form_function (start_x:f64, end_x:f64, amount_of_elements:usize) -> FEM {
        let l:f64 = (end_x - start_x) / amount_of_elements as f64;

        let form_matrix_size = 2;

        let mut form_matrix = Vec::new();
        for x in 0..form_matrix_size {
            form_matrix.push(Vec::new());
            for _ in 0..form_matrix_size {
                form_matrix[x].push(0.0);
            }
        }

        form_matrix[0][0] = (4.0 / l) - (11.0 * l / 6.0);
        form_matrix[1][0] = - (4.0 / l) - (11.0 * l / 3.0);
        form_matrix[0][1] = - (4.0 / l) - (11.0 * l / 3.0);
        form_matrix[1][1] = (4.0 / l) - (11.0 * l / 6.0);

        let mut right_matrix = Vec::new();
        for _ in 0..form_matrix_size {
            right_matrix.push(0.0);
        }

        right_matrix[0] = -7.0 * l / 2.0;
        right_matrix[1] = -7.0 * l / 2.0;

        FEM {
            start_x:start_x,
            end_x:end_x,
            amount_of_elements:amount_of_elements,
            matrix:FEM::generate_matrix(&form_matrix, &right_matrix, amount_of_elements),
            right_matrix:right_matrix,
            form_matrix:form_matrix,
        }
    }

    fn new_cube_form_function (start_x:f64, end_x:f64, amount_of_elements:usize) -> FEM {
        let l:f64 = (end_x - start_x) / amount_of_elements as f64;

        let form_matrix_size = 4;

        let mut form_matrix = Vec::new();
        for x in 0..form_matrix_size {
            form_matrix.push(Vec::new());
            for _ in 0..form_matrix_size {
                form_matrix[x].push(0.0);
            }
        }

        form_matrix[0][0] = 4.0 * 13.0 / (40.0 * l) - 11.0 * 19.0 * l / 1680.0;
        form_matrix[1][0] = 4.0 * -27.0 / (20.0 * l) - 11.0 * -3.0 * l / 140.0;
        form_matrix[2][0] = 4.0 * 189.0 / (40.0 * l) - 11.0 * 33.0 * l / 560.0;
        form_matrix[3][0] = 4.0 * -37.0 / (10.0 * l) - 11.0 * 8.0 * l / 105.0;

        form_matrix[0][1] = 4.0 * -27.0 / (20.0 * l) - 11.0 * -3.0 * l / 140.0;
        form_matrix[1][1] = 4.0 * 279.0 / (40.0 * l) - 11.0 * -27.0 * l / 560.0;
        form_matrix[2][1] = 4.0 * -54.0 / (5.0 * l) - 11.0 * 27.0 * l / 70.0;
        form_matrix[3][1] = 4.0 * 189.0 / (40.0 * l) - 11.0 * 33.0 * l / 560.0;

        form_matrix[0][2] = 4.0 * 189.0 / (40.0 * l) - 11.0 * 33.0 * l / 560.0;
        form_matrix[1][2] = 4.0 * -54.0 / (5.0 * l) - 11.0 * 27.0 * l / 70.0;
        form_matrix[2][2] = 4.0 * 279.0 / (40.0 * l) - 11.0 * -27.0 * l / 560.0;
        form_matrix[3][2] = 4.0 * -27.0 / (20.0 * l) - 11.0 * -3.0 * l / 140.0;

        form_matrix[0][3] = 4.0 * -37.0 / (10.0 * l) - 11.0 * 8.0 * l / 105.0;
        form_matrix[1][3] = 4.0 * 189.0 / (40.0 * l) - 11.0 * 33.0 * l / 560.0;
        form_matrix[2][3] = 4.0 * -27.0 / (20.0 * l) - 11.0 * -3.0 * l / 140.0;
        form_matrix[3][3] = 4.0 * 13.0 / (40.0 * l) - 11.0 * 19.0 * l / 1680.0;

        let mut right_matrix = Vec::new();
        for _ in 0..form_matrix_size {
            right_matrix.push(0.0);
        }

        right_matrix[0] = -7.0 * l / 8.0;
        right_matrix[1] = -7.0 * 3.0 * l / 8.0;
        right_matrix[2] = -7.0 * 3.0 * l / 8.0;
        right_matrix[3] = -7.0 * l / 8.0;

        FEM {
            start_x:start_x,
            end_x:end_x,
            amount_of_elements:amount_of_elements,
            matrix:FEM::generate_matrix(&form_matrix, &right_matrix, amount_of_elements),
            right_matrix:right_matrix,
            form_matrix:form_matrix,
        }
    }

    fn generate_matrix(form_matrix:&Vec<Vec<f64>>, right_matrix:&Vec<f64>, amount_of_elements:usize) -> Vec<Vec<f64>> {
        let mut matrix = Vec::new();

        let matrix_size:usize = (form_matrix.len() - 1) * (amount_of_elements - 1) + form_matrix.len();
        let form_matrix_size = form_matrix.len();

        for x in 0..matrix_size + 1 {
            matrix.push(Vec::new());
            for _ in 0..matrix_size {
                matrix[x].push(0.0);
            }
        }

        print_matrix(&matrix);

        for i in 0..amount_of_elements {
            for x in 0..form_matrix_size {
                for y in 0..form_matrix_size {
                    matrix[i * (form_matrix_size - 1) + x][matrix_size - i * (form_matrix_size - 1) - form_matrix_size + y] += form_matrix[x][y];
                }
            }
        }

        print_matrix(&matrix);


        for i in 0..amount_of_elements {
            for y in 0..form_matrix_size {
                matrix[matrix_size][matrix_size - i * (form_matrix_size - 1) - form_matrix_size + y] += right_matrix[y];
            }
        }

        print_matrix(&matrix);

        matrix
    }

    fn apply_boundary_conditions(&mut self) ->  &mut FEM {
        let matrix_size:usize = (self.form_matrix.len() - 1) * (self.amount_of_elements - 1) + self.form_matrix.len();

        for x in 0..matrix_size + 1 {
            self.matrix[x][matrix_size - 1] = 0.0;
        }

        self.matrix[0][matrix_size - 1] = 1.0;
        self.matrix[matrix_size][matrix_size - 1] = -10.0;

        self.matrix[matrix_size][0] -= 20.0;

        print_matrix(&self.matrix);
        self
    }

    fn solve(&mut self) {
        let l:f64 = (self.end_x - self.start_x) / self.amount_of_elements as f64;

        let result = gauss(&mut self.matrix);
        let analytical_result = solve_analytical(self.start_x, l, self.amount_of_elements);
        let mut max_error = 0.0;

        println!("{} {}", result.len(), analytical_result.len());

        let mut powered_result = Vec::new();
        for i in 0..analytical_result.len() {
            if (result[i*(self.form_matrix.len() - 1)] - analytical_result[i] as f64).abs() > max_error {
                max_error = (result[i*(self.form_matrix.len() - 1)] - analytical_result[i] as f64).abs();
            }
            println!("{} \t {} \t {}",i as f64 * l + self.start_x, result[i*(self.form_matrix.len() - 1)], analytical_result[i]);
            powered_result.push(result[i*(self.form_matrix.len() - 1)]);
        }
        println!("Max error: {}", max_error);

        let mut x_axis = Vec::new();

        for i in 0..result.len() {
            x_axis.push(i as f64 * l + self.start_x);
        }

        let mut fg = Figure::new();

        fg.clear_axes();
        fg.axes2d()
        .lines(x_axis.iter(), powered_result.iter(), &[Caption("FEM"), LineWidth(0.5), Color("black")])
        .lines(x_axis.iter(), analytical_result.iter(), &[Caption("Analytical"), LineWidth(0.5), Color("blue")]);

        fg.show();
    }
}

fn main() {
    {
        match File::create("foo.txt") {
            Ok(_) => {
            }
            Err(e) => {
                println!("Error writing file {}", e);
            }
        }

        FEM::new_linear_form_function(1.0, 32.0, 40)
        .apply_boundary_conditions()
        .solve();
    }
}

#[cfg(test)]
mod test {

    extern crate rand;
    use self::rand::Rng;

    #[test]
    fn gauss_test(){
        let mut matrix:Vec<Vec<f64>> = Vec::new();
        let test_matrix_size = 20;

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
}
