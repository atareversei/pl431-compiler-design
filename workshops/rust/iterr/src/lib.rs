fn flatten<O>(iter: O) -> Flatten<O::IntoIter>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    next_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(outer: O) -> Self {
        Self {
            outer,
            next_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut next_iter) = self.next_iter {
                if let Some(i) = next_iter.next() {
                    return Some(i);
                }
                self.next_iter = None;
            }

            if let Some(next_inner) = self.outer.next() {
                self.next_iter = Some(next_inner.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: Iterator + DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }

            if let Some(back_inner) = self.outer.next_back() {
                self.back_iter = Some(back_inner.into_iter());
            } else {
                return self.next_iter.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2)
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2)
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a", "b"], vec!["c", "d"]]);
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next_back(), Some("d"));
        assert_eq!(iter.next(), Some("b"));
        assert_eq!(iter.next_back(), Some("c"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
