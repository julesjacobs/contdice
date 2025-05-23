/// A binary tree with leaves of type L and nodes of type N, represented with
/// child pointers
use segment_tree::{ops::Min, SegmentPoint};
use std::collections::{HashMap, VecDeque};

/// A generic B-Tree type with leaf data type L and node data type N
#[derive(Clone, Debug)]
pub enum BTree<N, L>
where
    N: PartialEq + Eq + Clone,
    L: PartialEq + Eq + Clone,
{
    Leaf(L),
    Node(N, Box<BTree<N, L>>, Box<BTree<N, L>>),
}

/// A depth-first iterator for a tree
/// Visits each node in depth-first order
pub struct BreadthFirstIter<'a, N: 'a, L: 'a>
where
    N: PartialEq + Eq + Clone,
    L: PartialEq + Eq + Clone,
{
    queue: VecDeque<&'a BTree<N, L>>,
}

impl<'a, N, L> Iterator for BreadthFirstIter<'a, N, L>
where
    N: PartialEq + Eq + Clone,
    L: PartialEq + Eq + Clone,
{
    type Item = &'a BTree<N, L>;

    fn next(&mut self) -> Option<Self::Item> {
        use self::BTree::*;
        match self.queue.pop_front() {
            None => None,
            Some(v) => match v {
                &Leaf(_) => Some(v),
                &Node(_, ref l, ref r) => {
                    self.queue.push_back(l);
                    self.queue.push_back(r);
                    Some(v)
                }
            },
        }
    }
}

pub struct DepthFirstIter<'a, N: 'a, L: 'a>
where
    N: PartialEq + Eq + Clone,
    L: PartialEq + Eq + Clone,
{
    stack: Vec<&'a BTree<N, L>>,
}

impl<'a, N, L> Iterator for DepthFirstIter<'a, N, L>
where
    N: PartialEq + Eq + Clone,
    L: PartialEq + Eq + Clone,
{
    type Item = &'a BTree<N, L>;

    fn next(&mut self) -> Option<Self::Item> {
        use self::BTree::*;
        match self.stack.pop() {
            None => None,
            Some(v) => match v {
                &Leaf(_) => Some(v),
                &Node(_, _, ref r) => {
                    let mut cur: &'a BTree<N, L> = r;
                    loop {
                        match cur {
                            &BTree::Leaf(_) => {
                                self.stack.push(cur);
                                break;
                            }
                            &BTree::Node(_, ref l, _) => {
                                self.stack.push(cur);
                                cur = l;
                            }
                        }
                    }
                    Some(v)
                }
            },
        }
    }
}

impl<N, L> BTree<N, L>
where
    N: PartialEq + Eq + Clone,
    L: PartialEq + Eq + Clone,
{
    pub fn dfs_iter<'a>(&'a self) -> DepthFirstIter<'a, N, L> {
        let mut v = Vec::new();
        let mut cur = self;
        loop {
            match cur {
                &BTree::Leaf(_) => {
                    v.push(cur);
                    return DepthFirstIter { stack: v };
                }
                &BTree::Node(_, ref l, _) => {
                    v.push(cur);
                    cur = l;
                }
            }
        }
    }

    pub fn bfs_iter<'a>(&'a self) -> BreadthFirstIter<'a, N, L> {
        BreadthFirstIter {
            queue: VecDeque::from([self]),
        }
    }

    pub fn contains_leaf<F>(&self, f: &F) -> bool
    where
        F: Fn(&L) -> bool,
    {
        match self {
            &BTree::Leaf(ref v) => f(v),
            &BTree::Node(_, ref l, ref r) => l.contains_leaf(f) || r.contains_leaf(f),
        }
    }

    /// Find the breadth-first index of a leaf which satisfies F; None
    /// if none is found
    pub fn find_leaf_idx<F>(&self, f: &F) -> Option<usize>
    where
        F: Fn(&L) -> bool,
    {
        for (idx, i) in self.dfs_iter().enumerate() {
            match i {
                &BTree::Node(_, _, _) => (),
                &BTree::Leaf(ref l) => {
                    if f(l) {
                        return Some(idx);
                    } else {
                    }
                }
            }
        }
        None
    }

    /// Attempt to extract the data from a leaf node; panics if not a leaf
    pub fn extract_leaf(&self) -> &L {
        match self {
            Self::Leaf(ref v) => v,
            _ => panic!("extracting non-leaf"),
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            Self::Leaf(_) => true,
            _ => false,
        }
    }

    /// Flatten a BTree into a breadth-first iteration
    pub fn flatten(&self) -> Vec<&BTree<N, L>> {
        let mut v = Vec::new();
        for i in self.dfs_iter() {
            v.push(i)
        }
        v
    }

    /// Generates a mapping from DFS indexing to BFS indexing
    /// For instance, a tree can have the following DFS and BFS orderings:
    /// DFS:
    ///    0
    ///  1    4
    /// 2 3  5 6
    /// BFS:
    ///    0
    ///  1    2
    /// 3 4  5 6
    /// Then, the resulting vec will be:
    /// [0, 1, 3, 4, 2, 5, 6]
    pub fn dfs_to_bfs_mapping(&self) -> Vec<usize> {
        let mut r = Vec::new();
        let bfs_map = self.bfs_labeling();
        for i in self.dfs_iter() {
            let p: *const Self = i;
            r.push(bfs_map[&p]);
        }
        r
    }

    /// gives a vector that maps BTree pointers into their labeling in a breadth-first search ordering
    pub fn bfs_labeling(&self) -> HashMap<*const Self, usize> {
        let mut h: HashMap<*const Self, usize> = HashMap::new();
        for (idx, itm) in self.bfs_iter().enumerate() {
            let p: *const Self = itm;
            h.insert(p, idx);
        }
        h
    }

    pub fn dfs_labeling(&self) -> HashMap<*const Self, usize> {
        let mut h: HashMap<*const Self, usize> = HashMap::new();
        for (idx, itm) in self.dfs_iter().enumerate() {
            let p: *const Self = itm;
            h.insert(p, idx);
        }
        h
    }

    pub fn bfs_to_dfs_mapping(&self) -> Vec<usize> {
        let mut r = Vec::new();
        let dfs_map = self.dfs_labeling();
        for i in self.bfs_iter() {
            let p: *const Self = i;
            r.push(dfs_map[&p]);
        }
        r
    }
}

