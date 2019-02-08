pub fn grep_line(line: &str, pattern: &str) -> bool {
    line.contains(pattern)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
