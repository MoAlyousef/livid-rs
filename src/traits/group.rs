use crate::enums::{AlignContent, Style};
use crate::group::PARENTS;
use crate::traits::WidgetExt;

pub trait GroupExt: WidgetExt {
    fn begin(&self) {
        PARENTS.with(|p| p.borrow_mut().push(self.inner()));
    }
    fn end(&self) {
        PARENTS.with(|p| p.borrow_mut().pop());
    }
    fn add<W: WidgetExt>(&self, widget: &W) {
        self.inner().append(&widget.inner());
    }
    fn remove<W: WidgetExt>(&self, widget: &W) {
        self.inner().remove(&widget.inner());
    }
    fn set_align_content(&self, align: AlignContent) {
        self.inner().set_style(Style::AlignContent, align.to_str());
    }
    fn set_justify_content(&self, align: AlignContent) {
        self.inner()
            .set_style(Style::JustifyContent, align.to_str());
    }
}
