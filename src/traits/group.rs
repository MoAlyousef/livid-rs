use crate::group::PARENTS;
use crate::traits::WidgetExt;

pub trait GroupExt: WidgetExt {
    fn begin(&self) {
        unsafe {
            PARENTS.push(self.inner());
        }
    }
    fn end(&self) {
        unsafe {
            PARENTS.pop();
        }
    }
    fn add<W: WidgetExt>(&self, widget: &W) {
        self.inner().append(&widget.inner());
    }
    fn remove<W: WidgetExt>(&self, widget: &W) {
        self.inner().remove(&widget.inner());
    }
}
