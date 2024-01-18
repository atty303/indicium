use std::rc::Rc;
use kstring::KString;

pub type Tokenizer = Rc<Box<dyn Fn(&str) -> Vec<KString>>>;
