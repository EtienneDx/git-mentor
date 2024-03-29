#[macro_export]
macro_rules! error_from {
  ($from:ty, $toType:ty, $to:tt) => {
    impl From<$from> for $toType {
      fn from(_: $from) -> Self {
        Self::$to
      }
    }
  };
}
