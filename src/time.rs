//! 時間関連の構成要素。
use std::time::Duration;

/// 秒単位の時間尺を表すための構造体.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Seconds(pub u64);
impl From<Duration> for Seconds {
    fn from(f: Duration) -> Self {
        Seconds(f.as_secs())
    }
}
impl From<Seconds> for Duration {
    fn from(f: Seconds) -> Self {
        Duration::from_secs(f.0)
    }
}

/// Defines predefined constants for ease of use.
pub mod consts {
    use super::*;

    /// This value equals to Seconds(0).
    ///
    /// ```rust
    /// assert_eq!(ZERO_SECONDS.0, 0)
    /// ```
    pub const ZERO_SECONDS: Seconds = Seconds(0);

    /// This value equals to Seconds(1).
    ///
    /// ```rust
    /// assert_eq!(ONE_SECONDS.0, 1)
    /// ```
    pub const ONE_SECONDS: Seconds = Seconds(1);
}
