#!/usr/bin/env python3
import csv
import os
import random
import re
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Tuple


DB = os.environ.get("PGDATABASE", "new")
USER = os.environ.get("PGUSER", "postgres")
PASSWORD = os.environ.get("PGPASSWORD", "zxz2024")

SQL_OUT = Path("/home/zxz/benchmark/q5/q5_predicates_100.sql")
CSV_OUT = Path("/home/zxz/benchmark/q5/q5_predicates_100.csv")

RNG_SEED = int(os.environ.get("Q5_PRED_SEED", "20260115"))
TARGET = int(os.environ.get("Q5_PRED_TARGET", "100"))
BATCH = int(os.environ.get("Q5_PRED_BATCH", "300"))
MAX_EVAL = int(os.environ.get("Q5_PRED_MAX_EVAL", "15000"))


def sh_quote_sql(s: str) -> str:
    return "'" + s.replace("'", "''") + "'"


def psql_at(sql: str) -> str:
    env = os.environ.copy()
    env["PGPASSWORD"] = PASSWORD
    cmd = ["psql", "-U", USER, "-d", DB, "-v", "ON_ERROR_STOP=1", "-At", "-c", sql]
    out = subprocess.check_output(cmd, env=env, text=True)
    return out.strip()


def psql_rows(sql: str) -> List[List[str]]:
    out = psql_at(sql)
    if not out:
        return []
    return [line.split("|") for line in out.splitlines()]


def parse_pg_array_numbers(arr: str) -> List[float]:
    arr = arr.strip()
    if not (arr.startswith("{") and arr.endswith("}")):
        raise ValueError(f"Unexpected array format: {arr}")
    inner = arr[1:-1].strip()
    if inner == "":
        return []
    return [float(x) for x in inner.split(",")]

def is_safebound_supported_predicate(sql: str) -> bool:
    s = sql.strip()
    if "<>" in s or "!=" in s:
        return False
    if re.search(r"\b\w+\.\w+\s*(=|<=|>=|<|>)\s*\w+\.\w+\b", s):
        return False
    return True


@dataclass(frozen=True)
class Atom:
    group: str
    sql: str
    base_sql: str
    kind: str


BASE_SELECT = """SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
"""

BASE_FROM = """FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
"""

BASE_CTE = (
    "WITH base AS (\n"
    "  SELECT\n"
    "    -- node attrs\n"
    "    m.explicitlydeleted AS m_explicitlydeleted,\n"
    "    m.creationdate      AS m_creationdate,\n"
    "    m.length            AS m_length,\n"
    "    m.language          AS m_language,\n"
    "    m.browserused       AS m_browserused,\n"
    "    c.explicitlydeleted AS c_explicitlydeleted,\n"
    "    c.creationdate      AS c_creationdate,\n"
    "    c.length            AS c_length,\n"
    "    c.browserused       AS c_browserused,\n"
    "    t1.name             AS t1_name,\n"
    "    t2.name             AS t2_name,\n"
    "    -- edge attrs\n"
    "    pht.creationdate    AS pht_creationdate,\n"
    "    crp.creationdate    AS crp_creationdate,\n"
    "    crp.explicitlydeleted AS crp_explicitlydeleted,\n"
    "    cht.creationdate    AS cht_creationdate\n"
    f"{BASE_FROM}"
    ")\n"
)


def fetch_distributions() -> Dict[str, object]:
    d: Dict[str, object] = {}

    d["post_langs"] = [r[0] for r in psql_rows("SELECT language, count(*) FROM post GROUP BY language ORDER BY count(*) DESC LIMIT 20;") if r]
    d["post_browsers"] = [r[0] for r in psql_rows("SELECT browserused, count(*) FROM post GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;") if r and r[0] != ""]
    d["comment_browsers"] = [r[0] for r in psql_rows("SELECT browserused, count(*) FROM comment GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;") if r and r[0] != ""]

    # popular tags in the actual edges (post tags and comment tags)
    d["hot_post_tags"] = [r[0] for r in psql_rows(
        "SELECT t.name, count(*) FROM post_hastag_tag pht JOIN tag t ON t.id=pht.tagid "
        "GROUP BY t.name ORDER BY count(*) DESC LIMIT 80;"
    ) if r and r[0] != ""]
    d["hot_comment_tags"] = [r[0] for r in psql_rows(
        "SELECT t.name, count(*) FROM comment_hastag_tag cht JOIN tag t ON t.id=cht.tagid "
        "GROUP BY t.name ORDER BY count(*) DESC LIMIT 80;"
    ) if r and r[0] != ""]

    # lengths
    len_rows = psql_rows(
        "SELECT 'post_len' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY length)::text FROM post "
        "UNION ALL "
        "SELECT 'comment_len' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY length)::text FROM comment;"
    )
    for k, arr in len_rows:
        d[k] = parse_pg_array_numbers(arr)

    # creationdate percentiles for nodes/edges used in Q5
    cd_rows = psql_rows(
        "SELECT 'post_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM post "
        "UNION ALL "
        "SELECT 'comment_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM comment "
        "UNION ALL "
        "SELECT 'pht_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM post_hastag_tag "
        "UNION ALL "
        "SELECT 'cht_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM comment_hastag_tag "
        "UNION ALL "
        "SELECT 'crp_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM comment_replyof_post;"
    )
    for k, arr in cd_rows:
        d[k] = parse_pg_array_numbers(arr)

    return d


