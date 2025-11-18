#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Iterable, List, Optional, Set, Tuple


EDGE_REL_MAP_SUBSTR = [
    ("hastag", "Has_Tag"),
    ("hascreator", "Has_Creator"),
    ("hasinterest", "Has_Interest"),
    ("hastype", "Has_Type"),
    ("islocatedin", "Is_Located_In"),
    ("ispartof", "Is_Part_Of"),
    ("hasmember", "Has_Member"),
    ("containerof", "Container_Of"),
    ("replyof", "Reply_Of"),
    ("likes", "Like"),
    ("knows", "Knows"),
]

# Edge-table relation keyword normalization for Cypher-style type strings.
# Example: comment_hastag_tag -> Comment_has_tag
EDGE_REL_MID_NORMALIZE: Dict[str, str] = {
    "hastag": "has_tag",
    "hascreator": "has_creator",
    "hasinterest": "has_interest",
    "hastype": "has_type",
    "islocatedin": "is_located_in",
    "ispartof": "is_part_of",
    "hasmember": "has_member",
    "containerof": "container_of",
    "replyof": "reply_of",
    "knows": "knows",
    "likes": "likes",
}


def _label_from_table(table: str) -> str:
    # node label: TagClass, Person, Comment, Forum, City, Country, ...
    parts = [p for p in table.strip().split("_") if p]
    return "".join(p[:1].upper() + p[1:] for p in parts)


def _rel_from_edge_table(table: str) -> str:
    tl = table.lower()
    for sub, name in EDGE_REL_MAP_SUBSTR:
        if sub in tl:
            return name
    # fallback: Title_Case_With_Underscore
    parts = [p for p in table.strip().split("_") if p]
    return "_".join(p[:1].upper() + p[1:] for p in parts)


def _cypher_rel_type_from_edge_table(edge_table: str) -> str:
    """
    Convert edge-table name into Cypher relationship type.
    Prefer: <SrcLabel>_<rel_mid>  (and sometimes keep dst to disambiguate likes_*).

    Examples:
      comment_hastag_tag -> Comment_has_tag
      comment_hascreator_person -> Comment_has_creator
      person_hasinterest_tag -> Person_has_interest
      person_likes_comment -> Person_likes_comment   (keep dst to disambiguate likes_post vs likes_comment)
      person_knows_person -> Person_knows
    """
    parts = [p for p in edge_table.strip().split("_") if p]
    if len(parts) >= 3:
        src = parts[0]
        dst = parts[-1]
        mid_raw = "_".join(parts[1:-1]).lower().replace("_", "")
        mid = EDGE_REL_MID_NORMALIZE.get(mid_raw, "_".join(parts[1:-1]).lower())
        src_label = _label_from_table(src)
        # disambiguate likes_* by keeping dst suffix
        if mid == "likes":
            return f"{src_label}_{mid}_{dst}"
        return f"{src_label}_{mid}"
    # fallback
    return _label_from_table(edge_table)


class UnionFind:
    def __init__(self) -> None:
        self.parent: Dict[str, str] = {}
        self.rank: Dict[str, int] = {}

    def find(self, x: str) -> str:
        if x not in self.parent:
            self.parent[x] = x
            self.rank[x] = 0
            return x
        # path compression
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, a: str, b: str) -> None:
        ra = self.find(a)
        rb = self.find(b)
        if ra == rb:
            return
        if self.rank[ra] < self.rank[rb]:
            ra, rb = rb, ra
        self.parent[rb] = ra
        if self.rank[ra] == self.rank[rb]:
            self.rank[ra] += 1

    def groups(self) -> Dict[str, Set[str]]:
        g: Dict[str, Set[str]] = {}
        for x in list(self.parent.keys()):
            r = self.find(x)
            g.setdefault(r, set()).add(x)
        return g


