use ndarray::Array2;
use crate::utils::dot;

#[derive(Clone)]
pub struct Screenton {
    dot_size: usize,
    dot: Array2<f32>,
    dot_inv: Array2<f32>,
    lx_plus: usize,
    ly_plus: usize,
}

impl Screenton {
    pub fn new(dot_size: usize, lx_plus: usize, ly_plus:usize) -> Self {
        let (dot, dot_inv) = dot::create_dot(dot_size);

        Self {
            dot_size,
            dot,
            dot_inv,
            lx_plus,
            ly_plus
        }
    }
    pub fn run(&mut self, array: &mut Array2<f32>) {
        let(h,w)=(array.shape()[0],array.shape()[1]);

        for ly in 0..h {
            let ly2 = ly+self.ly_plus;
            let colum = ly2/self.dot_size;

            for lx in 0..w {
                let value = &mut array[[ly, lx]];
                if *value > 0.0 && *value < 1.0 {
                    let lx2 = lx+self.lx_plus;
                    let src_values= if (colum+ lx2/self.dot_size) % 2 == 1 {
                        self.dot_inv[[lx2 % self.dot_size,ly2 % self.dot_size]]
                    } else {
                        self.dot[[lx2 % self.dot_size,ly2 % self.dot_size]]
                    };
                    let src_value = src_values;
                    *value = if *value < src_value { 0.0 } else { 1.0 };
                }
            }
        }
    }
}