def build_atoms(d: Dict[str, object]) -> List[Atom]:
    atoms: List[Atom] = []

    # post predicates
    atoms.append(Atom("m", "m.explicitlydeleted = false", "m_explicitlydeleted = false", "lifecycle"))
    pcd = d["post_cd"]
    atoms += [
        Atom("m", f"m.creationdate >= {int(pcd[2])}", f"m_creationdate >= {int(pcd[2])}", "time"),
        Atom("m", f"m.creationdate BETWEEN {int(pcd[1])} AND {int(pcd[3])}", f"m_creationdate BETWEEN {int(pcd[1])} AND {int(pcd[3])}", "time"),
    ]
    plen = d["post_len"]
    atoms += [
        Atom("m", "m.length > 0", "m_length > 0", "content"),
        Atom("m", f"m.length BETWEEN {int(plen[0])} AND {int(plen[4])}", f"m_length BETWEEN {int(plen[0])} AND {int(plen[4])}", "content"),
    ]
    for lang in [x for x in d["post_langs"] if x != ""][:10]:
        atoms.append(Atom("m", f"m.language = {sh_quote_sql(lang)}", f"m_language = {sh_quote_sql(lang)}", "language"))
    for b in d["post_browsers"][:8]:
        atoms.append(Atom("m", f"m.browserused = {sh_quote_sql(b)}", f"m_browserused = {sh_quote_sql(b)}", "client"))

    # comment predicates
    atoms.append(Atom("c", "c.explicitlydeleted = false", "c_explicitlydeleted = false", "lifecycle"))
    ccd = d["comment_cd"]
    atoms += [
        Atom("c", f"c.creationdate >= {int(ccd[2])}", f"c_creationdate >= {int(ccd[2])}", "time"),
        Atom("c", f"c.creationdate BETWEEN {int(ccd[1])} AND {int(ccd[4])}", f"c_creationdate BETWEEN {int(ccd[1])} AND {int(ccd[4])}", "time"),
    ]
    clen = d["comment_len"]
    atoms += [
        Atom("c", f"c.length <= {int(clen[1])}", f"c_length <= {int(clen[1])}", "content"),
        Atom("c", f"c.length BETWEEN {int(clen[2])} AND {int(clen[3])}", f"c_length BETWEEN {int(clen[2])} AND {int(clen[3])}", "content"),
        Atom("c", f"c.length >= {int(clen[3])}", f"c_length >= {int(clen[3])}", "content"),
    ]
    for b in d["comment_browsers"][:8]:
        atoms.append(Atom("c", f"c.browserused = {sh_quote_sql(b)}", f"c_browserused = {sh_quote_sql(b)}", "client"))

    # tag1/tag2 predicates: pick from real popularity on post vs comment edges
    for name in d["hot_post_tags"][:25]:
        atoms.append(Atom("t1", f"t1.name = {sh_quote_sql(name)}", f"t1_name = {sh_quote_sql(name)}", "tag"))
    for name in d["hot_comment_tags"][:25]:
        atoms.append(Atom("t2", f"t2.name = {sh_quote_sql(name)}", f"t2_name = {sh_quote_sql(name)}", "tag"))

    # edge predicates
    atoms.append(Atom("crp", "crp.explicitlydeleted = false", "crp_explicitlydeleted = false", "lifecycle"))
    pht_cd = d["pht_cd"]
    cht_cd = d["cht_cd"]
    crp_cd = d["crp_cd"]
    atoms += [
        Atom("pht", f"pht.creationdate BETWEEN {int(pht_cd[1])} AND {int(pht_cd[4])}", f"pht_creationdate BETWEEN {int(pht_cd[1])} AND {int(pht_cd[4])}", "time"),
        Atom("cht", f"cht.creationdate BETWEEN {int(cht_cd[1])} AND {int(cht_cd[4])}", f"cht_creationdate BETWEEN {int(cht_cd[1])} AND {int(cht_cd[4])}", "time"),
        Atom("crp", f"crp.creationdate BETWEEN {int(crp_cd[1])} AND {int(crp_cd[4])}", f"crp_creationdate BETWEEN {int(crp_cd[1])} AND {int(crp_cd[4])}", "time"),
    ]

    atoms = [a for a in atoms if is_safebound_supported_predicate(a.sql)]
    return atoms


