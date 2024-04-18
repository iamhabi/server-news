pub mod scrap;

pub fn loop_scrap() {
    std::thread::spawn(|| {
        let delay_time = std::time::Duration::from_millis(1000 * 60 * 60);
  
        loop {
            scrap();
    
            std::thread::sleep(delay_time);
        }
    });
}

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