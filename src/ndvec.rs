/// NDVec
/// An N-Dimensional Array
/// Initialize with a 0.0 and mutate as needed
/// Using Graph like structure as the underlying Data structure
/// Keep a history of swaps to maintain order
/// when need just revert to get original order
/// [x:[],y:[],z:[],...n:[]]
///
///
#[derive(Debug)]
pub struct NDVec {
    inner: Vec<Vec<f32>>,
    swap_history: Vec<(usize, usize)>,
}
impl NDVec {
    /// Initialize with 0 as initial value of
    pub fn new(dim: usize) -> Self {
        Self {
            inner: vec![vec![0.0; 60]; dim],
            swap_history: Vec::new(),
        }
    }

    /// Swap the axis
    pub fn swap(&mut self, a_ind: usize, b_ind: usize) {
        self.inner.swap(a_ind, b_ind);
        self.swap_history.push((a_ind, b_ind));
    }

    /// Change value in an axis at a certain position
    pub fn mutate(&mut self, axis: usize, position: usize, value: f32) {
        self.inner
            .get_mut(axis)
            .and_then(|axis| axis.get_mut(position))
            .map(|s| *s = value);
    }

    /// Get a value at an axis at a certain position
    pub fn get(&self, axis: usize, position: usize) -> Option<&f32> {
        self.inner.get(axis).and_then(|p| p.get(position))
    }

    /// Revert to get original axis order
    pub fn revert(&mut self) {
        self.swap_history
            .iter()
            .rev()
            .for_each(|(a, b)| self.inner.swap(*b, *a));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_swap_x_z_y() {
        let mut n_array = NDVec::new(3);

        n_array.mutate(0, 0, 1.0);
        n_array.mutate(1, 0, 2.0);
        n_array.mutate(2, 0, 3.0);

        n_array.swap(2, 1);

        if let Some(s) = n_array.get(0, 0) {
            assert_eq!(*s, 1.0);
        }
        if let Some(s) = n_array.get(1, 0) {
            assert_eq!(*s, 3.0);
        }
        if let Some(s) = n_array.get(2, 0) {
            assert_eq!(*s, 2.0);
        }
    }

    #[test]
    fn test_swap_x_y_z() {
        let mut n_array = NDVec::new(3);

        n_array.mutate(0, 0, 1.0);
        n_array.mutate(1, 0, 2.0);
        n_array.mutate(2, 0, 3.0);

        // [x:[1,0,..,0;60],z:[3,0,..,0;60],y:[2,0,..,0;60]]
        n_array.swap(2, 1);

        // From x,z,y to x,y,z
        // [x:[1,0,..,0;60],y:[2,0,..,0;60],z:[3,0,..,0;60]]
        n_array.swap(1, 2);

        if let Some(s) = n_array.get(0, 0) {
            assert_eq!(*s, 1.0);
        }
        if let Some(s) = n_array.get(1, 0) {
            assert_eq!(*s, 2.0);
        }
        if let Some(s) = n_array.get(2, 0) {
            assert_eq!(*s, 3.0);
        }
    }

    #[test]
    fn test_swap_z_x_y() {
        let mut n_array = NDVec::new(3);

        n_array.mutate(0, 0, 1.0);
        n_array.mutate(1, 0, 2.0);
        n_array.mutate(2, 0, 3.0);
        // [x:[1,0,..,0;60],y:[2,0,..,0;60],z:[3,0,..,0;60]]

        // From x,y,z to z,x,y
        n_array.swap(2, 1);
        n_array.swap(0, 1);

        // [z:[3,0,..,0;60],x:[1,0,..,0;60],y:[2,0,..,0;60]]

        if let Some(s) = n_array.get(0, 0) {
            assert_eq!(*s, 3.0);
        }
        if let Some(s) = n_array.get(1, 0) {
            assert_eq!(*s, 1.0);
        }
        if let Some(s) = n_array.get(2, 0) {
            assert_eq!(*s, 2.0);
        }
    }

    #[test]
    fn test_revert() {
        let mut n_array = NDVec::new(3);

        n_array.mutate(0, 0, 1.0);
        n_array.mutate(1, 0, 2.0);
        n_array.mutate(2, 0, 3.0);
        // [x:[1,0,..,0;60],y:[2,0,..,0;60],z:[3,0,..,0;60]]

        // From x,y,z to z,x,y
        n_array.swap(2, 1);
        n_array.swap(0, 1);

        // [z:[3,0,..,0;60],x:[1,0,..,0;60],y:[2,0,..,0;60]]

        if let Some(s) = n_array.get(0, 0) {
            assert_eq!(*s, 3.0);
        }
        if let Some(s) = n_array.get(1, 0) {
            assert_eq!(*s, 1.0);
        }
        if let Some(s) = n_array.get(2, 0) {
            assert_eq!(*s, 2.0);
        }

        n_array.revert();

        if let Some(s) = n_array.get(0, 0) {
            assert_eq!(*s, 1.0);
        }
        if let Some(s) = n_array.get(1, 0) {
            assert_eq!(*s, 2.0);
        }
        if let Some(s) = n_array.get(2, 0) {
            assert_eq!(*s, 3.0);
        }
    }
}
