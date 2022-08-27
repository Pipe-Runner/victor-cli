use std::fmt;

pub struct Vector {
    data: Vec<f32>,
}

impl Vector {
    pub fn new(data: Vec<f32>) -> Self {
        Self { data }
    }

    /// Returns the length of the vector.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Adds the two vector and returns a new vector.
    /// Note: Does not modify the original vectors.
    pub fn add(&self, v1: Self) -> Self {
        let max_dim = self.len().max(v1.len());
        let min_dim = self.len().min(v1.len());

        let mut new_data: Vec<f32> = vec![0.0; max_dim];

        let mut last_idx: usize = 0;
        for i in 0..min_dim {
            new_data[i] = self.data[i] + v1.data[i];
            last_idx += 1;
        }

        // when dealing with unequal vectors
        if max_dim != min_dim {
            let larger_vec = if self.len() > v1.len() { self } else { &v1 };

            for i in last_idx..max_dim {
                new_data[i] = larger_vec.data[i];
            }
        }

        Self { data: new_data }
    }

    /// Subtracts the two vector and returns a new vector.
    /// Note: Does not modify the original vectors.
    pub fn sub(&self, v1: Self) -> Self {
        let max_dim = self.len().max(v1.len());
        let min_dim = self.len().min(v1.len());

        let mut new_data: Vec<f32> = vec![0.0; max_dim];

        let mut last_idx: usize = 0;
        for i in 0..min_dim {
            new_data[i] = self.data[i] - v1.data[i];
            last_idx += 1;
        }

        // when dealing with unequal vectors
        if max_dim != min_dim {
            let larger_vec = if self.len() > v1.len() { self } else { &v1 };

            for i in last_idx..max_dim {
                new_data[i] = -larger_vec.data[i];
            }
        }

        Self { data: new_data }
    }

    pub fn scalar_mul(&self, k: f32) -> Self {
        let mut new_data: Vec<f32> = vec![0.0; self.len()];

        for i in 0..self.len() {
            new_data[i] = self.data[i] * (k as f32);
        }

        Self { data: new_data }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: Vec<String> = Vec::new();

        for i in 0..self.len() {
            let temp = self.data[i].to_string();
            output.push(temp);
        }

        write!(f, "{}", output.join(", "))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0, 20.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0, 30.0]);
        let v3 = v1.add(v2);
        assert_eq!(v3.data, vec![5.0, 7.0, 9.0, 50.0]);
    }

    #[test]
    fn add_unequal() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0, 30.0]);
        let v3 = v1.add(v2);
        assert_eq!(v3.data, vec![5.0, 7.0, 9.0, 30.0]);
    }

    #[test]
    fn sub() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0, 20.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0, 30.0]);
        let v3 = v1.sub(v2);
        assert_eq!(v3.data, vec![-3.0, -3.0, -3.0, -10.0]);
    }

    #[test]
    fn sub_unequal() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0, 30.0]);
        let v3 = v1.sub(v2);
        assert_eq!(v3.data, vec![-3.0, -3.0, -3.0, -30.0]);
    }

    #[test]
    fn scalar_mul() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0, 20.0]);
        let v2 = v1.scalar_mul(2.0);
        assert_eq!(v2.data, vec![2.0, 4.0, 6.0, 40.0]);
    }
}
