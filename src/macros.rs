macro_rules! element {
    ($n:ident) => {
        pub fn element(&self) -> &str {
            $n
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

macro_rules! set_parent {
    () => {
        set_parent!(TypeEntity, TypeEntity);
    };
    ($t:ty) => {
        pub(crate) fn set_parent(&mut self, self_ptr: &mut $t) {
            self.subset.set_parent(self_ptr);
        }
    };
    ($t:ty, $s:ty) => {
        pub(crate) fn set_parent(&mut self, parent_ptr: &mut $t, self_ptr: &mut $s) {
            self.parent = parent_ptr;
            self.subset.set_parent(self_ptr);
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
