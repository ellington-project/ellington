pub fn trueish() -> bool{ 
    true 
}

#[cfg(test)]
mod tests {
    #[test]
    fn tautology_internal() {
        assert!(super::trueish());
    }
}
