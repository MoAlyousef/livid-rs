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
}
