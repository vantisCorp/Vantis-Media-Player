//! Gesture Recognition Module
//! 
//! Provides touch gesture recognition for interactive elements.

use iced::Point;
use std::time::{Duration, Instant};

/// Gesture recognizer
#[derive(Debug, Clone)]
pub struct GestureRecognizer {
    /// Minimum distance for swipe (in pixels)
    pub min_swipe_distance: f32,
    /// Maximum time for tap (in milliseconds)
    pub max_tap_duration: Duration,
    /// Maximum time for double tap
    pub max_double_tap_interval: Duration,
    /// Minimum scale change for pinch
    pub min_pinch_scale: f32,
    /// Touch history for gesture detection
    touch_history: Vec<TouchPoint>,
    /// Last tap time for double tap detection
    last_tap_time: Option<Instant>,
    /// Current gesture being tracked
    current_gesture: Option<Gesture>,
}

impl Default for GestureRecognizer {
    fn default() -> Self {
        Self {
            min_swipe_distance: 50.0,
            max_tap_duration: Duration::from_millis(300),
            max_double_tap_interval: Duration::from_millis(300),
            min_pinch_scale: 0.1,
            touch_history: Vec::new(),
            last_tap_time: None,
            current_gesture: None,
        }
    }
}

impl GestureRecognizer {
    /// Create a new gesture recognizer
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Handle touch start
    pub fn touch_start(&mut self, point: Point) {
        self.touch_history.clear();
        self.touch_history.push(TouchPoint {
            position: point,
            time: Instant::now(),
        });
        self.current_gesture = Some(Gesture::Started(point));
    }
    
    /// Handle touch move
    pub fn touch_move(&mut self, point: Point) {
        if let Some(first) = self.touch_history.first() {
            self.touch_history.push(TouchPoint {
                position: point,
                time: Instant::now(),
            });
            
            let distance = first.position.distance(point);
            if distance >= self.min_swipe_distance {
                let direction = calculate_swipe_direction(first.position, point);
                self.current_gesture = Some(Gesture::Swiping(direction, point));
            }
        }
    }
    
    /// Handle touch end
    pub fn touch_end(&mut self, point: Point) -> Option<Gesture> {
        let gesture = if let Some(first) = self.touch_history.first() {
            let duration = first.time.elapsed();
            let distance = first.position.distance(point);
            
            if distance < self.min_swipe_distance && duration < self.max_tap_duration {
                // It's a tap - check for double tap
                if let Some(last_tap) = self.last_tap_time {
                    if last_tap.elapsed() < self.max_double_tap_interval {
                        self.last_tap_time = None;
                        Some(Gesture::DoubleTap(point))
                    } else {
                        self.last_tap_time = Some(Instant::now());
                        Some(Gesture::Tap(point))
                    }
                } else {
                    self.last_tap_time = Some(Instant::now());
                    Some(Gesture::Tap(point))
                }
            } else if distance >= self.min_swipe_distance {
                // It's a swipe
                let direction = calculate_swipe_direction(first.position, point);
                Some(Gesture::Swipe(direction, first.position, point))
            } else {
                // Long press
                Some(Gesture::LongPress(point))
            }
        } else {
            None
        };
        
        self.touch_history.clear();
        self.current_gesture = None;
        gesture
    }
    
    /// Get current gesture
    pub fn current_gesture(&self) -> Option<&Gesture> {
        self.current_gesture.as_ref()
    }
    
    /// Cancel current gesture
    pub fn cancel(&mut self) {
        self.touch_history.clear();
        self.current_gesture = None;
    }
}

/// Touch point for gesture tracking
#[derive(Debug, Clone)]
struct TouchPoint {
    position: Point,
    time: Instant,
}

/// Gesture types
#[derive(Debug, Clone, Copy)]
pub enum Gesture {
    /// Gesture started at point
    Started(Point),
    /// Single tap at point
    Tap(Point),
    /// Double tap at point
    DoubleTap(Point),
    /// Long press at point
    LongPress(Point),
    /// Swipe in direction, from start to end
    Swipe(SwipeDirection, Point, Point),
    /// Currently swiping
    Swiping(SwipeDirection, Point),
    /// Pinch with scale factor
    Pinch(f32, Point),
    /// Pan with delta
    Pan(Point, Point),
}