def _split_where_predicates(where_sql: str) -> List[str]:
    """
    Split a WHERE clause string into individual predicate strings.
    Handles BETWEEN ... AND ... by not splitting on the AND inside BETWEEN.
    """
    s = where_sql.strip()
    if not s:
        return []

    out: List[str] = []
    buf: List[str] = []

    i = 0
    in_quote: Optional[str] = None
    between_pending = 0  # 1 means we've seen BETWEEN and haven't consumed its inner AND yet

    def flush() -> None:
        t = "".join(buf).strip()
        if t:
            out.append(t)
        buf.clear()

    while i < len(s):
        ch = s[i]
        if in_quote:
            buf.append(ch)
            if ch == in_quote:
                # handle doubled quotes inside SQL strings: '' stays in string
                if in_quote == "'" and i + 1 < len(s) and s[i + 1] == "'":
                    buf.append("'")
                    i += 2
                    continue
                in_quote = None
            i += 1
            continue

        if ch in ("'", '"'):
            in_quote = ch
            buf.append(ch)
            i += 1
            continue

        # lookahead for keywords
        if s[i : i + 7].upper() == "BETWEEN" and (i == 0 or not s[i - 1].isalnum()) and (
            i + 7 == len(s) or not s[i + 7].isalnum()
        ):
            between_pending = 1
            buf.append(s[i : i + 7])
            i += 7
            continue

        # split on AND, except the one inside BETWEEN ... AND ...
        if s[i : i + 5].upper() == " AND ":
            if between_pending == 1:
                # consume the BETWEEN's AND
                between_pending = 0
                buf.append(" AND ")
                i += 5
                continue
            flush()
            i += 5
            continue

        buf.append(ch)
        i += 1

    flush()
    return out


_RE_FROM = re.compile(r"^\s*FROM\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+AS\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*$", re.IGNORECASE)
_RE_JOIN = re.compile(r"^\s*JOIN\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+AS\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*$", re.IGNORECASE)
_RE_EQ = re.compile(
    r"([a-zA-Z_][a-zA-Z0-9_]*)\.([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*([a-zA-Z_][a-zA-Z0-9_]*)\.([a-zA-Z_][a-zA-Z0-9_]*)"
)


@dataclass(frozen=True)
class JoinInfo:
    alias_to_table: Dict[str, str]
    # union-find classes on alias.col (all lower-cased columns)
    uf: UnionFind
    # columns observed per alias (from ON clauses)
    cols_by_alias: Dict[str, Set[str]]


def _extract_base_join_block(sql_text: str) -> str:
    """
    Take the first query's FROM..WHERE block as the "base join" definition.
    """
    lines = sql_text.splitlines()
    from_i = None
    where_i = None
    for i, ln in enumerate(lines):
        if from_i is None and re.match(r"^\s*FROM\b", ln, re.IGNORECASE):
            from_i = i
            continue
        if from_i is not None and re.match(r"^\s*WHERE\b", ln, re.IGNORECASE):
            where_i = i
            break
    if from_i is None:
        return ""
    if where_i is None:
        where_i = len(lines)
    return "\n".join(lines[from_i:where_i])


def _parse_join_info(sql_text: str) -> JoinInfo:
    base = _extract_base_join_block(sql_text)
    alias_to_table: Dict[str, str] = {}
    uf = UnionFind()
    cols_by_alias: Dict[str, Set[str]] = {}

    for ln in base.splitlines():
        m = _RE_FROM.match(ln)
        if m:
            table, alias = m.group(1), m.group(2)
            alias_to_table[alias] = table
            continue
        m = _RE_JOIN.match(ln)
        if m:
            table, alias = m.group(1), m.group(2)
            alias_to_table[alias] = table
            continue

        for a1, c1, a2, c2 in _RE_EQ.findall(ln):
            uf.union(f"{a1}.{c1.lower()}", f"{a2}.{c2.lower()}")
            cols_by_alias.setdefault(a1, set()).add(c1.lower())
            cols_by_alias.setdefault(a2, set()).add(c2.lower())

    return JoinInfo(alias_to_table=alias_to_table, uf=uf, cols_by_alias=cols_by_alias)


def _is_edge_alias(alias: str, alias_to_table: Dict[str, str]) -> bool:
    t = alias_to_table.get(alias, "")
    return "_" in t


