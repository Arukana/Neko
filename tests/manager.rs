extern crate neko;

use neko::prelude::*;


#[test]
fn test_manager_new() {
  assert!(Manager::new().is_ok());
}

