use std::rc::Rc;

pub trait WithSource {
    fn get_complete_source(self: &Self) -> Rc<String>;
}
