// Helper function to concatenate classes
pub fn cn(classes: &[impl AsRef<str>]) -> String {
  classes.iter().map(|s| s.as_ref()).collect::<Vec<&str>>().join(" ")
}
