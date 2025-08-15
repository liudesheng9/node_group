from .node_group import PyIdNode, PyIdPair, group_id_pairs as group_id_pairs_rs
from typing import List




class IdNode:
    inner: PyIdNode

    def __init__(self, id_type: str, id_name: str):
        self.inner = PyIdNode(id_type, id_name)

    def __init__(self, inner: PyIdNode):
        self.inner = inner

    @staticmethod
    def from_string(id_str: str) -> "IdNode":
        py_id_node = PyIdNode.from_string(id_str)
        node = IdNode(py_id_node.id_type, py_id_node.id_name)
        node.inner = py_id_node
        return node

    @property
    def id_name(self) -> str:
        return self.inner.id_name

    @property
    def id_type(self) -> str:
        return self.inner.id_type
    
    @id_name.setter
    def set_id_name(self, id_name: str):
        self.inner.set_id_name(id_name)
    
    @id_type.setter
    def set_id_type(self, id_type: str):
        self.inner.set_id_type(id_type)

    def as_string(self) -> str:
        return self.inner.as_string()

    def __str__(self):
        return self.inner.__str__()

    def __repr__(self):
        return self.inner.__repr__()

    def __eq__(self, other):
        if not isinstance(other, IdNode):
            return NotImplemented
        return self.id_type == other.id_type and self.id_name == other.id_name


class IdPair:
    inner: PyIdPair

    def __init__(self, id_node1: IdNode, id_node2: IdNode):
        self.inner = PyIdPair(id_node1.inner, id_node2.inner)

    @staticmethod
    def from_string(id_pair_str: str) -> "IdPair":
        py_id_pair = PyIdPair.from_string(id_pair_str)
        node1 = IdNode(py_id_pair.node1().id_type, py_id_pair.node1().id_name)
        node2 = IdNode(py_id_pair.node2().id_type, py_id_pair.node2().id_name)
        pair = IdPair(node1, node2)
        pair.inner = py_id_pair
        return pair

    @property
    def node1(self) -> IdNode:
        py_node = self.inner.node1()
        return IdNode(py_node.id_type, py_node.id_name)

    @property
    def node2(self) -> IdNode:
        py_node = self.inner.node2()
        return IdNode(py_node.id_type, py_node.id_name)

    def as_string(self) -> str:
        return self.inner.as_string()

    def __eq__(self, other):
        if not isinstance(other, IdPair):
            return NotImplemented
        return {self.node1, self.node2} == {other.node1, other.node2}


def group_id_pairs(id_pairs: List[IdPair]) -> List[List[IdNode]]:
    py_id_pairs = [pair.inner for pair in id_pairs]
    grouped_nodes_rs = group_id_pairs_rs(py_id_pairs)

    return [[IdNode(node_rs) for node_rs in group_rs] for group_rs in grouped_nodes_rs]
    