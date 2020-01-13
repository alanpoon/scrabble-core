import numpy as np
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, Iterator, List, Set, Optional

from cached_property import cached_property

Letter = str
MIN_LETTER = ord('a')
LETTER_OFFSET = 0
WORD_TERMINATOR_BIT_OFFSET = 8
NODE_TERMINATOR_BIT_OFFSET = 16
TARGET_BIT_OFFSET = 32


class FrozenTrie(frozenset):
    def display(self):
        inner_items = [f"{repr(k)} -> {v.display()}" if v else repr(k) for k, v in self]
        inner = ", ".join(inner_items)
        return f"{{{inner}}}"


@dataclass
class DawgEdge:
    letter: str
    word_terminator: bool
    target: Optional[int]
    node_terminator: bool

    def as_int(self) -> int:
        result = ord(self.letter) << 0
        if self.word_terminator:
            result += 1 << WORD_TERMINATOR_BIT_OFFSET
        if self.node_terminator:
            result += 1 << NODE_TERMINATOR_BIT_OFFSET
        target = self.target if self.target is not None else 2 ** 32 - 1  # all ones
        result += target << TARGET_BIT_OFFSET
        return result


@dataclass(eq=False)
class TrieNode:
    word_terminator: bool = False
    children: Dict[Letter, "TrieNode"] = field(default_factory=dict)
    edges_start: Optional[int] = None

    def dawg_edges(self, visited_nodes: Set["TrieNode"]) -> Iterator[DawgEdge]:
        if self in visited_nodes:
            return
        visited_nodes.add(self)

        children = self.children
        for i, (letter, child) in enumerate(children.items(), 1):
            edges_start = child.edges_start if child.children else None
            node_terminator = i == len(children)
            yield DawgEdge(letter, child.word_terminator, edges_start, node_terminator)
        for child in children.values():
            yield from child.dawg_edges(visited_nodes)

    @property
    def size(self) -> int:
        return len(self.children)

    def get_or_create_child(self, c: Letter) -> "TrieNode":
        child = self.children.get(c)
        if child is None:
            child = TrieNode()
            self.children[c] = child
        return child

    @cached_property
    def frozen(self) -> FrozenTrie:
        return FrozenTrie(((k, v.word_terminator), v.frozen) for k, v in self.children.items())

    def convert_to_dawg(self, seen_nodes: Dict["TrieNode", "TrieNode"]) -> None:
        if self in seen_nodes:
            return  # will be replaced
        for node in self.children.values():
            node.convert_to_dawg(seen_nodes)
        for k, node in self.children.items():
            self.children[k] = seen_nodes[node]
        seen_nodes[self] = self

    def descendants(self) -> List["TrieNode"]:
        return list(self._descendants())

    def _descendants(self) -> Iterator["TrieNode"]:
        children = self.children.values()
        for node in children:
            yield node
        for node in children:
            yield from node.descendants()

    def __eq__(self, other: "TrieNode") -> bool:
        if not isinstance(other, TrieNode):
            return False
        return self.frozen == other.frozen

    def __hash__(self) -> int:
        return hash(self.frozen)


@dataclass
class Trie:
    nodes_tree: List[TrieNode] = field(init=False)

    def __post_init__(self) -> None:
        self.nodes_tree = [TrieNode()]  # root node

    @property
    def root_node(self) -> TrieNode:
        return self.nodes_tree[0]

    @classmethod
    def from_words(cls, words: List[str]) -> "Trie":
        trie = Trie()
        trie.add_words(words)
        return trie

    def add_words(self, words: List[str]) -> None:
        for word in words:
            self.add_word(word)

    def add_word(self, word: str) -> None:
        node = self.root_node
        for c in word:
            node = node.get_or_create_child(c)
        node.word_terminator = True

    def convert_to_dawg(self) -> None:
        seen_nodes: Dict[TrieNode, TrieNode] = dict()
        self.root_node.convert_to_dawg(seen_nodes)

    def label_edges_starts(self) -> List[TrieNode]:
        index = 0
        ordered_nodes = [self.root_node] + self.root_node.descendants()
        node_labels: Dict[TrieNode, int] = {}
        for node in ordered_nodes:
            edges_start = node_labels.get(node)
            if edges_start is None:
                node_labels[node] = edges_start = index
                index += node.size
            node.edges_start = edges_start
        return ordered_nodes

    def dawg_edges(self) -> List[DawgEdge]:
        return list(self.root_node.dawg_edges(set()))


def main() -> None:
    words_path = Path(__file__).parent / "scrabble_words.txt"
    words = words_path.read_text().strip().split("\n")

    trie = Trie.from_words(words)
    trie.convert_to_dawg()
    trie.label_edges_starts()
    edges = trie.dawg_edges()
    dawg_bytes = np.array([edge.as_int() for edge in edges]).tobytes()

    with Path("dawg.bin").open("wb") as f:
        f.write(dawg_bytes)


if __name__ == '__main__':
    main()