def _build_entity_graph(join: JoinInfo) -> Tuple[Set[str], Dict[str, Tuple[str, str]]]:
    """
    Build a node-alias graph whose edges are edge-table aliases.
    Returns:
      - node_aliases
      - edge_alias -> (node_u, node_v)
    """
    alias_to_table = join.alias_to_table
    uf_groups = join.uf.groups()

    node_aliases: Set[str] = {a for a, t in alias_to_table.items() if "_" not in t}
    edge_aliases: Set[str] = set(alias_to_table.keys()) - node_aliases

    # Build reverse: element -> group id
    elem_to_gid: Dict[str, str] = {}
    for gid, elems in uf_groups.items():
        for e in elems:
            elem_to_gid[e] = gid

    # For each node alias, only consider its ".id" as its key.
    node_key_gid: Dict[str, str] = {}
    for n in node_aliases:
        k = f"{n}.id"
        gid = elem_to_gid.get(k)
        if gid:
            node_key_gid[n] = gid

    # For each edge alias, find which node keys it connects to by equivalence.
    edge_to_nodes: Dict[str, Set[str]] = {e: set() for e in edge_aliases}
    for e in edge_aliases:
        # Any e.<col> that unions with some node's id indicates endpoint.
        # Scan known union-find elements for this alias.
        for elem, gid in elem_to_gid.items():
            if not elem.startswith(e + "."):
                continue
            for n, ngid in node_key_gid.items():
                if gid == ngid:
                    edge_to_nodes[e].add(n)

    # Build endpoint pairs.
    edge_endpoints: Dict[str, Tuple[str, str]] = {}
    for e, ns in edge_to_nodes.items():
        if len(ns) >= 2:
            u, v = sorted(ns)[:2]
            edge_endpoints[e] = (u, v)
        elif len(ns) == 1:
            u = next(iter(ns))
            edge_endpoints[e] = (u, u)
        else:
            # unknown endpoints; skip from traversal
            continue

    return node_aliases, edge_endpoints


def _infer_edge_direction(
    join: JoinInfo, edge_alias: str, endpoints: Tuple[str, str]
) -> Tuple[str, str]:
    """
    Choose (src_node_alias, dst_node_alias) for an edge alias.
    Uses common column naming conventions if possible (src/dst, *1id/*2id, place1id/place2id, <token>id).
    Falls back to endpoints as given.
    """
    u, v = endpoints
    alias_to_table = join.alias_to_table
    edge_table = alias_to_table.get(edge_alias, "")
    cols = sorted(join.cols_by_alias.get(edge_alias, set()))

    # map edge column -> node alias by union-find equivalence with node.id
    uf_groups = join.uf.groups()
    elem_to_gid: Dict[str, str] = {}
    for gid, elems in uf_groups.items():
        for e in elems:
            elem_to_gid[e] = gid

    node_id_gid: Dict[str, str] = {}
    for a, t in alias_to_table.items():
        if "_" in t:
            continue
        gid = elem_to_gid.get(f"{a}.id")
        if gid:
            node_id_gid[a] = gid

    col_to_node: Dict[str, str] = {}
    for c in cols:
        gid = elem_to_gid.get(f"{edge_alias}.{c}")
        if not gid:
            continue
        for n, ngid in node_id_gid.items():
            if gid == ngid:
                col_to_node[c] = n

    def pick(col: str) -> Optional[str]:
        return col_to_node.get(col.lower())

    # 1) src/dst
    if "src" in col_to_node and "dst" in col_to_node:
        return (col_to_node["src"], col_to_node["dst"])

    # 2) comment1id/comment2id, place1id/place2id, *1id/*2id
    for a1, a2 in [("comment1id", "comment2id"), ("place1id", "place2id")]:
        if a1 in col_to_node and a2 in col_to_node:
            return (col_to_node[a1], col_to_node[a2])
    c1 = next((c for c in cols if c.endswith("1id") and c in col_to_node), None)
    c2 = next((c for c in cols if c.endswith("2id") and c in col_to_node), None)
    if c1 and c2:
        return (col_to_node[c1], col_to_node[c2])

    # 3) token-based (<src>id, <dst>id) from edge table name
    parts = [p for p in edge_table.split("_") if p]
    if len(parts) >= 3:
        src_tok = parts[0].lower()
        dst_tok = parts[-1].lower()
        src_col = f"{src_tok}id"
        dst_col = f"{dst_tok}id"
        if src_col in col_to_node and dst_col in col_to_node:
            return (col_to_node[src_col], col_to_node[dst_col])

    # fallback: stable order
    return (u, v)


