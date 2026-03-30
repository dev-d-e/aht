macro_rules! attribute_get {
    ($n:ident, $t:ty, $a:tt) => {
        pub(crate) fn $n(&self) -> Option<&$t> {
            if let Some(Attribute::$a(o)) = self.attribute.get(&AttrName::$a) {
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
            if !self.attribute.contains_key(&a) {
                self.attribute.insert(a.clone(), Attribute::$a($v));
            }
            if let Some(Attribute::$a(o)) = self.attribute.get_mut(&a) {
                Some(o)
            } else {
                None
            }
        }
    };
}

macro_rules! resize {
    () => {
        pub(crate) fn resize(&mut self, c: &mut LayoutCoord, cx: &mut PageContext) {
            if let Some(e) = cx.get(self.element) {
                self.rect.get_attr(&e, c)
            }
        }
    };
}

macro_rules! right_bottom {
    () => {
        pub(crate) fn right_bottom(&self, cx: &mut PageContext) -> Option<Coord2D> {
            if self.rect.is_empty() {
                return None;
            }
            if let Some(a) = cx.get(self.element).and_then(|e| e.disabled()) {
                if *a {
                    return None;
                }
            }
            Some(self.rect.right_bottom())
        }
    };
}

macro_rules! draw_check {
    ($self:ident, $cx:ident) => {
        if $self.rect.is_empty() {
            return;
        }
        if let Some(e) = $cx.get($self.element) {
            if let Some(a) = e.hidden() {
                if *a {
                    return;
                }
            }

            if let Some(a) = e.disabled() {
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
