import itertools
from typing import List

import polars as pl

from .structs import IdNode, IdPair



def get_id_pairs_from_df(df: pl.DataFrame) -> List[IdPair]:
    """
    Generates PyIdPairs from a Polars DataFrame.

    Each column is considered an ID type, and each row contains IDs.
    Pairs are created for every combination of two columns in the DataFrame.

    Returns a list of PyIdPair objects compatible with `node_group.group_id_pairs`.
    """
    id_pairs: List[IdPair] = []
    column_pairs = list(itertools.combinations(df.columns, 2))

    for col1, col2 in column_pairs:
        filtered_df = df.filter(pl.col(col1).is_not_null() & pl.col(col2).is_not_null())
        if not filtered_df.is_empty():
            pairs_data = filtered_df.select([col1, col2]).to_dicts()
            for pair in pairs_data:
                # Rust constructor for PyIdNode is (id_type, id_name)
                node1 = IdNode(col1, str(pair[col1]))
                node2 = IdNode(col2, str(pair[col2]))
                id_pairs.append(IdPair(node1, node2))
    return id_pairs


