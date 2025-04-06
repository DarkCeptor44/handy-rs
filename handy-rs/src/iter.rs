use std::fmt::Display;

/// Trait to convert a vector of `T` into a vector of `&T` or `&mut T`
pub trait IntoRefVec<'a, T> {
    /// Converts a vector of `T` to a vector of `&mut T`
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::iter::IntoRefVec;
    ///
    /// let mut x = vec![1, 2, 3];
    /// let y = x.as_mut_ref_vec();
    /// assert_eq!(y, vec![&mut 1, &mut 2, &mut 3]);
    /// ```
    fn as_mut_ref_vec(&'a mut self) -> Vec<&'a mut T>;

    /// Converts a vector of `T` to a vector of `&T`
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::iter::IntoRefVec;
    ///
    /// let x = vec![1, 2, 3];
    /// let y = x.as_ref_vec();
    /// assert_eq!(y, vec![&1, &2, &3]);
    /// ```
    fn as_ref_vec(&'a self) -> Vec<&'a T>;
}

impl<'a, T> IntoRefVec<'a, T> for Option<Vec<T>> {
    fn as_mut_ref_vec(&'a mut self) -> Vec<&'a mut T> {
        self.iter_mut().flat_map(|v| v.iter_mut()).collect()
    }

    fn as_ref_vec(&'a self) -> Vec<&'a T> {
        self.iter().flat_map(|v| v.iter()).collect()
    }
}

impl<'a, T> IntoRefVec<'a, T> for Vec<T> {
    fn as_mut_ref_vec(&'a mut self) -> Vec<&'a mut T> {
        self.iter_mut().collect()
    }

    fn as_ref_vec(&'a self) -> Vec<&'a T> {
        self.iter().collect()
    }
}

impl<'a, T> IntoRefVec<'a, T> for [T] {
    fn as_mut_ref_vec(&'a mut self) -> Vec<&'a mut T> {
        self.iter_mut().collect()
    }

    fn as_ref_vec(&'a self) -> Vec<&'a T> {
        self.iter().collect()
    }
}

/// Trait for converting an iterable of items that can be displayed into a vector of strings
pub trait StringIterable {
    /// Converts the iterable to a vector of strings.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::iter::StringIterable;
    ///
    /// let x = vec![1, 2, 3];
    /// let y = x.to_string_vec();
    /// assert_eq!(y, vec!["1", "2", "3"]);
    /// ```
    fn to_string_vec(&self) -> Vec<String>;
}

impl<T> StringIterable for Option<Vec<T>>
where
    T: Display,
{
    fn to_string_vec(&self) -> Vec<String> {
        self.iter()
            .flat_map(|v| v.iter().map(ToString::to_string))
            .collect()
    }
}

impl<T> StringIterable for [T]
where
    T: Display,
{
    fn to_string_vec(&self) -> Vec<String> {
        self.iter().map(ToString::to_string).collect()
    }
}

impl<T> StringIterable for Vec<T>
where
    T: Display,
{
    fn to_string_vec(&self) -> Vec<String> {
        self.iter().map(ToString::to_string).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_ref_vec() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.as_ref_vec(), vec![&1, &2, &3, &4, &5]);

        let v2 = Some(v);
        assert_eq!(v2.as_ref_vec(), vec![&1, &2, &3, &4, &5]);

        let mut v3 = vec![1, 2, 3, 4, 5];
        assert_eq!(
            v3.as_mut_ref_vec(),
            vec![&mut 1, &mut 2, &mut 3, &mut 4, &mut 5]
        );

        let mut v4 = Some(v3);
        assert_eq!(
            v4.as_mut_ref_vec(),
            vec![&mut 1, &mut 2, &mut 3, &mut 4, &mut 5]
        );
    }

    #[test]
    fn test_string_iterable() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.to_string_vec(), vec!["1", "2", "3", "4", "5"]);

        let v2 = [1, 2, 3, 4, 5];
        assert_eq!(v2.to_string_vec(), vec!["1", "2", "3", "4", "5"]);

        let v3 = Some(v);
        assert_eq!(v3.to_string_vec(), vec!["1", "2", "3", "4", "5"]);

        let v4: Option<Vec<i32>> = None;
        assert_eq!(v4.to_string_vec(), Vec::<String>::new());
    }
}