def _euler_walk_nodes_edges(node_aliases: Set[str], edge_endpoints: Dict[str, Tuple[str, str]]) -> Tuple[List[str], List[str]]:
    """
    Return (nodes_in_order, edges_in_order) where edges connect consecutive nodes.
    Covers each edge alias once when possible.
    """
    # adjacency: node -> list of edge aliases
    adj: Dict[str, List[str]] = {n: [] for n in node_aliases}
    deg: Dict[str, int] = {n: 0 for n in node_aliases}
    for e, (u, v) in edge_endpoints.items():
        if u not in adj or v not in adj:
            continue
        adj[u].append(e)
        adj[v].append(e)
        deg[u] += 1
        deg[v] += 1

    # pick start
    odd = [n for n, d in deg.items() if d % 2 == 1 and d > 0]
    if odd:
        start = sorted(odd)[0]
    else:
        nonzero = [n for n, d in deg.items() if d > 0]
        start = sorted(nonzero)[0] if nonzero else (sorted(node_aliases)[0] if node_aliases else "")

    if not start:
        return ([], [])

    # Use stack of (node, incoming_edge) to build Euler circuit/trail.
    used: Set[str] = set()
    stack: List[Tuple[str, Optional[str]]] = [(start, None)]
    circuit: List[Tuple[str, Optional[str]]] = []

    # Make adjacency deterministic
    for n in adj:
        adj[n].sort(reverse=True)

    while stack:
        v, inc = stack[-1]
        while adj[v] and adj[v][-1] in used:
            adj[v].pop()
        if not adj[v]:
            circuit.append(stack.pop())
            continue
        e = adj[v].pop()
        if e in used:
            continue
        used.add(e)
        u, w = edge_endpoints[e]
        nxt = w if v == u else u
        stack.append((nxt, e))

    circuit.reverse()
    nodes: List[str] = []
    edges: List[str] = []
    for i, (n, inc) in enumerate(circuit):
        nodes.append(n)
        if i > 0 and inc:
            edges.append(inc)
    return nodes, edges


def _parse_where_and_truecard(line: str) -> Tuple[str, Optional[int]]:
    """
    From a line like:
      WHERE ... ; -- || 1468
    returns ("...", 1468)
    """
    m = re.search(r"--\s*\|\|\s*(\d+)\s*$", line)
    truecard = int(m.group(1)) if m else None
    # strip leading WHERE and trailing ; and comment
    s = re.sub(r"--\s*\|\|.*$", "", line).strip()
    s = re.sub(r"^\s*WHERE\s+", "", s, flags=re.IGNORECASE).strip()
    s = s.rstrip(";").strip()
    return s, truecard


def _collect_predicates_by_alias(where_sql: str, alias_to_table: Dict[str, str]) -> Dict[str, List[str]]:
    preds = _split_where_predicates(where_sql)
    out: Dict[str, List[str]] = {}
    for p in preds:
        m = re.match(r"^\s*([a-zA-Z_][a-zA-Z0-9_]*)\.(.+)$", p.strip())
        if not m:
            continue
        alias = m.group(1)
        if alias not in alias_to_table:
            continue
        body = m.group(2).strip()
        out.setdefault(alias, []).append(body)

    # Expand BETWEEN into two comparisons in output, e.g.:
    #   dst BETWEEN 10 AND 20  =>  dst >= 10  AND  dst <= 20
    for a, conds in list(out.items()):
        out[a] = _expand_between_to_range(conds)
    return out


_RE_BETWEEN = re.compile(
    r"^\s*([a-zA-Z_][a-zA-Z0-9_]*)\s+BETWEEN\s+(.+?)\s+AND\s+(.+?)\s*$",
    re.IGNORECASE,
)


def _is_number_literal(x: str) -> bool:
    s = x.strip()
    return bool(re.match(r"^[+-]?\d+(\.\d+)?$", s))


