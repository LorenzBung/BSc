#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct PascalsTriangle {
    height: u32,
}

impl PascalsTriangle {
    pub fn new(i: u32) -> Self {
        PascalsTriangle { height: i }
    }
    pub fn rows(&self) -> Vec<Vec<u32>> {
        let rows = self.height as usize;
        let mut matrix = Vec::with_capacity(rows);

        for line in 0..rows {
            let current = line as usize;
            matrix.push(Vec::with_capacity(current + 1));
            matrix[current].push(1);

            if current > 1 {
                let previous = current - 1;
                for index in 1..current {
                    let add = matrix[previous][index - 1] + matrix[previous][index];
                    matrix[current].push(add);
                }
            }

            if current > 0 {
                matrix[current].push(1);
            }
        }

        println!("{:?}", matrix);

        matrix
    }
}
