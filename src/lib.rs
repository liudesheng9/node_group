use pyo3::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};

// =============================
// Core data types and logic
// =============================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdNode {
    id_name: String,
    id_type: String,
}

impl IdNode {
    fn parse(id_str: &str) -> Self {
        let parts: Vec<&str> = id_str.split("::").collect();
        IdNode {
            id_name: parts[1].to_string(),
            id_type: parts[0].to_string(),
        }
    }

    fn get_id_name(&self) -> &str {
        &self.id_name
    }

    fn get_id_type(&self) -> &str {
        &self.id_type
    }

    fn set_id_name(&mut self, id_name: String) {
        self.id_name = id_name;
    }

    fn set_id_type(&mut self, id_type: String) {
        self.id_type = id_type;
    }
}

impl Display for IdNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.id_type, self.id_name)
    }
}

#[derive(Debug, Clone)]
pub struct IdPair {
    id_node1: IdNode,
    id_node2: IdNode,
    id_set: HashSet<IdNode>,
}

impl IdPair {
    fn create(id_node1: IdNode, id_node2: IdNode) -> Self {
        let mut id_set = HashSet::new();
        id_set.insert(id_node1.clone());
        id_set.insert(id_node2.clone());
        IdPair {
            id_node1,
            id_node2,
            id_set,
        }
    }

    fn check_node_in_pair(&self, id_in: &IdNode) -> bool {
        self.id_set.contains(id_in)
    }

    fn get_next_node(&self, id_in: &IdNode) -> &IdNode {
        if id_in == &self.id_node1 {
            &self.id_node2
        } else if id_in == &self.id_node2 {
            &self.id_node1
        } else {
            panic!("id_in is not in the id pair");
        }
    }

    fn to_pair_string(&self) -> String {
        format!("{}${}", self.id_node1, self.id_node2)
    }

    fn parse(id_pair_str: &str) -> Self {
        let parts: Vec<&str> = id_pair_str.split('$').collect();
        let id_node1 = IdNode::parse(parts[0]);
        let id_node2 = IdNode::parse(parts[1]);
        Self::create(id_node1, id_node2)
    }
}

impl PartialEq for IdPair {
    fn eq(&self, other: &Self) -> bool {
        self.id_set == other.id_set
    }
}

impl Eq for IdPair {}

#[derive(Debug)]
struct IdPairGraph {
    id_pair_list: Vec<IdPair>,
    id_nodes: Vec<IdNode>,
    adj: HashMap<IdNode, Vec<IdNode>>,
}

impl IdPairGraph {
    fn new(id_pair_list: Vec<IdPair>, id_nodes: Vec<IdNode>) -> Self {
        let mut adj: HashMap<IdNode, Vec<IdNode>> = id_nodes
            .iter()
            .cloned()
            .map(|node| (node, Vec::new()))
            .collect();
        for pair in &id_pair_list {
            if let Some(neighbors) = adj.get_mut(&pair.id_node1) {
                neighbors.push(pair.id_node2.clone());
            }
            if let Some(neighbors) = adj.get_mut(&pair.id_node2) {
                neighbors.push(pair.id_node1.clone());
            }
        }
        IdPairGraph {
            id_pair_list,
            id_nodes,
            adj,
        }
    }

    fn get_neighbor(&self, id_in: &IdNode) -> &[IdNode] {
        self.adj.get(id_in).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

fn group_id_pairs_core(id_pairs: Vec<IdPair>) -> Vec<Vec<IdNode>> {
    let mut nodes: HashSet<IdNode> = HashSet::new();
    for pair in &id_pairs {
        nodes.insert(pair.id_node1.clone());
        nodes.insert(pair.id_node2.clone());
    }
    let id_nodes_vec: Vec<IdNode> = nodes.into_iter().collect();
    let graph = IdPairGraph::new(id_pairs, id_nodes_vec.clone());

    let mut visited: HashSet<IdNode> = HashSet::new();
    let mut groups: Vec<Vec<IdNode>> = Vec::new();
    for node in &graph.id_nodes {
        if !visited.contains(node) {
            let mut group: Vec<IdNode> = Vec::new();
            let mut q: VecDeque<IdNode> = VecDeque::new();
            q.push_back(node.clone());
            visited.insert(node.clone());
            while let Some(curr) = q.pop_front() {
                group.push(curr.clone());
                for neighbor in graph.get_neighbor(&curr).iter().cloned() {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor.clone());
                        q.push_back(neighbor);
                    }
                }
            }
            groups.push(group);
        }
    }
    groups
}

// =============================
// Python bindings
// =============================

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyIdNode {
    inner: IdNode,
}

#[pymethods]
impl PyIdNode {
    #[new]
    pub fn new(id_type: String, id_name: String) -> Self {
        Self {
            inner: IdNode { id_name, id_type },
        }
    }

    #[staticmethod]
    pub fn from_string(id_str: &str) -> Self {
        Self {
            inner: IdNode::parse(id_str),
        }
    }

    #[getter]
    pub fn id_name(&self) -> &str {
        self.inner.get_id_name()
    }

    #[getter]
    pub fn id_type(&self) -> &str {
        self.inner.get_id_type()
    }

    pub fn as_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn __str__(&self) -> String {
        self.inner.to_string()
    }
    pub fn __repr__(&self) -> String {
        self.inner.to_string()
    }

    #[setter]
    pub fn set_id_name(&mut self, id_name: String) {
        self.inner.set_id_name(id_name);
    }

    #[setter]
    pub fn set_id_type(&mut self, id_type: String) {
        self.inner.set_id_type(id_type);
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyIdPair {
    inner: IdPair,
}

#[pymethods]
impl PyIdPair {
    #[new]
    pub fn new(id_node1: PyRef<PyIdNode>, id_node2: PyRef<PyIdNode>) -> Self {
        Self {
            inner: IdPair::create((*id_node1).inner.clone(), (*id_node2).inner.clone()),
        }
    }

    #[staticmethod]
    pub fn from_string(id_pair_str: &str) -> Self {
        Self {
            inner: IdPair::parse(id_pair_str),
        }
    }

    pub fn as_string(&self) -> String {
        self.inner.to_pair_string()
    }

    pub fn node1(&self) -> PyIdNode {
        PyIdNode {
            inner: self.inner.id_node1.clone(),
        }
    }
    pub fn node2(&self) -> PyIdNode {
        PyIdNode {
            inner: self.inner.id_node2.clone(),
        }
    }
}

#[pyfunction]
pub fn group_id_pairs(id_pairs: Vec<PyIdPair>) -> Vec<Vec<PyIdNode>> {
    let pairs: Vec<IdPair> = id_pairs.into_iter().map(|s| s.inner).collect();
    group_id_pairs_core(pairs)
        .into_iter()
        .map(|group| group.into_iter().map(|n| PyIdNode { inner: n }).collect())
        .collect()
}

#[pymodule]
fn node_group(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyIdNode>()?;
    m.add_class::<PyIdPair>()?;
    m.add_function(wrap_pyfunction!(group_id_pairs, m)?)?;
    Ok(())
}