def _expand_between_to_range(conds: List[str]) -> List[str]:
    """
    Convert:
      amount BETWEEN 1000 AND 5000
    into:
      amount >= 1000
      amount <= 5000
    Keeps other predicates unchanged.
    """
    out: List[str] = []
    for c in conds:
        m = _RE_BETWEEN.match(c)
        if not m:
            out.append(c)
            continue
        col = m.group(1)
        lo = m.group(2).strip()
        hi = m.group(3).strip()
        # If numeric, ensure lo <= hi for nicer output.
        if _is_number_literal(lo) and _is_number_literal(hi):
            try:
                lo_f = float(lo)
                hi_f = float(hi)
                if lo_f > hi_f:
                    lo, hi = hi, lo
            except Exception:
                pass
        out.append(f"{col} >= {lo}")
        out.append(f"{col} <= {hi}")
    return out


def _format_alias_props(alias: str, conds: List[str], is_edge: bool) -> str:
    eqs: List[str] = []
    others: List[str] = []
    for c in conds:
        cc = c.strip()
        m = re.match(r"^([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(.+)$", cc)
        if m:
            col = m.group(1)
            rhs = m.group(2).strip()
            eqs.append(f"{col}={rhs}")
            continue
        # keep non-equality predicates as-is (including >=, <=, LIKE, IS NULL, etc.)
        others.append(cc)

    s = ""
    if eqs:
        s += " {" + ", ".join(eqs) + "}"
    if others:
        # Represent closed intervals / non-equality filters explicitly with WHERE.
        # Example: WHERE dst >= lo AND dst <= hi   (closed interval [lo, hi])
        s += " WHERE " + " AND ".join(others)
    return s


def _expand_between_predicates(preds: List[str]) -> List[str]:
    """
    Expand alias.col BETWEEN lo AND hi  ->  alias.col >= lo  AND  alias.col <= hi
    Also normalize boolean literals to TRUE/FALSE.
    """
    out: List[str] = []
    re_between = re.compile(
        r"^\s*([a-zA-Z_][a-zA-Z0-9_]*)\.([a-zA-Z_][a-zA-Z0-9_]*)\s+BETWEEN\s+(.+?)\s+AND\s+(.+?)\s*$",
        re.IGNORECASE,
    )
    for p in preds:
        pp = p.strip()
        pp = re.sub(r"^\(+", "", pp)
        pp = re.sub(r"\)+$", "", pp)
        m = re_between.match(pp)
        if m:
            a, c, lo, hi = m.group(1), m.group(2), m.group(3).strip(), m.group(4).strip()
            out.append(f"{a}.{c} >= {lo}")
            out.append(f"{a}.{c} <= {hi}")
            continue
        # normalize boolean literals (best-effort)
        pp = re.sub(r"\btrue\b", "TRUE", pp, flags=re.IGNORECASE)
        pp = re.sub(r"\bfalse\b", "FALSE", pp, flags=re.IGNORECASE)
        out.append(pp)
    return out


def _format_cypher_query(
    join: JoinInfo,
    node_aliases: Set[str],
    edge_endpoints: Dict[str, Tuple[str, str]],
    edge_order: List[str],
    where_sql: str,
    truecard: int,
) -> List[str]:
    alias_to_table = join.alias_to_table

    # Determine edges in stable order
    edges = [e for e in edge_order if e in edge_endpoints]
    if not edges:
        edges = sorted(edge_endpoints.keys())

    # Build MATCH patterns
    match_parts: List[str] = []
    seen_nodes_in_match: List[str] = []
    seen_edges_in_match: List[str] = []

    for e in edges:
        u, v = edge_endpoints[e]
        src, dst = _infer_edge_direction(join, e, (u, v))
        src_lbl = _label_from_table(alias_to_table.get(src, src))
        dst_lbl = _label_from_table(alias_to_table.get(dst, dst))
        rel_type = alias_to_table.get(e, e)
        match_parts.append(f"({src}:{src_lbl})-[{e}:{rel_type}]->({dst}:{dst_lbl})")
        if src not in seen_nodes_in_match:
            seen_nodes_in_match.append(src)
        if dst not in seen_nodes_in_match:
            seen_nodes_in_match.append(dst)
        if e not in seen_edges_in_match:
            seen_edges_in_match.append(e)

    # Build FILTER WHERE predicates (as-is, but expand BETWEEN)
    preds = _split_where_predicates(where_sql)
    preds = _expand_between_predicates(preds)

    lines: List[str] = []
    lines.append("MATCH \n" + ",\n".join(match_parts))
    lines.append("FILTER WHERE")
    for i, p in enumerate(preds):
        lines.append(f"{p}")

    # RETURN nodes then edges (in appearance order), then any remaining aliases
    node_alias_list = seen_nodes_in_match + [n for n in sorted(node_aliases) if n not in set(seen_nodes_in_match)]
    edge_alias_list = seen_edges_in_match + [
        e for e in sorted(a for a, t in alias_to_table.items() if "_" in t) if e not in set(seen_edges_in_match)
    ]
    ret = ", ".join(node_alias_list + edge_alias_list)
    lines.append(f"RETURN {ret}; -- || {truecard}")
    return lines


