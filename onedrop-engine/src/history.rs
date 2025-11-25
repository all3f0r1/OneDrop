//! History management for presets, mash-ups, and colors.

use std::collections::VecDeque;

/// Generic history structure with back/forward navigation.
#[derive(Debug, Clone)]
pub struct History<T> {
    /// Items in history
    items: VecDeque<T>,
    
    /// Current index in history
    current_index: Option<usize>,
    
    /// Maximum size of history
    max_size: usize,
}

impl<T: Clone> History<T> {
    /// Create a new history with specified maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(max_size),
            current_index: None,
            max_size,
        }
    }
    
    /// Push a new item to history.
    pub fn push(&mut self, item: T) {
        // If we're not at the end, remove all items after current
        if let Some(idx) = self.current_index {
            while self.items.len() > idx + 1 {
                self.items.pop_back();
            }
        }
        
        // Add new item
        self.items.push_back(item);
        
        // Trim if exceeds max size
        while self.items.len() > self.max_size {
            self.items.pop_front();
        }
        
        // Update current index
        self.current_index = if self.items.is_empty() {
            None
        } else {
            Some(self.items.len() - 1)
        };
    }
    
    /// Go back to previous item.
    pub fn back(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }
        
        match self.current_index {
            None => {
                // Start at the end
                self.current_index = Some(self.items.len() - 1);
                self.items.back()
            }
            Some(0) => {
                // Already at the beginning
                self.items.front()
            }
            Some(idx) => {
                // Go back one
                self.current_index = Some(idx - 1);
                self.items.get(idx - 1)
            }
        }
    }
    
    /// Go forward to next item.
    pub fn forward(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }
        
        match self.current_index {
            None => None,
            Some(idx) if idx >= self.items.len() - 1 => {
                // Already at the end
                self.items.back()
            }
            Some(idx) => {
                // Go forward one
                self.current_index = Some(idx + 1);
                self.items.get(idx + 1)
            }
        }
    }
    
    /// Get current item.
    pub fn current(&self) -> Option<&T> {
        self.current_index.and_then(|idx| self.items.get(idx))
    }
    
    /// Check if can go back.
    pub fn can_go_back(&self) -> bool {
        match self.current_index {
            None => !self.items.is_empty(),
            Some(0) => false,
            Some(_) => true,
        }
    }
    
    /// Check if can go forward.
    pub fn can_go_forward(&self) -> bool {
        match self.current_index {
            None => false,
            Some(idx) => idx < self.items.len() - 1,
        }
    }
    
    /// Get the number of items in history.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    
    /// Check if history is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    /// Clear all history.
    pub fn clear(&mut self) {
        self.items.clear();
        self.current_index = None;
    }
}

impl<T: Clone> Default for History<T> {
    fn default() -> Self {
        Self::new(100) // Default max size
    }
}

/// State for mash-up operations.
#[derive(Debug, Clone, PartialEq)]
pub struct MashUpState {
    /// Source preset names
    pub source_presets: Vec<String>,
    
    /// Mash-up type
    pub mash_type: MashUpType,
    
    /// Timestamp when created
    pub timestamp: std::time::SystemTime,
}

/// Type of mash-up operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MashUpType {
    /// Regular mash-up (warp + comp)
    Regular,
    
    /// Deep mash-up (all 5 bins)
    Deep,
}

/// State for color randomization.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorState {
    /// Color values (RGB)
    pub colors: Vec<[f32; 3]>,
    
    /// Timestamp when created
    pub timestamp: std::time::SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_push() {
        let mut history = History::new(5);
        
        history.push(1);
        history.push(2);
        history.push(3);
        
        assert_eq!(history.len(), 3);
        assert_eq!(history.current(), Some(&3));
    }

    #[test]
    fn test_history_back() {
        let mut history = History::new(5);
        
        history.push(1);
        history.push(2);
        history.push(3);
        
        assert_eq!(history.back(), Some(&2));
        assert_eq!(history.back(), Some(&1));
        assert_eq!(history.back(), Some(&1)); // Can't go before first
    }

    #[test]
    fn test_history_forward() {
        let mut history = History::new(5);
        
        history.push(1);
        history.push(2);
        history.push(3);
        
        history.back();
        history.back();
        
        assert_eq!(history.forward(), Some(&2));
        assert_eq!(history.forward(), Some(&3));
        assert_eq!(history.forward(), Some(&3)); // Can't go past last
    }

    #[test]
    fn test_history_max_size() {
        let mut history = History::new(3);
        
        history.push(1);
        history.push(2);
        history.push(3);
        history.push(4);
        history.push(5);
        
        assert_eq!(history.len(), 3);
        assert_eq!(history.items[0], 3);
        assert_eq!(history.items[1], 4);
        assert_eq!(history.items[2], 5);
    }

    #[test]
    fn test_history_can_navigate() {
        let mut history = History::new(5);
        
        history.push(1);
        history.push(2);
        history.push(3);
        
        assert!(!history.can_go_forward());
        assert!(history.can_go_back());
        
        history.back();
        
        assert!(history.can_go_forward());
        assert!(history.can_go_back());
        
        history.back();
        
        assert!(history.can_go_forward());
        assert!(!history.can_go_back());
    }

    #[test]
    fn test_history_clear() {
        let mut history = History::new(5);
        
        history.push(1);
        history.push(2);
        history.push(3);
        
        history.clear();
        
        assert!(history.is_empty());
        assert_eq!(history.current(), None);
    }
}
