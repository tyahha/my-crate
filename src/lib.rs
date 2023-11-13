//! # My crate
//!
//! `my_crate`は、ユーティリティの集まりであり、特定の計算をより便利に行うことができます。
/// 引数同士を足す
///
/// # Example
///
/// ```
/// let five = 5;
/// let six = 6;
///
/// assert_eq!(my_crate::add(five, six), 11);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