# GQL解析的正则表达式
_RE_GQL_NODE = re.compile(r"\(([a-zA-Z_][a-zA-Z0-9_]*):([a-zA-Z_][a-zA-Z0-9_]*)\)")
_RE_GQL_EDGE = re.compile(r"\[([a-zA-Z_][a-zA-Z0-9_]*):([a-zA-Z_][a-zA-Z0-9_]*)\]")
_RE_GQL_PREDICATE = re.compile(
    r"([a-zA-Z_][a-zA-Z0-9_]*)\s*\.\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*(>=|<=|>|<|!=|<>|=|LIKE|IS\s+NULL|IS\s+NOT\s+NULL)\s*(.*)$",
    re.IGNORECASE,
)


def _parse_gql_query(gql_text: str) -> Tuple[Dict[str, str], Dict[str, Tuple[str, str, str]], List[str]]:
    """
    解析GQL查询，提取顶点、边和谓词信息。
    返回: (alias_to_label, edge_info, predicates)
    """
    alias_to_label: Dict[str, str] = {}
    edge_info: Dict[str, Tuple[str, str, str]] = {}
    predicates: List[str] = []
    
    lines = gql_text.splitlines()
    in_match = False
    in_filter = False
    match_lines = []
    
    for line in lines:
        line_stripped = line.strip()
        if not line_stripped or line_stripped.startswith("--"):
            continue
        
        if line_stripped.upper().startswith("MATCH"):
            in_match = True
            in_filter = False
            # MATCH后面可能还有内容，也可能没有
            match_content = line_stripped[5:].strip()
            match_lines = [match_content] if match_content else []
            continue
        
        if in_match:
            if line_stripped.upper().startswith("FILTER"):
                in_match = False
                in_filter = True
                # 合并所有MATCH行，保留逗号以便分割
                match_text = " ".join([m for m in match_lines if m.strip()])
                if match_text:
                    _parse_match_clause(match_text, alias_to_label, edge_info)
                continue
            # 继续收集MATCH行，保留逗号（不去掉）
            match_lines.append(line_stripped)
            continue
        
        if in_filter:
            if line_stripped.upper().startswith("RETURN"):
                in_filter = False
                continue
            pred = re.sub(r"^\s*AND\s+", "", line_stripped, flags=re.IGNORECASE)
            if pred:
                predicates.append(pred)
    
    return alias_to_label, edge_info, predicates


def _parse_match_clause(match_text: str, alias_to_label: Dict[str, str], edge_info: Dict[str, Tuple[str, str, str]]) -> None:
    """解析MATCH子句"""
    patterns = [p.strip() for p in match_text.split(",") if p.strip()]
    
    for pattern in patterns:
        # 查找所有节点: (alias:Label)
        nodes = _RE_GQL_NODE.findall(pattern)
        for alias, label in nodes:
            alias_to_label[alias] = label
        
        # 查找边: [edge_alias:rel_type]
        edge_match = _RE_GQL_EDGE.search(pattern)
        if edge_match:
            edge_alias, rel_type = edge_match.groups()
            # 找到边前后的节点
            parts = re.split(r"->|<-", pattern)
            if len(parts) == 2:
                src_match = _RE_GQL_NODE.search(parts[0].strip())
                dst_match = _RE_GQL_NODE.search(parts[1].strip())
                if src_match and dst_match:
                    src_alias = src_match.group(1)
                    dst_alias = dst_match.group(1)
                    if "->" in pattern:
                        edge_info[edge_alias] = (src_alias, dst_alias, rel_type)
                    elif "<-" in pattern:
                        edge_info[edge_alias] = (dst_alias, src_alias, rel_type)


