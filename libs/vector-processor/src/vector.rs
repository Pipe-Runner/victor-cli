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

    pub fn dot(&self, v1: &Self) -> f32 {
        let min_dim = self.len().min(v1.len());

        let mut dot_product: f32 = 0.0;
        for i in 0..min_dim {
            dot_product += self.data[i] * v1.data[i];
        }
        dot_product
    }

    // Only works for 3D or less
    pub fn cross(&self, v1: &Self) -> Self {
        assert!(
            self.len() <= 3 && v1.len() <= 3,
            "Cross product is only defined for 3D vectors"
        );

        let mut a = vec![0.0; 3];
        let mut b = vec![0.0; 3];

        for i in 0..self.len() {
            a[i] = self.data[i];
        }

        for i in 0..v1.len() {
            b[i] = v1.data[i];
        }

        let mut new_data: Vec<f32> = vec![0.0; 3];
        new_data[0] = a[1] * b[2] - a[2] * b[1];
        new_data[1] = a[2] * b[0] - a[0] * b[2];
        new_data[2] = a[0] * b[1] - a[1] * b[0];
        Self { data: new_data }
    }

    pub fn norm(&self) -> f32 {
        let mut sum: f32 = 0.0;
        for i in &self.data {
            sum += i.powi(2);
        }
        sum.sqrt()
    }

    // Answer is in radians
    pub fn find_angle_between(&self, v1: &Self) -> f32 {
        (self.dot(v1) / (self.norm() * v1.norm())).acos()
    }

    // Returns a 3D basis for a given vector.
    pub fn find_basis_with_one(v: &Self) -> (Self, Self, Self) {
        let temp = v.add(Vector::new(vec![1.0, 1.0, 1.0]));

        let b = v.cross(&temp);
        let c = b.cross(&v);

        (
            v.scalar_mul(1.0 / v.norm()),
            b.scalar_mul(1.0 / b.norm()),
            c.scalar_mul(1.0 / c.norm()),
        )
    }

    // Returns a 3D basis for a given vector.
    pub fn find_basis_with_two(v1: &Self, v2: &Self) -> (Self, Self, Self) {
        if v1.dot(v2) == 1.0 {
            // vectors are parallel
            return Self::find_basis_with_one(v1);
        }

        let a = v1.cross(v2);
        let b = v1.cross(&a);

        (
            v1.scalar_mul(1.0 / v1.norm()),
            a.scalar_mul(1.0 / a.norm()),
            b.scalar_mul(1.0 / b.norm()),
        )
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
    fn find_angle_between() {
        let v1 = Vector::new(vec![1.0, 0.0, 0.0]);
        let v2 = Vector::new(vec![-1.0, 0.0, 0.0]);
        assert_eq!(v1.find_angle_between(&v2), std::f32::consts::PI);
    }

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
        assert_eq!(v1.scalar_mul(2.0).data, vec![2.0, 4.0, 6.0, 40.0]);
    }

    #[test]
    fn dot() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(v1.dot(&v2), 14.0);
    }

    #[test]
    fn dot_unequal() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0, 20.0]);
        let v2 = Vector::new(vec![1.0, 2.0]);
        assert_eq!(v1.dot(&v2), 5.0);
    }

    #[test]
    fn cross() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![3.0, 2.0, 1.0]);
        assert_eq!(v1.cross(&v2).data, vec![-4.0, 8.0, -4.0]);
    }

    #[test]
    fn cross_unequal() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![3.0, 2.0]);
        assert_eq!(v1.cross(&v2).data, vec![-6.0, 9.0, -4.0]);
    }
}
