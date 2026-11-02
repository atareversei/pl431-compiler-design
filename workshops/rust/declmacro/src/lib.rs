#[macro_export]
macro_rules! avec {
    ($($element:expr),* $(,)?) => {{
        // New Rust versions allow metavariables to be uses like ${count}, but
        // they are not fully stable.
        // let mut vs = Vec::with_capacity(${count($element)});

        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::avec![@COUNT; $($element),*]);
        $(vs.push($element);)*
        vs
    }};

    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        // * We don't use for loops, because in a busy loop it would waste
        // * a lot of cycles on bound checking
        vs.resize($count, $element);
        vs
    }};

    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::avec![@SUBST; $element]),*])
    };

    (@SUBST; $element:expr) => {()};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vec() {
        let vec: Vec<u32> = avec![];
        assert!(vec.is_empty());
    }

    #[test]
    fn single() {
        let vec: Vec<u32> = avec![42];
        assert!(!vec.is_empty());
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 42);
    }

    #[test]
    fn double() {
        let vec: Vec<u32> = avec![42, 43];
        assert!(!vec.is_empty());
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 42);
        assert_eq!(vec[0], 43);
    }

    #[test]
    fn repetition() {
        let vec: Vec<u32> = avec![2;1];
        assert!(!vec.is_empty());
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 1);
    }
}
