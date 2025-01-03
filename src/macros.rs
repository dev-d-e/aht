macro_rules! element {
    () => {
        pub(crate) fn element(&self) -> &Arc<RwLock<Element>> {
            &self.element
        }
    };
}

macro_rules! zero {
    () => {
        ///Set zero point on rectangular coordinates.
        pub fn set_zero(&mut self, x: isize, y: isize) {
            self.zero.x = x;
            self.zero.y = y;
        }

        pub fn set_zero_xyz(&mut self, x: isize, y: isize, z: isize) {
            self.zero.x = x;
            self.zero.y = y;
            self.zero.z = z;
        }
    };
}

macro_rules! right_bottom {
    () => {
        pub(crate) fn right_bottom(&self) -> Coord2D {
            self.outside.final_position().clone()
        }
    };
}

macro_rules! range {
    ($c:expr) => {{
        let mut r = Range::new();
        r.color = $c;
        r
    }};
    ($c:expr, $x_rad:expr, $y_rad:expr) => {{
        let mut r = Range::new();
        r.color = $c;
        r.x_rad = $x_rad;
        r.y_rad = $y_rad;
        r
    }};
}
