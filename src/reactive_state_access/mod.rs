// If the stored type is clone, then implement clone for ReactiveStateAccess
pub mod atom;
pub mod atom_undo;
pub mod reaction;

pub trait CloneReactiveState<T>
where
    T: Clone + 'static,
{
    fn get(&self) -> T;
    fn soft_get(&self) -> Option<T>;
}

pub trait ObserveChangeReactiveState<T>
where
    T: Clone + 'static + PartialEq,
{
    fn observe_change(&self) -> (Option<T>, T);
    fn has_changed(&self) -> bool;
    fn on_change<F: FnOnce(&T, &T) -> R, R>(&self, func: F) -> R;
}