def choose_predicates_balanced(
    atoms_by_group: Dict[str, List[Atom]],
    group_counts: Dict[str, int],
    kind_counts: Dict[str, int],
    combo_counts: Dict[Tuple[str, ...], int],
) -> List[Atom]:
    node_groups = ["m", "c", "t1", "t2"]
    edge_groups = ["pht", "cht", "crp"]
    all_groups = [g for g in atoms_by_group.keys() if atoms_by_group.get(g)]

    def weight_for_group(g: str) -> float:
        return 1.0 / (1.0 + group_counts.get(g, 0))

    def pick_group(pool: List[str], avoid: set) -> str:
        candidates = [g for g in pool if g not in avoid and atoms_by_group.get(g)]
        if not candidates:
            return ""
        weights = [weight_for_group(g) for g in candidates]
        return random.choices(candidates, weights=weights, k=1)[0]

    def pick_atom_for_group(g: str) -> Atom:
        candidates = atoms_by_group[g]
        candidates_sorted = sorted(candidates, key=lambda a: kind_counts.get(a.kind, 0))
        top = candidates_sorted[: max(6, len(candidates_sorted) // 3)]
        return random.choice(top)

    def score(groups: List[str], atoms: List[Atom]) -> float:
        key = tuple(sorted(set(groups)))
        combo_pen = combo_counts.get(key, 0)
        group_pen = sum(group_counts.get(g, 0) for g in set(groups))
        kind_pen = sum(kind_counts.get(a.kind, 0) for a in atoms)
        return combo_pen * 50.0 + group_pen * 3.0 + kind_pen * 1.0

    best: List[Atom] = []
    best_score = float("inf")

    for _ in range(40):
        k = 2 if random.random() < 0.55 else 3
        chosen: List[str] = []
        used: set = set()

        # require at least one edge predicate most of the time
        require_edge = (random.random() < 0.90)
        g_edge = pick_group(edge_groups if require_edge else all_groups, used)
        if g_edge:
            chosen.append(g_edge)
            used.add(g_edge)

        # include at least one node predicate
        g_node = pick_group(node_groups, used)
        if g_node:
            chosen.append(g_node)
            used.add(g_node)

        while len(chosen) < k:
            g = pick_group(all_groups, used)
            if not g:
                break
            chosen.append(g)
            used.add(g)

        # prefer including both t1 and t2 sometimes to cover tag1/tag2 combinations
        if ("t1" in chosen) ^ ("t2" in chosen):
            if random.random() < 0.70:
                missing = "t2" if "t1" in chosen else "t1"
                if missing not in used and atoms_by_group.get(missing):
                    chosen[-1] = missing
                    used.add(missing)

        atoms = [pick_atom_for_group(g) for g in chosen if g]
        uniq: Dict[str, Atom] = {}
        for a in atoms:
            uniq[a.sql] = a
        atoms2 = list(uniq.values())
        if len(atoms2) < 2 or len(atoms2) > 3:
            continue

        s = score(chosen, atoms2)
        if s < best_score:
            best_score = s
            best = atoms2

    if not best:
        candidates = [a for g in all_groups for a in atoms_by_group[g]]
        best = random.sample(candidates, k=2)
    return best


def batch_counts(where_base_sql_list: List[str]) -> Tuple[int, List[int]]:
    selects = ["count(*) AS base_cnt"]
    for i, cond in enumerate(where_base_sql_list, start=1):
        alias = f"c{i:03d}"
        selects.append(f"count(*) FILTER (WHERE {cond}) AS {alias}")
    q = BASE_CTE + "SELECT " + ", ".join(selects) + " FROM base;"
    row = psql_at(q)
    parts = row.split("|") if row else []
    if len(parts) != 1 + len(where_base_sql_list):
        raise RuntimeError(f"Unexpected batch_counts output columns: got {len(parts)} expected {1+len(where_base_sql_list)}")
    base_cnt = int(parts[0])
    counts = [int(x) for x in parts[1:]]
    return base_cnt, counts


def q5_base_count() -> int:
    q = f"SELECT count(*) FROM (SELECT 1 {BASE_FROM}) q;"
    return int(psql_at(q))


def main() -> None:
    random.seed(RNG_SEED)
    d = fetch_distributions()
    atoms = build_atoms(d)

    atoms_by_group: Dict[str, List[Atom]] = {}
    for a in atoms:
        atoms_by_group.setdefault(a.group, []).append(a)

    all_groups = ["m", "c", "t1", "t2", "pht", "cht", "crp"]
    for g in all_groups:
        atoms_by_group.setdefault(g, [])

    group_counts: Dict[str, int] = {g: 0 for g in atoms_by_group.keys()}
    kind_counts: Dict[str, int] = {}
    combo_counts: Dict[Tuple[str, ...], int] = {}

    base_cnt = q5_base_count()
    if base_cnt <= 0:
        raise SystemExit(f"Unexpected Q5 base cardinality: {base_cnt}")

    print(f"[init] db={DB} user={USER} seed={RNG_SEED} target={TARGET} batch={BATCH} max_eval={MAX_EVAL}")
    print(f"[init] Q5 base true_cardinality={base_cnt}")
    print(f"[init] atoms_total={len(atoms)} groups={len(atoms_by_group)}")

    accepted: List[Tuple[int, List[Atom], int]] = []
    seen_where: set[str] = set()

    evaluated = 0
    batch_no = 0
    while len(accepted) < TARGET and evaluated < MAX_EVAL:
        batch_no += 1
        candidates: List[Tuple[List[Atom], str, str]] = []
        while len(candidates) < BATCH and evaluated < MAX_EVAL:
            preds = choose_predicates_balanced(atoms_by_group, group_counts, kind_counts, combo_counts)
            if not (2 <= len(preds) <= 3):
                continue
            join_where = " AND ".join(f"({p.sql})" for p in preds)
            if join_where in seen_where:
                continue
            seen_where.add(join_where)
            base_where = " AND ".join(f"({p.base_sql})" for p in preds)
            candidates.append((preds, join_where, base_where))
            evaluated += 1

        if not candidates:
            break

        base_cnt2, counts = batch_counts([c[2] for c in candidates])
        if base_cnt2 != base_cnt:
            base_cnt = base_cnt2

        newly = 0
        for (preds, _, _), cnt in zip(candidates, counts):
            if cnt <= 0:
                continue
            if len(accepted) >= TARGET:
                break
            idx = len(accepted) + 1
            accepted.append((idx, preds, cnt))
            newly += 1
            combo_key = tuple(sorted({p.group for p in preds}))
            combo_counts[combo_key] = combo_counts.get(combo_key, 0) + 1
            for p in preds:
                group_counts[p.group] = group_counts.get(p.group, 0) + 1
                kind_counts[p.kind] = kind_counts.get(p.kind, 0) + 1

        rate = (len(accepted) / evaluated) if evaluated else 0.0
        print(f"[batch {batch_no}] +{newly} accepted, total={len(accepted)}/{TARGET} (evaluated={evaluated}, pass_rate={rate:.3f})")

    if len(accepted) < TARGET:
        raise SystemExit(f"Only collected {len(accepted)} valid queries after evaluated={evaluated}.")

    # write combined SQL
    sql_lines: List[str] = []
    sql_lines.append(f"-- Q5 base true_cardinality: {base_cnt}")
    sql_lines.append(f"-- Generated with seed={RNG_SEED}")
    sql_lines.append("")
    for idx, preds, cnt in accepted:
        pid = f"q5_p{idx:03d}"
        preds_str = " AND ".join(p.sql for p in preds)
        sql_lines.append(f"-- {pid}")
        sql_lines.append(f"-- predicates: {preds_str}")
        sql_lines.append(BASE_SELECT.rstrip())
        sql_lines.append(BASE_FROM.rstrip())
        sql_lines.append(f"WHERE {preds_str}; -- || {cnt}")
        sql_lines.append("")
    SQL_OUT.write_text("\n".join(sql_lines) + "\n")

    # write CSV
    with CSV_OUT.open("w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["predicate1", "predicate2", "predicate3", "truecard", "selectivity"])
        for _, preds, cnt in accepted:
            psqls = [p.sql for p in preds]
            while len(psqls) < 3:
                psqls.append("")
            sel = cnt / base_cnt
            w.writerow([psqls[0], psqls[1], psqls[2], cnt, f"{sel:.10f}"])

    print(f"Wrote {TARGET} queries to {SQL_OUT}")
    print(f"Wrote CSV to {CSV_OUT}")


if __name__ == "__main__":
    main()

