use crate::traits::IsElementIterable;
use crate::traits::widget_base::WidgetBase;
use crate::enums::{AlignContent, Style};
use crate::traits::WidgetExt;

pub trait GroupExt: WidgetExt {
    fn begin(&self) {
        crate::group::Group::group_begin(&self.inner());
    }
    fn end(&self) {
        crate::group::Group::group_end();
    }
    fn clear(&self) {
        self.inner().set_inner_html("");
    }
    fn add<W: WidgetExt>(&self, widget: &W) where Self: Sized {
        self.inner().append(&widget.inner());
    }
    fn remove<W: WidgetExt>(&self, widget: &W) where Self: Sized {
        self.inner().remove(&widget.inner());
    }
    fn set_align_content(&self, align: AlignContent) {
        self.inner().set_style(Style::AlignContent, align.to_str());
    }
    fn set_justify_content(&self, align: AlignContent) {
        self.inner()
            .set_style(Style::JustifyContent, align.to_str());
    }
    fn children(&self) -> Vec<Box<dyn WidgetExt>> {
        let mut v: Vec<Box<dyn WidgetExt>> = vec![];
        let c = self.inner().children();
        for e in c.iter() {
            let f = unsafe {crate::frame::Label::from_widget(&e)};
            v.push(Box::new(f));
        }
        v
    }
}
