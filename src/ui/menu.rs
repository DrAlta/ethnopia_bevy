pub trait Item{
    fn enabled(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MenuTree<I: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash>{
    Children(Vec<MenuTree<I>>),
    Item(I),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pruned<I: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash>{
    Branch(Vec<Pruned<I>>),
    Item(I),
}

impl<'a, I: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> MenuTree<I>{
    pub fn prune(&'a self) -> Option<Pruned<&'a I>>
    where &'a I: Item {
        let mut branches: Vec<Pruned<&I>> = match self {
            MenuTree::Children(menu_trees) => {
                menu_trees.iter().filter_map(|child|{
                    child.prune()
                }).collect()
            },
            MenuTree::Item(x) => {
                return if x.enabled() {
                    Some(Pruned::Item(x))
                } else {
                    None
                }
            },
        };
        // if we don't have any children return None
        if branches.is_empty() {
            return None
        };
        // if we have only one child just return it
        if branches.len() == 1 {
            return branches.pop()
        };
        Some(Pruned::Branch(branches))
    }
}