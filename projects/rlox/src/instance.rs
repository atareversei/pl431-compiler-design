use crate::class::Class;

pub struct Instance {
    class: Class,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        Self { class }
    }
}
