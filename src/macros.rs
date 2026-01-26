macro_rules! attribute_get {
    ($n:ident, $t:ty, $a:tt) => {
        pub(crate) fn $n(&self) -> Option<&$t> {
            if let Some(Attribute::$a(o)) = self.0.get(&AttrName::$a) {
                Some(o)
            } else {
                None
            }
        }
    };
}

macro_rules! attribute_get_or_insert {
    ($n:ident, $t:ty, $a:tt, $v:expr) => {
        pub(crate) fn $n(&mut self) -> Option<&mut $t> {
            let a = AttrName::$a;
            if !self.0.contains_key(&a) {
                self.0.insert(a.clone(), Attribute::$a($v));
            }
            if let Some(Attribute::$a(o)) = self.0.get_mut(&a) {
                Some(o)
            } else {
                None
            }
        }
    };
}

macro_rules! resize {
    () => {
        pub(crate) fn resize(&mut self, c: &mut LayoutCoord) {
            if let Ok(e) = self.element.read() {
                self.rect.get_attr(&e, c)
            }
        }
    };
}

macro_rules! right_bottom {
    () => {
        pub(crate) fn right_bottom(&self) -> Option<Coord2D> {
            if self.rect.is_empty() {
                return None;
            }
            if let Ok(e) = self.element().read() {
                if let Some(a) = e.attribute().disabled() {
                    if *a {
                        return None;
                    }
                }
            }
            Some(self.rect.right_bottom())
        }
    };
}

macro_rules! draw_check {
    ($self:ident) => {
        if $self.rect.is_empty() {
            return;
        }
        if let Ok(e) = $self.element().read() {
            if let Some(a) = e.attribute().hidden() {
                if *a {
                    return;
                }
            }

            if let Some(a) = e.attribute().disabled() {
                if *a {
                    return;
                }
            }
        }
    };
}

macro_rules! deref {
    ($t:ty, $target:ty, $o:tt) => {
        impl std::ops::Deref for $t {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$o
            }
        }

        impl std::ops::DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$o
            }
        }
    };
}

macro_rules! option_return {
    ($a:expr) => {
        if let Some(o) = $a {
            o
        } else {
            return;
        }
    };
    ($a:expr, $r:expr) => {
        if let Some(o) = $a {
            o
        } else {
            return $r;
        }
    };
}

macro_rules! result_return {
    ($a:expr) => {
        if let Ok(o) = $a {
            o
        } else {
            return;
        }
    };
    ($a:expr, $r:expr) => {
        if let Ok(o) = $a {
            o
        } else {
            return $r;
        }
    };
}
