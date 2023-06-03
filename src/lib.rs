pub trait Grid {
    // the grid is a 2D array of cells
    /*
    Vec< - this vector contains the horizontal rows
        Vec< - this vector contains the cells within a row
            Cell
        >
    >
    Example:
    [
        [Cell, Cell, Cell],
        [Cell, Cell, Cell],
        [Cell, Cell, Cell]
    ]
    upper left corner is [0][0]
    lower left corner is [height-1][0]
    higher first index is lower row, known as y
    higher second index is righter cell, known as x
    */
    ///
    type Field: Clone + Default;

    fn get_grid(&self) -> Vec<Vec<Self::Field>>;
    fn get_grid_ref(&self) -> &Vec<Vec<Self::Field>>;
    fn get_grid_mut(&mut self) -> &mut Vec<Vec<Self::Field>>;
    fn set_grid(&mut self, grid: Vec<Vec<Self::Field>>);

    fn get_width(&self) -> usize {
        self.get_grid_ref().get(0).unwrap().len()
    }
    fn get_height(&self) -> usize {
        self.get_grid_ref().len()
    }

    fn get_field_ref(&self, y: usize, x: usize) -> Option<&Self::Field> {
        self.get_grid_ref().get(y).and_then(|row| row.get(x))
    }
    fn get_field_mut(&mut self, y: usize, x: usize) -> Option<&mut Self::Field> {
        self.get_grid_mut()
            .get_mut(y)
            .and_then(|row| row.get_mut(x))
    }
    fn get_field_box(&self, y: usize, x: usize) -> Option<Box<Self::Field>>;

    fn set_field(&mut self, y: usize, x: usize, field: Self::Field) {
        self.get_grid_mut()[y][x] = field;
    }

    fn in_bounds(&self, y: usize, x: usize) -> bool {
        x < self.get_width() && y < self.get_height()
    }

    fn neighbors_indices(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let x = x as isize + i;
                let y = y as isize + j;

                if x < 0 || y < 0 {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;

                if self.in_bounds(y, x) {
                    neighbors.push((y, x));
                }
            }
        }

        neighbors
    }
    fn neighbors_ref(&self, y: usize, x: usize) -> Vec<&Self::Field> {
        self.neighbors_indices(y, x)
            .iter()
            .map(|(i, j)| self.get_field_ref(*i, *j))
            .filter(|field| field.is_some())
            .map(|field| field.unwrap())
            .collect()
    }

    /// This function is used to find the diagonals branching from a cell.
    /// A possible use case would be the bishop in chess.
    /// It returns a vector of vectors, where each vector contains a diagonal.
    /// The first element is the diagonal towards the upper left corner.
    /// The second element is the diagonal towards the upper right corner.
    /// The third element is the diagonal towards the lower left corner.
    /// The fourth element is the diagonal towards the lower right corner.
    ///
    fn diagonals_indices(&self, y: usize, x: usize) -> Vec<Vec<(usize, usize)>> {
        let mut diagonals = Vec::new();

        let mut up_left = Vec::new();
        let mut up_right = Vec::new();
        let mut down_left = Vec::new();
        let mut down_right = Vec::new();

        let mut x_clone = x;
        let mut y_clone = y;
        while x_clone > 0 && y_clone > 0 {
            x_clone -= 1;
            y_clone -= 1;
            let target = (y_clone, x_clone);
            if self.in_bounds(target.0, target.1) {
                up_left.push(target);
            }
        }
        x_clone = x;
        y_clone = y;
        while x_clone < self.get_width() - 1 && y_clone > 0 {
            x_clone += 1;
            y_clone -= 1;
            let target = (y_clone, x_clone);
            if self.in_bounds(target.0, target.1) {
                up_right.push(target);
            }
        }
        x_clone = x;
        y_clone = y;
        while x_clone > 0 && y_clone < self.get_height() - 1 {
            x_clone -= 1;
            y_clone += 1;
            let target = (y_clone, x_clone);
            if self.in_bounds(target.0, target.1) {
                down_left.push(target);
            }
        }
        x_clone = x;
        y_clone = y;
        while x_clone < self.get_width() - 1 && y_clone < self.get_height() - 1 {
            x_clone += 1;
            y_clone += 1;
            let target = (y_clone, x_clone);
            if self.in_bounds(target.0, target.1) {
                down_right.push(target);
            }
        }

        diagonals.push(up_left);
        diagonals.push(up_right);
        diagonals.push(down_left);
        diagonals.push(down_right);
        diagonals
    }
    fn diagonals_ref(&self, y: usize, x: usize) -> Vec<Vec<&Self::Field>> {
        self.diagonals_indices(y, x)
            .iter()
            .map(|diagonal| {
                diagonal
                    .iter()
                    .map(|(y, x)| self.get_field_ref(*y, *x))
                    .filter(|field| field.is_some())
                    .map(|field| field.unwrap())
                    .collect()
            })
            .collect()
    }

    fn straight_indices(&self, y: usize, x: usize) -> Vec<Vec<(usize, usize)>> {
        let mut straight = Vec::new();

        let mut up = Vec::new();
        let mut down = Vec::new();
        let mut left = Vec::new();
        let mut right = Vec::new();

        for i in 0..x {
            left.push((y, x - i - 1));
        }
        for i in x + 1..self.get_width() {
            right.push((y, i));
        }
        for i in 0..y {
            up.push((y - i - 1, x));
        }
        for i in y + 1..self.get_height() {
            down.push((i, x));
        }

        straight.push(up);
        straight.push(down);
        straight.push(left);
        straight.push(right);

        straight
    }
    fn straight_ref(&self, y: usize, x: usize) -> Vec<Vec<&Self::Field>> {
        self.straight_indices(y, x)
            .iter()
            .map(|straight| {
                straight
                    .iter()
                    .map(|(y, x)| self.get_field_ref(*y, *x))
                    .filter(|field| field.is_some())
                    .map(|field| field.unwrap())
                    .collect()
            })
            .collect()
    }

    // Using this function, a knight-like movement can be implemented.
    // The offset is a tuple of two numbers, where the first number is the first offset and the second number is the second offset.
    // Calling this function with (1,2) will return the indices of the fields that a knight can move to.
    // The same goes for (2,1).
    fn knight_like_indices(
        &self,
        y: usize,
        x: usize,
        offset: (isize, isize),
    ) -> Vec<(usize, usize)> {
        let mut knight_like = Vec::new();

        let x_clone = x as isize;
        let y_clone = y as isize;

        let target = ((y_clone + offset.1) as usize, (x_clone + offset.0) as usize);
        if self.in_bounds(target.0, target.1) {
            knight_like.push(target);
        }

        let target = ((y_clone + offset.1) as usize, (x_clone - offset.0) as usize);
        if self.in_bounds(target.0, target.1) {
            knight_like.push(target);
        }

        let target = ((y_clone - offset.1) as usize, (x_clone + offset.0) as usize);
        if self.in_bounds(target.0, target.1) {
            knight_like.push(target);
        }

        let target = ((y_clone - offset.1) as usize, (x_clone - offset.0) as usize);
        if self.in_bounds(target.0, target.1) {
            knight_like.push(target);
        }

        // repeat for the second offset
        let target = ((y_clone + offset.0) as usize, (x_clone + offset.1) as usize);
        if self.in_bounds(target.0, target.1) {
            knight_like.push(target);
        }

        let target = ((y_clone + offset.0) as usize, (x_clone - offset.1) as usize);
        if self.in_bounds(target.0, target.1) {
            knight_like.push(target);
        }

        let target = ((y_clone - offset.0) as usize, (x_clone + offset.1) as usize);
        if self.in_bounds(target.0, target.1) {
            knight_like.push(target);
        }

        let target = ((y_clone - offset.0) as usize, (x_clone - offset.1) as usize);
        if self.in_bounds(target.0, target.1) {
            knight_like.push(target);
        }

        knight_like
    }
    fn shift_positive(&mut self, y: usize, x: usize, start: bool) -> Result<(), String> {
        let mut new_grid = Vec::new();
        if start {
            for _ in 0..y {
                let mut row = Vec::new();
                for _ in 0..self.get_width() {
                    row.push(<Self as Grid>::Field::default());
                }
                new_grid.push(row);
            }
        }
        for i in 0..self.get_height() {
            let mut row = Vec::new();
            for _ in 0..x {
                row.push(<Self as Grid>::Field::default());
            }
            for j in 0..self.get_width() {
                row.push(
                    self.get_field_ref(i, j)
                        .ok_or(&format!("Field not found at index {i}-{j}"))?
                        .clone(),
                );
            }
            new_grid.push(row);
        }
        if !start {
            for _ in 0..y {
                let mut row = Vec::new();
                for _ in 0..self.get_width() {
                    row.push(<Self as Grid>::Field::default());
                }
                new_grid.push(row);
            }
        }
        self.set_grid(new_grid);
        Ok(())
    }
    /// more entire grid by x, y
    /// it is essentially the same as calling shift_positive with negative values
    fn shift_negative(&mut self, y: usize, x: usize, start: bool) -> Result<(), String> {
        let mut new_grid = Vec::new();
        if start {
            for i in y..self.get_height() {
                let mut row = Vec::new();
                for j in x..self.get_width() {
                    row.push(
                        self.get_field_ref(i, j)
                            .ok_or(&format!("Field not found at index {i}-{j}"))?
                            .clone(),
                    );
                }
                new_grid.push(row);
            }
        } else {
            for i in 0..self.get_height() - y {
                let mut row = Vec::new();
                for j in 0..self.get_width() - x {
                    row.push(
                        self.get_field_ref(i, j)
                            .ok_or(&format!("Field not found at index {i}-{j}"))?
                            .clone(),
                    );
                }
                new_grid.push(row);
            }
        }
        self.set_grid(new_grid);
        Ok(())
    }
    fn shift(&mut self, y: isize, x: isize, start: bool) -> Result<(), String> {
        if y < 0 || x < 0 {
            self.shift_negative(y.abs() as usize, x.abs() as usize, start)
        } else {
            self.shift_positive(y as usize, x as usize, start)
        }
    }
}
