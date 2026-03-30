use super::*;

///"Head" represents head.
#[derive(Debug, Getters)]
pub(crate) struct Head {
    #[getset(get = "pub(crate)")]
    title: Option<ElementKey>,
}

impl Head {
    pub(crate) fn new(cx: &mut PageContext) -> Self {
        let mut title = None;
        if let Some(e) = cx.head_element() {
            for &k in e.subset() {
                if let Some(o) = cx.get(k) {
                    match o.mark_type() {
                        Mark::TITLE => {
                            title.replace(k);
                        }
                        _ => {}
                    }
                }
            }
        }
        Self { title }
    }
}
