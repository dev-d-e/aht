macro_rules! get_set {
    ($field:ident, $set:ident, $t:ty) => {
        ///Returns a reference.
        pub fn $field(&self) -> &$t {
            &self.$field
        }

        pub fn $set(&mut self, s: $t) {
            self.$field = s;
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

macro_rules! id_class {
    () => {
        get_set!(id, set_id, String);

        get_set!(class, set_class, String);
    };
}

macro_rules! text {
    () => {
        get_set!(text, set_text, String);
    };
}

macro_rules! subset {
    () => {
        pub fn subset(&self) -> &VecDeque<TypeEntity> {
            &self.subset
        }

        pub fn push_subset(&mut self, e: TypeEntity) {
            self.subset.push_back(e);
        }
    };
}

macro_rules! tip {
    () => {
        get_set!(tip, set_tip, String);
    };
}

macro_rules! hidden {
    () => {
        get_set!(hidden, set_hidden, bool);
    };
}

macro_rules! name {
    () => {
        get_set!(name, set_name, String);
    };
}

macro_rules! value {
    () => {
        get_set!(value, set_value, String);
    };
}

macro_rules! readonly {
    () => {
        get_set!(readonly, set_readonly, bool);
    };
}

macro_rules! disabled {
    () => {
        get_set!(disabled, set_disabled, bool);
    };
}

macro_rules! required {
    () => {
        get_set!(required, set_required, bool);
    };
}

macro_rules! column_row {
    () => {
        get_set!(column, set_column, i32);

        get_set!(row, set_row, i32);
    };
}

macro_rules! xpoints_ypoints {
    () => {
        get_set!(xpoints, set_xpoints, Vec<i32>);

        get_set!(ypoints, set_ypoints, Vec<i32>);
    };
}

macro_rules! range_background {
    () => {
        ///Set point on rectangular coordinates.
        pub fn set_xy(&mut self, x: i32, y: i32) {
            self.range.left = x;
            self.range.top = y;
        }

        ///Set width and height.
        pub fn set_side(&mut self, width: i32, height: i32) {
            self.range.right = self.range.left.saturating_add(width);
            self.range.bottom = self.range.top.saturating_add(height);
        }

        ///Returns a reference.
        pub fn range(&self) -> &IRect {
            &self.range
        }

        ///Returns a mutable reference.
        pub fn mut_range(&mut self) -> &mut IRect {
            &mut self.range
        }

        pub(crate) fn to_rect(&self) -> Rect {
            Rect::from_irect(self.range)
        }

        get_set!(background, set_background, Color);
    };
}

macro_rules! align {
    () => {
        get_set!(horizontal_align, set_horizontal_align, HorizontalAlign);

        get_set!(vertical_align, set_vertical_align, VerticalAlign);
    };
}

macro_rules! font_color {
    () => {
        get_set!(font_color, set_font_color, Color);
    };
}

macro_rules! shape_background_border {
    () => {
        get_set!(shape, set_shape, Shape);

        get_set!(shape_background, set_shape_background, Color);

        get_set!(border_width, set_border_width, i32);

        get_set!(border_color, set_border_color, Color);
    };
}

macro_rules! cursor_x {
    () => {
        get_set!(cursor_x, set_cursor_x, i32);

        get_set!(cursor_width, set_cursor_width, i32);
    };
}

macro_rules! cursor_y {
    () => {
        get_set!(cursor_y, set_cursor_y, i32);

        get_set!(cursor_height, set_cursor_height, i32);
    };
}

macro_rules! href {
    () => {
        get_set!(href, set_href, String);
    };
}

macro_rules! multiple {
    () => {
        get_set!(multiple, set_multiple, bool);
    };
}

macro_rules! selected {
    () => {
        get_set!(selected, set_selected, bool);
    };
}

//
//
macro_rules! to_type {
    ($o:ident, $t:ty, $s:ident, $attr:ident, $subset:ident $(, $pattern:pat_param, $rst:expr)*) => {
        let mut $o = <$t>::new();
        $o.set_text($s);
        for attr in $attr.drain(..) {
            $o.attr(attr);
        }
        for sub in $subset.drain(..) {
            $o.push_subset(TypeEntity::from(sub));
        }
    };
}
