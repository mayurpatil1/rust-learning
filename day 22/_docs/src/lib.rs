//! # Loot
//!
//! A tiny library for valuing a pile of loot items.
//! This `//!` comment documents the whole crate (it appears on the front
//! page of `cargo doc`). Note the `!` — it documents the thing it's INSIDE.

/// A single item with a name and a gold value.
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub value: u32,
}

impl Item {
    /// Creates a new [`Item`].
    ///
    /// # Examples
    ///
    /// ```
    /// use _docs::Item;
    ///
    /// let sword = Item::new("sword", 150);
    /// assert_eq!(sword.value, 150);
    /// ```
    pub fn new(name: &str, value: u32) -> Item {
        Item {
            name: String::from(name),
            value,
        }
    }
}

/// Returns the total gold value of all items.
///
/// # Examples
///
/// ```
/// use _docs::{Item, total_value};
///
/// let loot = vec![
///     Item::new("gem", 300),
///     Item::new("coin", 50),
/// ];
/// assert_eq!(total_value(&loot), 350);
/// ```
///
/// An empty pile is worth nothing:
///
/// ```
/// use _docs::total_value;
///
/// assert_eq!(total_value(&[]), 0);
/// ```
pub fn total_value(items: &[Item]) -> u32 {
    items.iter().map(|item| item.value).sum()
}

/// Returns the single most valuable item, or `None` if the slice is empty.
///
/// # Examples
///
/// ```
/// use _docs::{Item, most_valuable};
///
/// let loot = vec![Item::new("ring", 80), Item::new("crown", 500)];
/// assert_eq!(most_valuable(&loot).unwrap().name, "crown");
/// ```
pub fn most_valuable(items: &[Item]) -> Option<&Item> {
    items.iter().max_by_key(|item| item.value)
}

// Regular UNIT tests (Day 13) — internal, use `//` not `///`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_of_empty_is_zero() {
        assert_eq!(total_value(&[]), 0);
    }

    #[test]
    fn total_sums_values() {
        let loot = vec![Item::new("a", 10), Item::new("b", 25)];
        assert_eq!(total_value(&loot), 35);
    }

    #[test]
    fn most_valuable_of_empty_is_none() {
        assert!(most_valuable(&[]).is_none());
    }
}