def _parse_predicate_to_json(pred_str: str, edge_aliases: Set[str]) -> Optional[Dict]:
    """解析单个谓词字符串，返回JSON格式的谓词对象"""
    pred_str = pred_str.strip()
    if not pred_str:
        return None
    
    m = _RE_GQL_PREDICATE.match(pred_str)
    if not m:
        return None
    
    alias, property_name, op, value = m.groups()
    target = "edge" if alias in edge_aliases else "vertex"
    
    op_map = {
        "=": "eq",
        "!=": "ne",
        "<>": "ne",
        ">": "gt",
        ">=": "ge",
        "<": "lt",
        "<=": "le",
        "like": "like",
        "is null": "is_null",
        "is not null": "is_not_null",
    }
    op_normalized = op_map.get(op.lower().strip(), op.lower())
    
    value = value.strip()
    value_obj = {}
    
    if value.startswith("'") and value.endswith("'"):
        str_value = value[1:-1].replace("''", "'")
        value_obj = {"String": str_value}
    elif value.startswith('"') and value.endswith('"'):
        str_value = value[1:-1].replace('""', '"')
        value_obj = {"String": str_value}
    elif _is_number_literal(value):
        try:
            if "." in value:
                value_obj = {"Float64": float(value)}
            else:
                value_obj = {"Int64": int(value)}
        except ValueError:
            value_obj = {"String": value}
    elif value.upper() in ("TRUE", "FALSE"):
        value_obj = {"Boolean": value.upper() == "TRUE"}
    else:
        value_obj = {"String": value}
    
    return {
        "alias": alias,
        "property": property_name,
        "op": op_normalized,
        "value": value_obj,
    }


def _gql_to_json(gql_lines: List[str], query_id: str, truecard: Optional[int] = None) -> Dict:
    """
    将GQL查询行转换为JSON格式。
    """
    gql_text = "\n".join(gql_lines)
    alias_to_label, edge_info, predicates = _parse_gql_query(gql_text)
    
    # 构建顶点列表（只包含节点，不包含边）
    vertices = []
    node_alias_to_id: Dict[str, int] = {}
    edge_aliases = set(edge_info.keys())
    
    # 只添加节点别名（不在边别名集合中的）
    for idx, (alias, label) in enumerate(sorted(alias_to_label.items()), 1):
        if alias not in edge_aliases:
            vertices.append({"id": idx, "label": label.lower()})
            node_alias_to_id[alias] = idx
    
    # 构建边列表
    edges = []
    edge_alias_to_id: Dict[str, int] = {}
    for idx, (edge_alias, (src_alias, dst_alias, rel_type)) in enumerate(sorted(edge_info.items()), 1):
        src_id = node_alias_to_id.get(src_alias)
        dst_id = node_alias_to_id.get(dst_alias)
        if src_id is None or dst_id is None:
            continue
        
        # 构建边标签: SrcLabel_RelType_DstLabel
        src_label = alias_to_label.get(src_alias, "")
        dst_label = alias_to_label.get(dst_alias, "")
        
        # 从rel_type中提取中间的关系部分
        # rel_type格式可能是: comment_hascreator_person 或 hascreator
        rel_parts = rel_type.split("_")
        if len(rel_parts) >= 3:
            # 格式: src_rel_dst，提取中间部分
            rel_mid = "_".join(rel_parts[1:-1])
        else:
            # 格式: rel，直接使用
            rel_mid = rel_type
        
        # 转换为Title Case格式（每个单词首字母大写）
        rel_mid_parts = rel_mid.split("_")
        rel_title = "_".join(p.capitalize() for p in rel_mid_parts)
        
        edge_label = f"{src_label}_{rel_title}_{dst_label}".lower()
        
        edges.append({
            "id": idx,
            "label": edge_label,
            "src": src_id,
            "dst": dst_id,
        })
        edge_alias_to_id[edge_alias] = idx
    
    # 构建谓词列表
    pred_list = []
    predicate_id = 1
    for pred_str in predicates:
        parsed = _parse_predicate_to_json(pred_str, edge_aliases)
        if not parsed:
            continue
        
        alias = parsed["alias"]
        # 根据别名判断是点还是边
        target = "edge" if alias in edge_aliases else "vertex"
        
        if target == "vertex":
            entity_id = node_alias_to_id.get(alias)
        else:
            entity_id = edge_alias_to_id.get(alias)
        
        if entity_id is None:
            continue
        
        pred_list.append({
            "predicate_id": predicate_id,
            "target": target,
            "id": entity_id,
            "property": parsed["property"],
            "op": parsed["op"],
            "value": parsed["value"],
        })
        predicate_id += 1
    
    result = {
        "vertices": vertices,
        "edges": edges,
        "predicates": pred_list,
    }
    
    return result


