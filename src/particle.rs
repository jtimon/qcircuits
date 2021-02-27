//! A particle can be observed by a Filter or counted by a Detector.

/// A particle can be observed by a Filter or counted by a Detector.
pub trait Particle {
    // if up returns true, false otherwise
    fn observe_updown(&mut self) -> bool;
    // if left returns true, false otherwise
    fn observe_leftright(&mut self) -> bool;
}
