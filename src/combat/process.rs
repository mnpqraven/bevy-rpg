// speed calc here
use crate::ecs::component::*;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct TurnOrderList<T, U> {
    //entity, speed, ..
    unit_vec: Vec<(T, U)>,
    index: usize,
}
#[allow(dead_code)]
impl<T, U> TurnOrderList<T, U>
where
    U: Ord + Copy,
{
    /// Creates a TurnOrderList with an empty vec and index 0
    pub fn new() -> Self {
        Self { unit_vec: Vec::new(), index: 0 }
    }
    /// Sorts the collection based on value from lowest to highest,
    /// then transform to circular collection
    pub fn new_sorted(mut items: Vec<(T, U)>) -> Self {
        items.sort_by_key(|k| k.1);
        Self {
            unit_vec: items,
            index: 0,
        }
    }
    /// Return the next item in the list, if the current index is at the end
    /// of the list, return the first item instead of None
    pub fn next(&mut self) -> &(T, U) {
        self.tick_index();
        &self.unit_vec[self.index]
    }
    /// Get current item
    pub fn get_current(&mut self) -> &(T, U) {
        &self.unit_vec[self.index]
    }
    /// Get item at specified index
    pub fn get(&mut self, index: usize) -> &(T, U) {
        &self.unit_vec[index]
    }
    fn tick_index(&mut self) {
        self.index = (self.index + 1) % self.unit_vec.len();
    }
    /// Remove the Some(indexed) element in the list, if it's before the
    /// current iterator then the index is shifted back once
    ///
    /// If no index (None) is specified then the current item is removed
    fn remove(&mut self, ind: Option<usize>) {
        let index = ind.unwrap_or(self.index);
        self.unit_vec.remove(index);
        // not shifting if index is 0 or before removing item
        if self.index >= index && self.index != 0 {
            self.index -= 1 % self.unit_vec.len();
        }
    }
    /// Same as remove, but the item in the vector is returned instead
    fn take(&mut self, ind: Option<usize>) -> (T, U) {
        let index = ind.unwrap_or(self.index);
        // not shifting if index is 0 or before removing item
        let to_pop = self.unit_vec.swap_remove(index);
        self.unit_vec.sort_by_key(|k| k.1);
        if self.index >= index && self.index != 0 {
            self.index -= 1 % self.unit_vec.len();
        }
        to_pop
    }
}

/// Query units and returns TurnOrderList
pub fn generate_turn_order(unit_q: Query<(Entity, &Speed)>, mut commands: Commands) {
    let mut query: Vec<(Entity, Speed)> = Vec::new();
    for (ent, speed_ptr) in unit_q.iter() {
        query.push((ent, *speed_ptr));
    }
    commands.insert_resource(TurnOrderList::new_sorted(query));
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
        assert_eq!(turnord.next().1, 2);
        assert_eq!(turnord.next().1, 3);
        assert_eq!(turnord.next().1, 5);
        assert_eq!(turnord.next().1, 41);
        assert_eq!(turnord.next().1, 2);
    }
    #[test]
    fn next_after_delete_pass() {
        let list = unsorted_vec();
        let mut turnord = TurnOrderList::new_sorted(list);
        turnord.unit_vec.remove(3);
        assert_eq!(turnord.next().1, 2);
        assert_eq!(turnord.next().1, 3);
        // assert_eq!(turnord.next(), &5);
        assert_eq!(turnord.next().1, 41);
        assert_eq!(turnord.next().1, 2);
        assert_eq!(turnord.next().1, 2);
    }
    #[test]
    fn delete_next_pass() {
        let list = unsorted_vec();
        let mut turnord = TurnOrderList::new_sorted(list); // (2) 2 3 5 41
        assert_eq!(turnord.index, 0);

        assert_eq!(turnord.next().1, 2); // 2 (2) 3 5 41
        assert_eq!(turnord.index, 1);

        assert_eq!(turnord.next().1, 3); // 2 2 (3) 5 41
        assert_eq!(turnord.index, 2);

        turnord.remove(Some(turnord.index)); // 2 (2) 5 41
        assert_eq!(turnord.index, 1);

        assert_eq!(turnord.next().1, 5); // 2 2 (5) 41
        assert_eq!(turnord.index, 2);
        assert_eq!(turnord.next().1, 41); // 2 2 5 (41)
        turnord.remove(None); // 2 2 (5)
        assert_eq!(turnord.get_current(), &(None, 5));

        assert_eq!(turnord.next().1, 2); // (2) 2 5
        turnord.remove(Some(1)); // (2) 5
        assert_eq!(turnord.next().1, 5); // 2 (5)
    }
    #[test]
    fn take_pass() {
        let list = unsorted_vec();
        let mut turnord = TurnOrderList::new_sorted(list); // (2) 2 3 5 41
        assert_eq!(turnord.take(Some(3)), (None, 5)); // take 5, vec (2) 2 3 41
        assert_eq!(turnord.index, 0);
        assert_eq!(
            turnord.unit_vec,
            vec![(None, 2), (None, 2), (None, 3), (None, 41)]
        );
        assert_eq!(turnord.take(None), (None, 2)); // take 2, vec (2) 3 41
        assert_eq!(turnord.index, 0);
        assert_eq!(turnord.unit_vec, vec![(None, 2), (None, 3), (None, 41)]);
    }
}
