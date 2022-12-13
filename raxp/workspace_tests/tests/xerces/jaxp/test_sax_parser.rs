

fn create_test_file(name: String) -> NamedTempFile {
    let mut tmp = Builder::new().prefix(&name).suffix(".xml").tempfile().unwrap();
    tmp.write("<?xml version=\"1.0\"?><test a1=\"x\" a2=\"y\"/>".as_bytes());
    tmp
}