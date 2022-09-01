#[cfg(test)]
mod tests {
    #[test]
    fn test_get_inodenums() {
        let inodenums = crate::get_inodenums(&[1,2,3]);
        println!("{:?}", inodenums);
    }
}
