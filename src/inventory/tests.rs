#[cfg(test)]
mod tests {
    use inventory::get_hosts;

    #[test]
    fn json_file_load_returns_ok() {
        assert!(get_hosts().is_ok() && !get_hosts().is_err());
    }

    #[test]
    fn json_file_load_returns_vec() {
        assert!(get_hosts().is_ok() && !get_hosts().is_err());
    }
}