/// Swipe direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl SwipeDirection {
    /// Get arrow symbol for direction
    pub fn arrow(&self) -> &'static str {
        match self {
            SwipeDirection::Up => "↑",
            SwipeDirection::Down => "↓",
            SwipeDirection::Left => "←",
            SwipeDirection::Right => "→",
        }
    }
}

/// Calculate swipe direction from start and end points
fn calculate_swipe_direction(start: Point, end: Point) -> SwipeDirection {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    
    if dx.abs() > dy.abs() {
        if dx > 0.0 {
            SwipeDirection::Right
        } else {
            SwipeDirection::Left
        }
    } else {
        if dy > 0.0 {
            SwipeDirection::Down
        } else {
            SwipeDirection::Up
        }
    }
}

/// Multi-touch gesture handler
#[derive(Debug, Clone)]
pub struct MultiTouchHandler {
    /// Active touch points
    touches: Vec<(u64, Point)>,
    /// Initial pinch distance
    initial_pinch_distance: Option<f32>,
    /// Current scale
    scale: f32,
}

impl Default for MultiTouchHandler {
    fn default() -> Self {
        Self {
            touches: Vec::new(),
            initial_pinch_distance: None,
            scale: 1.0,
        }
    }
}

impl MultiTouchHandler {
    /// Create a new multi-touch handler
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Handle touch start
    pub fn touch_start(&mut self, id: u64, point: Point) {
        self.touches.push((id, point));
        if self.touches.len() == 2 {
            self.initial_pinch_distance = Some(self.calculate_distance());
        }
    }
    
    /// Handle touch move
    pub fn touch_move(&mut self, id: u64, point: Point) -> Option<Gesture> {
        if let Some(touch) = self.touches.iter_mut().find(|(tid, _)| *tid == id) {
            touch.1 = point;
        }
        
        if self.touches.len() == 2 {
            if let Some(initial) = self.initial_pinch_distance {
                let current = self.calculate_distance();
                self.scale = current / initial;
                
                let center = self.calculate_center();
                return Some(Gesture::Pinch(self.scale, center));
            }
        }
        
        None
    }
    
    /// Handle touch end
    pub fn touch_end(&mut self, id: u64) {
        self.touches.retain(|(tid, _)| *tid != id);
        if self.touches.len() < 2 {
            self.initial_pinch_distance = None;
        }
    }
    
    /// Get current scale
    pub fn scale(&self) -> f32 {
        self.scale
    }
    
    fn calculate_distance(&self) -> f32 {
        if self.touches.len() >= 2 {
            self.touches[0].1.distance(self.touches[1].1)
        } else {
            0.0
        }
    }
    
    fn calculate_center(&self) -> Point {
        if self.touches.is_empty() {
            return Point::ORIGIN;
        }
        
        let sum = self.touches.iter().fold(Point::ORIGIN, |acc, (_, p)| {
            Point::new(acc.x + p.x, acc.y + p.y)
        });
        
        Point::new(
            sum.x / self.touches.len() as f32,
            sum.y / self.touches.len() as f32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_swipe_direction() {
        let start = Point::new(0.0, 0.0);
        
        let right = calculate_swipe_direction(start, Point::new(100.0, 10.0));
        assert_eq!(right, SwipeDirection::Right);
        
        let left = calculate_swipe_direction(start, Point::new(-100.0, 10.0));
        assert_eq!(left, SwipeDirection::Left);
        
        let down = calculate_swipe_direction(start, Point::new(10.0, 100.0));
        assert_eq!(down, SwipeDirection::Down);
        
        let up = calculate_swipe_direction(start, Point::new(10.0, -100.0));
        assert_eq!(up, SwipeDirection::Up);
    }
    
    #[test]
    fn test_gesture_recognizer_tap() {
        let mut recognizer = GestureRecognizer::new();
        let point = Point::new(50.0, 50.0);
        
        recognizer.touch_start(point);
        
        // Simulate quick tap
        std::thread::sleep(Duration::from_millis(50));
        let gesture = recognizer.touch_end(point);
        
        assert!(matches!(gesture, Some(Gesture::Tap(_))));
    }
}