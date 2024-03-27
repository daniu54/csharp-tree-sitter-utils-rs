use std::rc::Rc;

pub trait WithSource<'s> {
    fn get_complete_source(self: &Self) -> Rc<&'s str>;
}