def convert_one_sql_file(sql_path: Path) -> Tuple[Path, List[Path]]:
    sql_text = sql_path.read_text(encoding="utf-8")
    join = _parse_join_info(sql_text)
    alias_to_table = join.alias_to_table

    node_aliases, edge_endpoints = _build_entity_graph(join)
    node_order, edge_order = _euler_walk_nodes_edges(node_aliases, edge_endpoints)

    # Precompute render label for each alias
    node_label: Dict[str, str] = {}
    edge_label: Dict[str, str] = {}
    for a, t in alias_to_table.items():
        if _is_edge_alias(a, alias_to_table):
            edge_label[a] = _rel_from_edge_table(t)
        else:
            node_label[a] = _label_from_table(t)

    out_lines: List[str] = []
    json_paths: List[Path] = []
    current_qid: Optional[str] = None
    auto_idx = 0

    for ln in sql_text.splitlines():
        m = re.match(r"^\s*--\s*([a-zA-Z0-9_]+)\s*$", ln)
        if m:
            current_qid = m.group(1)
            continue
        if not re.match(r"^\s*WHERE\b", ln, re.IGNORECASE):
            continue

        where_sql, truecard = _parse_where_and_truecard(ln)
        if truecard is None:
            continue

        auto_idx += 1
        qid = current_qid or f"{sql_path.stem}_p{auto_idx:03d}"

        gql_lines = [f"-- {qid}"]
        gql_lines.extend(
            _format_cypher_query(
                join=join,
                node_aliases=node_aliases,
                edge_endpoints=edge_endpoints,
                edge_order=edge_order,
                where_sql=where_sql,
                truecard=truecard,
            )
        )
        gql_lines.append("")
        
        out_lines.extend(gql_lines)
        
        # 为每个查询生成独立的JSON文件
        json_query = _gql_to_json(gql_lines, qid, truecard)
        json_path = sql_path.with_name(f"{qid}.json")
        json_path.write_text(json.dumps(json_query, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        json_paths.append(json_path)

    out_path = sql_path.with_name(sql_path.name.replace("_predicates_100.sql", "_predicates_100_gql.txt"))
    out_path.write_text("\n".join(out_lines).rstrip() + "\n", encoding="utf-8")
    
    return out_path, json_paths


def iter_sql_files(root: Path) -> Iterable[Path]:
    yield from root.glob("**/*_predicates_100.sql")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parent,
        help="Benchmark root directory (default: directory containing this script).",
    )
    args = ap.parse_args()

    root: Path = args.root
    sql_files = sorted(iter_sql_files(root))
    if not sql_files:
        raise SystemExit(f"No *_predicates_100.sql found under {root}")

    for p in sql_files:
        gql_path, json_paths = convert_one_sql_file(p)
        print(f"[ok] {p.relative_to(root)} -> {gql_path.name}, {len(json_paths)} JSON files")


if __name__ == "__main__":
    main()

