macro_rules! get_set {
    ($field:ident, $t:ty) => {
        ///Returns a reference.
        pub fn $field(&self) -> &$t {
            &self.$field
        }
    };
    ($field:ident, $set:ident, $t:ty) => {
        get_set!($field, $t);

        pub fn $set(&mut self, s: $t) {
            self.$field = s;
        }
    };
    ($field:ident, $mut:ident, $set:ident, $t:ty) => {
        get_set!($field, $set, $t);

        ///Returns a mutable reference.
        pub fn $mut(&mut self) -> &mut $t {
            &mut self.$field
        }
    };
}

macro_rules! element {
    ($n:ident) => {
        pub fn element(&self) -> &str {
            $n
        }
    };
}

macro_rules! subset {
    () => {
        get_set!(subset, Subset);

        pub fn push_subset(&mut self, e: TypeEntity) {
            self.subset.vec().push_back(e);
        }
    };
}

macro_rules! text {
    () => {
        get_set!(text, set_text, String);
    };
}

macro_rules! action {
    () => {
        get_set!(action, set_action, String);
    };
}

macro_rules! asynchronous {
    () => {
        get_set!(asynchronous, set_asynchronous, bool);
    };
}

macro_rules! class_id {
    () => {
        get_set!(class, set_class, String);

        get_set!(id, set_id, String);
    };
}

macro_rules! column_row {
    () => {
        get_set!(column, set_column, Points);

        get_set!(row, set_row, Points);
    };
}

macro_rules! disabled {
    () => {
        get_set!(disabled, set_disabled, bool);
    };
}

macro_rules! enctype {
    () => {
        get_set!(enctype, set_enctype, String);
    };
}

macro_rules! hidden {
    () => {
        get_set!(hidden, set_hidden, bool);
    };
}

macro_rules! href {
    () => {
        get_set!(href, set_href, String);
    };
}

macro_rules! method {
    () => {
        get_set!(method, set_method, String);
    };
}

macro_rules! multiple {
    () => {
        get_set!(multiple, set_multiple, bool);
    };
}

macro_rules! name {
    () => {
        get_set!(name, set_name, String);
    };
}

macro_rules! ordinal {
    () => {
        get_set!(ordinal, set_ordinal, Ordinal);
    };
}

macro_rules! readonly {
    () => {
        get_set!(readonly, set_readonly, bool);
    };
}

macro_rules! required {
    () => {
        get_set!(required, set_required, bool);
    };
}

macro_rules! selected {
    () => {
        get_set!(selected, set_selected, bool);
    };
}

macro_rules! src {
    () => {
        get_set!(src, set_src, String);
    };
}

macro_rules! tip {
    () => {
        get_set!(tip, set_tip, String);
    };
}

macro_rules! value {
    () => {
        get_set!(value, set_value, String);
    };
}

macro_rules! range_background {
    () => {
        ///Set point on rectangular coordinates.
        pub fn set_xy(&mut self, x: isize, y: isize) {
            self.range.x = x;
            self.range.y = y;
        }

        ///Set width.
        pub fn set_width(&mut self, width: isize) {
            self.range.width = width;
        }

        ///Set height.
        pub fn set_height(&mut self, height: isize) {
            self.range.height = height;
        }

        get_set!(range, mut_range, set_range, RectangleRange);

        get_set!(background, set_background, Color);
    };
}

macro_rules! align {
    () => {
        get_set!(horizontal_align, set_horizontal_align, HorizontalAlign);

        get_set!(vertical_align, set_vertical_align, VerticalAlign);
    };
}

macro_rules! apply_font {
    () => {
        get_set!(apply_font, mut_apply_font, set_apply_font, ApplyFont);
    };
}

macro_rules! shape_background_border {
    () => {
        get_set!(shape, set_shape, Shape);

        get_set!(shape_background, set_shape_background, Color);

        get_set!(border_width, set_border_width, isize);

        get_set!(border_color, set_border_color, Color);
    };
}

macro_rules! cursor_x {
    () => {
        get_set!(cursor_x, set_cursor_x, isize);

        get_set!(cursor_width, set_cursor_width, isize);
    };
}

macro_rules! cursor_y {
    () => {
        get_set!(cursor_y, set_cursor_y, isize);

        get_set!(cursor_height, set_cursor_height, isize);
    };
}

//
//
macro_rules! to_type {
    ($t:ty, $s:ident, $attr:ident, $subset:ident) => {{
        let mut o = <$t>::new();
        o.set_text($s);
        $attr.drain(..).for_each(|a| o.attr(a));
        $subset
            .drain(..)
            .for_each(|sub| o.push_subset(TypeEntity::from(sub)));
        o
    }};
    ($t:ty, $s:ident, $attr:ident) => {{
        let mut o = <$t>::new();
        o.set_text($s);
        $attr.drain(..).for_each(|a| o.attr(a));
        o
    }};
}

macro_rules! ordinal_xy {
    ($subset_xy:ident, $o:ident) => {
        if let Some((x, y)) = $subset_xy.next($o.ordinal()) {
            $o.set_xy(x, y);
        }
    };
}
