pub fn exit_status_to_code(code: i32) -> u8 {
  if !(0..=255).contains(&code) {
    return 255;
  }
  u8::try_from(code).unwrap()
}
