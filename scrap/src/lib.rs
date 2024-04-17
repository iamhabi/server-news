pub mod scrap;

pub fn scrap() {
    scrap::scrap_news();
}

#[cfg(test)]
mod tests {
    use crate::scrap;

    #[test]
    fn test_scrap() {
        scrap();
    }
}