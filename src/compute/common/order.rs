//! Sorting flex and grid items into order-modified document order
use crate::util::sys::Vec;

/// The number of distinct `order` values up to which items are bucketed rather than sorted
const MAX_BUCKETED_ORDER_VALUES: usize = 16;

/// Returns the indexes `0..n` of the given `order` values arranged in order-modified document order:
/// ascending by `order`, with ties kept in their original (document) order.
///
/// Containers rarely use more than a handful of distinct `order` values, so (like WebKit's
/// `OrderIterator`) the distinct values are collected and the items are scanned once per value.
/// That is linear and inherently stable. Containers with many distinct values fall back to a
/// comparison sort of `(order, index)` keys.
pub(crate) fn order_modified_permutation(orders: impl Iterator<Item = i32>) -> Vec<u32> {
    let orders: Vec<i32> = orders.collect();

    let mut distinct = [0i32; MAX_BUCKETED_ORDER_VALUES];
    let mut distinct_count = 0;
    for &order in &orders {
        if let Err(position) = distinct[..distinct_count].binary_search(&order) {
            if distinct_count == MAX_BUCKETED_ORDER_VALUES {
                return sorted_permutation(&orders);
            }
            distinct.copy_within(position..distinct_count, position + 1);
            distinct[position] = order;
            distinct_count += 1;
        }
    }

    let mut permutation = Vec::with_capacity(orders.len());
    for &value in &distinct[..distinct_count] {
        permutation
            .extend(orders.iter().enumerate().filter(|&(_, &order)| order == value).map(|(index, _)| index as u32));
    }
    permutation
}

/// Fallback for containers with many distinct `order` values: sort `(order, index)` keys.
fn sorted_permutation(orders: &[i32]) -> Vec<u32> {
    let mut keys: Vec<(i32, u32)> = orders.iter().enumerate().map(|(index, &order)| (order, index as u32)).collect();
    keys.sort_unstable();
    keys.iter().map(|&(_, index)| index).collect()
}

/// Reorder `items` in place such that `items[k]` becomes the item previously at `permutation[k]`.
/// Each item is moved at most once (by following the permutation's cycles). `permutation` is
/// consumed as scratch space.
pub(crate) fn apply_permutation<T>(items: &mut [T], permutation: &mut [u32]) {
    debug_assert_eq!(items.len(), permutation.len());
    for start in 0..items.len() {
        let mut position = start;
        loop {
            let source = permutation[position] as usize;
            permutation[position] = position as u32;
            if source == start {
                break;
            }
            items.swap(position, source);
            position = source;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_permutation, order_modified_permutation, MAX_BUCKETED_ORDER_VALUES};

    fn expected_permutation(orders: &[i32]) -> Vec<u32> {
        let mut indexes: Vec<u32> = (0..orders.len() as u32).collect();
        indexes.sort_by_key(|&index| orders[index as usize]);
        indexes
    }

    #[test]
    fn order_modified_permutation_matches_stable_sort() {
        let cases: Vec<Vec<i32>> = vec![
            vec![],
            vec![0],
            vec![0, 0, 0],
            vec![2, -1, 0, 3, -2, 2, -1, 0, 3, -2, 2],
            vec![i32::MAX, i32::MIN, 0, 0, i32::MIN],
            (0..100).map(|i| (i * 7919) % 5 - 2).collect(),
            // More distinct values than the bucketing limit (exercises the sort fallback)
            (0..100).map(|i| (i * 7919) % (MAX_BUCKETED_ORDER_VALUES as i32 * 3) - 20).collect(),
            (0..(MAX_BUCKETED_ORDER_VALUES as i32 + 1)).rev().collect(),
        ];
        for orders in cases {
            assert_eq!(order_modified_permutation(orders.iter().copied()), expected_permutation(&orders), "{orders:?}");
        }
    }

    #[test]
    fn apply_permutation_reorders_items() {
        let items = ["a", "b", "c", "d", "e", "f"];
        for permutation in
            [[0u32, 1, 2, 3, 4, 5], [5, 4, 3, 2, 1, 0], [1, 0, 3, 2, 5, 4], [2, 0, 1, 5, 3, 4], [3, 4, 5, 0, 1, 2]]
        {
            let mut reordered = items;
            let mut scratch: Vec<u32> = permutation.to_vec();
            apply_permutation(&mut reordered, &mut scratch);
            let expected: Vec<&str> = permutation.iter().map(|&index| items[index as usize]).collect();
            assert_eq!(reordered.to_vec(), expected);
        }
    }
}
