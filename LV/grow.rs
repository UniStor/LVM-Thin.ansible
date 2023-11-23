fn grow(increaseBy: i8) -> i8 {
  let lv: &str = "fedora_fedora/root"
  format!("lvextend -L+{}G {}", increaseBy, lv)
}