#[test]
fn test_traversal() {
    use self::BTree::*;
    let vtree: BTree<i32, i32> = Node(
        4,
        Box::new(Node(2, Box::new(Leaf(1)), Box::new(Leaf(3)))),
        Box::new(Node(6, Box::new(Leaf(5)), Box::new(Leaf(7)))),
    );

    for (idx, v) in vtree.dfs_iter().enumerate() {
        let value = match v {
            &Node(ref v, _, _) => *v,
            &Leaf(ref v) => *v,
        };
        assert_eq!((idx + 1) as i32, value);
    }
}

#[derive(Debug, Clone)]
/// A helper structure for efficiently computing the LCA of a tree
pub struct LeastCommonAncestor {
    seg_tree: SegmentPoint<usize, Min>,
    /// maps BFS-ordered tree indices into their first occurrence in the Euler tour
    index_map: Vec<usize>,
}

impl LeastCommonAncestor {
    /// Build a vector that holds the Euler tour of the tree
    /// with nodes indexed in breadth-first order
    pub fn build_euler_vec<N, L>(
        tree: &BTree<N, L>,
        map: &HashMap<*const BTree<N, L>, usize>,
        v: &mut Vec<usize>,
    ) where
        N: PartialEq + Eq + Clone,
        L: PartialEq + Eq + Clone,
    {
        let p: *const BTree<N, L> = tree;
        let idx = map[&p];
        match &tree {
            BTree::Leaf(_) => v.push(idx),
            BTree::Node(_, ref l, ref r) => {
                v.push(idx);
                LeastCommonAncestor::build_euler_vec(l, map, v);
                v.push(idx);
                LeastCommonAncestor::build_euler_vec(r, map, v);
                v.push(idx);
            }
        }
    }

    pub fn new<N, L>(tree: &BTree<N, L>) -> LeastCommonAncestor
    where
        N: PartialEq + Eq + Clone,
        L: PartialEq + Eq + Clone,
    {
        let mut euler_vec = Vec::new();
        let bfs_map = tree.bfs_labeling();
        LeastCommonAncestor::build_euler_vec(tree, &bfs_map, &mut euler_vec);

        // build the node lookup map
        let n = tree.dfs_iter().count();
        let mut lookup = vec![None; n];
        for i in 0..(euler_vec.len()) {
            let cur_var = euler_vec[i];
            if lookup[cur_var].is_none() {
                lookup[cur_var] = Some(i);
            }
        }
        LeastCommonAncestor {
            seg_tree: SegmentPoint::build(euler_vec, Min),
            index_map: lookup.iter().map(|x| x.unwrap()).collect(),
        }
    }

    /// Given two breadth-first indexes into the tree, returns the LCA
    /// between these two indexes
    pub fn lca(&self, l: usize, r: usize) -> usize {
        if l == r {
            l
        } else {
            let (l, r) = (self.index_map[l], self.index_map[r]);
            let (l, r) = if l < r { (l, r) } else { (r, l) };
            self.seg_tree.query(l, r)
        }
    }
}

#[test]
fn test_lca() {
    use self::BTree::*;
    //    0
    //  1    2
    // 3 4  5 6
    let vtree: BTree<i32, i32> = Node(
        0,
        Box::new(Node(1, Box::new(Leaf(3)), Box::new(Leaf(4)))),
        Box::new(Node(2, Box::new(Leaf(5)), Box::new(Leaf(6)))),
    );
    let lca = LeastCommonAncestor::new(&vtree);
    assert_eq!(lca.lca(3, 4), 1);
    assert_eq!(lca.lca(3, 5), 0);
    assert_eq!(lca.lca(1, 3), 1);
    assert_eq!(lca.lca(1, 6), 0);
    assert_eq!(lca.lca(1, 1), 1);
    assert_eq!(lca.lca(5, 6), 2);
}
