// speed calc here
use crate::ecs::component::*;
use bevy::prelude::*;

#[derive(Debug)]
struct TurnOrderList<T, U> {
    //entity, speed, ..
    unit_vec: Vec<(T, U)>,
    index: usize,
}
#[allow(dead_code)]
impl<T, U> TurnOrderList<T, U>
where
    U: Ord + Copy,
{
    /// Sorts the collection based on value from lowest to highest,
    /// then transform to circular collection
    fn new_sorted(mut items: Vec<(T, U)>) -> Self {
        items.sort_by_key(|k| k.1);
        Self {
            unit_vec: items,
            index: 0,
        }
    }
    /// Return the next item in the list, if the current index is at the end
    /// of the list, return the first item instead of None
    fn next_circular(&mut self) -> &(T, U) {
        self.tick_index();
        &self.unit_vec[self.index]
    }
    /// Get current item
    fn get_current(&mut self) -> &(T, U) {
        &self.unit_vec[self.index]
    }
    /// Get item at specified index
    fn get(&mut self, index: usize) -> &(T, U) {
        &self.unit_vec[index]
    }
    fn tick_index(&mut self) {
        self.index = (self.index + 1) % self.unit_vec.len();
    }
    /// Remove the Some(indexed) element in the list, if it's before the current
    /// iterator then the index is shifted back once
    ///
    /// If no index (None) is specified then the current item is removed
    fn remove(&mut self, ind: Option<usize>) {
        let index = ind.unwrap_or(self.index);
        self.unit_vec.remove(index);
        // not shifting if index is 0 or before removing item
        if self.index >= index && self.index != 0 {
            self.index = self.index - 1 % self.unit_vec.len();
        }
    }
}

/// Query units and returns TurnOrderList
pub fn generate_turn_order(unit_q: Query<(Entity, &Speed)>) {
    debug!("DEBUG ===================================");
    let mut query: Vec<(Entity, &Speed)> = Vec::new();
    for item in unit_q.iter() {
        query.push(item);
    }
    info!("{:?}", query);
    let sorted_vec = TurnOrderList::new_sorted(query);
    info!("{:?}", sorted_vec);
}

#[cfg(test)]
mod test {
    use super::*;

    fn unsorted_vec() -> Vec<(Option<i32>, i32)> {
        vec![(None, 3), (None, 2), (None, 41), (None, 2), (None, 5)]
    }
    #[test]
    fn t_new_sorted() {
        let list = unsorted_vec();
        let turnord = TurnOrderList::new_sorted(list);
        assert_eq!(
            turnord.unit_vec,
            vec![(None, 2), (None, 2), (None, 3), (None, 5), (None, 41)]
        );
    }
    #[test]
    fn next_pass_pass() {
        let list = unsorted_vec();
        let mut turnord = TurnOrderList::new_sorted(list);
        assert_eq!(turnord.next_circular().1, 2);
        assert_eq!(turnord.next_circular().1, 3);
        assert_eq!(turnord.next_circular().1, 5);
        assert_eq!(turnord.next_circular().1, 41);
        assert_eq!(turnord.next_circular().1, 2);
    }
    #[test]
    fn next_after_delete_pass() {
        let list = unsorted_vec();
        let mut turnord = TurnOrderList::new_sorted(list);
        turnord.unit_vec.remove(3);
        assert_eq!(turnord.next_circular().1, 2);
        assert_eq!(turnord.next_circular().1, 3);
        // assert_eq!(turnord.next(), &5);
        assert_eq!(turnord.next_circular().1, 41);
        assert_eq!(turnord.next_circular().1, 2);
        assert_eq!(turnord.next_circular().1, 2);
    }
    #[test]
    fn delete_next_pass() {
        let list = unsorted_vec();
        let mut turnord = TurnOrderList::new_sorted(list); // (2) 2 3 5 41
        assert_eq!(turnord.index, 0);

        assert_eq!(turnord.next_circular().1, 2); // 2 (2) 3 5 41
        assert_eq!(turnord.index, 1);

        assert_eq!(turnord.next_circular().1, 3); // 2 2 (3) 5 41
        assert_eq!(turnord.index, 2);

        turnord.remove(Some(turnord.index)); // 2 (2) 5 41
        assert_eq!(turnord.index, 1);

        assert_eq!(turnord.next_circular().1, 5); // 2 2 (5) 41
        assert_eq!(turnord.index, 2);
        assert_eq!(turnord.next_circular().1, 41); // 2 2 5 (41)
        turnord.remove(None); // 2 2 (5)
        assert_eq!(turnord.get_current(), &(None, 5));

        assert_eq!(turnord.next_circular().1, 2); // (2) 2 5
        turnord.remove(None); // (2) 5
        println!("{:?}", turnord);
        assert_eq!(turnord.next_circular().1, 5); // 2 (5)
    }
}
