use std::iter::repeat_with;

pub type Size = (usize, usize);

pub struct Grid2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid2D<T> {
    pub fn new_with<F>((width, height): Size, generator: F) -> Self
    where
        F: FnMut() -> T,
    {
        let data = repeat_with(generator).take(width * height).collect();

        Grid2D {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        self.data.get(y * self.width + x)
    }

    pub fn get_pair_mut(
        &mut self,
        (x1, y1): (usize, usize),
        (x2, y2): (usize, usize),
    ) -> Option<(&mut T, &mut T)> {
        let i1 = y1 * self.width + x1;
        let i2 = y2 * self.width + x2;

        if i1 >= self.data.len() || i2 >= self.data.len() {
            None
        } else if i1 < i2 {
            let (slice1, slice2) = self.data.split_at_mut(i2);
            Some((&mut slice1[i1], &mut slice2[0]))
        } else if i1 > i2 {
            let (slice1, slice2) = self.data.split_at_mut(i1);
            Some((&mut slice2[0], &mut slice1[i2]))
        } else {
            // Cannot return two aliasing mutable references
            None
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access() {
        let grid = Grid2D::new_with((10, 2), || 1);
        assert_eq!(grid.get((9, 1)), Some(&1));
        assert_eq!(grid.get((1, 9)), None);
    }
}
