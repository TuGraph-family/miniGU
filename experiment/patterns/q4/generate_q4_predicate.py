#!/usr/bin/env python3
import csv
import os
import random
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Tuple


DB = os.environ.get("PGDATABASE", "new")
USER = os.environ.get("PGUSER", "postgres")
PASSWORD = os.environ.get("PGPASSWORD", "zxz2024")

SQL_OUT = Path("/home/zxz/benchmark/q4/q4_predicates_100.sql")
CSV_OUT = Path("/home/zxz/benchmark/q4/q4_predicates_100.csv")

RNG_SEED = int(os.environ.get("Q4_PRED_SEED", "20260115"))
TARGET = int(os.environ.get("Q4_PRED_TARGET", "100"))
BATCH = int(os.environ.get("Q4_PRED_BATCH", "300"))
MAX_EVAL = int(os.environ.get("Q4_PRED_MAX_EVAL", "15000"))


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


@dataclass(frozen=True)
class Atom:
    group: str
    sql: str
    base_sql: str
    kind: str


# Message is modeled as post (alias m) as in q4.sql
BASE_SELECT = """SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
"""

BASE_FROM = """FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
"""

BASE_CTE = (
    "WITH base AS (\n"
    "  SELECT\n"
    "    -- node attrs\n"
    "    m.explicitlydeleted   AS m_explicitlydeleted,\n"
    "    m.creationdate        AS m_creationdate,\n"
    "    m.length              AS m_length,\n"
    "    m.language            AS m_language,\n"
    "    m.browserused         AS m_browserused,\n"
    "    c.explicitlydeleted   AS c_explicitlydeleted,\n"
    "    c.creationdate        AS c_creationdate,\n"
    "    c.length              AS c_length,\n"
    "    c.browserused         AS c_browserused,\n"
    "    t.name                AS t_name,\n"
    "    creator.id            AS creator_id,\n"
    "    creator.gender        AS creator_gender,\n"
    "    creator.language      AS creator_language,\n"
    "    creator.browserused   AS creator_browserused,\n"
    "    liker.id              AS liker_id,\n"
    "    liker.gender          AS liker_gender,\n"
    "    liker.language        AS liker_language,\n"
    "    liker.browserused     AS liker_browserused,\n"
    "    -- edge attrs\n"
    "    pht.creationdate      AS pht_creationdate,\n"
    "    php.creationdate      AS php_creationdate,\n"
    "    php.explicitlydeleted AS php_explicitlydeleted,\n"
    "    plp.creationdate      AS plp_creationdate,\n"
    "    plp.explicitlydeleted AS plp_explicitlydeleted,\n"
    "    crp.creationdate      AS crp_creationdate,\n"
    "    crp.explicitlydeleted AS crp_explicitlydeleted\n"
    f"{BASE_FROM}"
    ")\n"
)


def fetch_distributions() -> Dict[str, object]:
    d: Dict[str, object] = {}

    d["genders"] = [r[0] for r in psql_rows("SELECT gender, count(*) FROM person GROUP BY gender ORDER BY count(*) DESC;") if r and r[0] != ""]
    d["person_langs"] = [r[0] for r in psql_rows("SELECT language, count(*) FROM person GROUP BY language ORDER BY count(*) DESC LIMIT 20;") if r and r[0] != ""]
    d["post_langs"] = [r[0] for r in psql_rows("SELECT language, count(*) FROM post GROUP BY language ORDER BY count(*) DESC LIMIT 20;") if r]

    d["person_browsers"] = [r[0] for r in psql_rows("SELECT browserused, count(*) FROM person GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;") if r and r[0] != ""]
    d["post_browsers"] = [r[0] for r in psql_rows("SELECT browserused, count(*) FROM post GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;") if r and r[0] != ""]
    d["comment_browsers"] = [r[0] for r in psql_rows("SELECT browserused, count(*) FROM comment GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;") if r and r[0] != ""]

    # tags used on posts (Q4 path)
    d["hot_post_tags"] = [r[0] for r in psql_rows(
        "SELECT t.name, count(*) FROM post_hastag_tag pht JOIN tag t ON t.id=pht.tagid "
        "GROUP BY t.name ORDER BY count(*) DESC LIMIT 60;"
    ) if r and r[0] != ""]

    # lengths
    len_rows = psql_rows(
        "SELECT 'post_len' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY length)::text FROM post "
        "UNION ALL "
        "SELECT 'comment_len' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY length)::text FROM comment;"
    )
    for k, arr in len_rows:
        d[k] = parse_pg_array_numbers(arr)

    # creationdate percentiles for nodes/edges used in Q4
    cd_rows = psql_rows(
        "SELECT 'post_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM post "
        "UNION ALL "
        "SELECT 'comment_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM comment "
        "UNION ALL "
        "SELECT 'plp_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM person_likes_post "
        "UNION ALL "
        "SELECT 'php_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM post_hascreator_person "
        "UNION ALL "
        "SELECT 'crp_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM comment_replyof_post "
        "UNION ALL "
        "SELECT 'pht_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM post_hastag_tag;"
    )
    for k, arr in cd_rows:
        d[k] = parse_pg_array_numbers(arr)

    return d


def build_atoms(d: Dict[str, object]) -> List[Atom]:
    atoms: List[Atom] = []

    # message(post) predicates
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

    # tag predicates (popular on posts)
    for name in d["hot_post_tags"][:25]:
        atoms.append(Atom("t", f"t.name = {sh_quote_sql(name)}", f"t_name = {sh_quote_sql(name)}", "tag"))

    # creator / liker predicates
    for g in d["genders"][:2]:
        atoms.append(Atom("creator", f"creator.gender = {sh_quote_sql(g)}", f"creator_gender = {sh_quote_sql(g)}", "demographic"))
        atoms.append(Atom("liker", f"liker.gender = {sh_quote_sql(g)}", f"liker_gender = {sh_quote_sql(g)}", "demographic"))

    for lang in d["person_langs"][:12]:
        atoms.append(Atom("creator", f"creator.language = {sh_quote_sql(lang)}", f"creator_language = {sh_quote_sql(lang)}", "language"))
        atoms.append(Atom("liker", f"liker.language = {sh_quote_sql(lang)}", f"liker_language = {sh_quote_sql(lang)}", "language"))

    for b in d["person_browsers"][:8]:
        atoms.append(Atom("creator", f"creator.browserused = {sh_quote_sql(b)}", f"creator_browserused = {sh_quote_sql(b)}", "client"))
        atoms.append(Atom("liker", f"liker.browserused = {sh_quote_sql(b)}", f"liker_browserused = {sh_quote_sql(b)}", "client"))

    # edge predicates: likes/hasCreator/replyOf/hasTag
    atoms.append(Atom("php", "php.explicitlydeleted = false", "php_explicitlydeleted = false", "lifecycle"))
    atoms.append(Atom("plp", "plp.explicitlydeleted = false", "plp_explicitlydeleted = false", "lifecycle"))
    atoms.append(Atom("crp", "crp.explicitlydeleted = false", "crp_explicitlydeleted = false", "lifecycle"))

    php_cd = d["php_cd"]
    plp_cd = d["plp_cd"]
    crp_cd = d["crp_cd"]
    pht_cd = d["pht_cd"]
    atoms += [
        Atom("php", f"php.creationdate BETWEEN {int(php_cd[1])} AND {int(php_cd[4])}", f"php_creationdate BETWEEN {int(php_cd[1])} AND {int(php_cd[4])}", "time"),
        Atom("plp", f"plp.creationdate >= {int(plp_cd[2])}", f"plp_creationdate >= {int(plp_cd[2])}", "time"),
        Atom("crp", f"crp.creationdate BETWEEN {int(crp_cd[1])} AND {int(crp_cd[4])}", f"crp_creationdate BETWEEN {int(crp_cd[1])} AND {int(crp_cd[4])}", "time"),
        Atom("pht", f"pht.creationdate BETWEEN {int(pht_cd[1])} AND {int(pht_cd[3])}", f"pht_creationdate BETWEEN {int(pht_cd[1])} AND {int(pht_cd[3])}", "time"),
    ]

    return atoms


def choose_predicates_balanced(
    atoms_by_group: Dict[str, List[Atom]],
    group_counts: Dict[str, int],
    kind_counts: Dict[str, int],
    combo_counts: Dict[Tuple[str, ...], int],
) -> List[Atom]:
    node_groups = ["m", "c", "t", "creator", "liker"]
    edge_groups = ["pht", "php", "plp", "crp"]
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

        # encourage edge coverage
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


def q4_base_count() -> int:
    q = f"SELECT count(*) FROM (SELECT 1 {BASE_FROM}) q;"
    return int(psql_at(q))


def main() -> None:
    random.seed(RNG_SEED)
    d = fetch_distributions()
    atoms = build_atoms(d)

    atoms_by_group: Dict[str, List[Atom]] = {}
    for a in atoms:
        atoms_by_group.setdefault(a.group, []).append(a)

    all_groups = ["m", "c", "t", "creator", "liker", "pht", "php", "plp", "crp"]
    for g in all_groups:
        atoms_by_group.setdefault(g, [])

    group_counts: Dict[str, int] = {g: 0 for g in atoms_by_group.keys()}
    kind_counts: Dict[str, int] = {}
    combo_counts: Dict[Tuple[str, ...], int] = {}

    base_cnt = q4_base_count()
    if base_cnt <= 0:
        raise SystemExit(f"Unexpected Q4 base cardinality: {base_cnt}")

    print(f"[init] db={DB} user={USER} seed={RNG_SEED} target={TARGET} batch={BATCH} max_eval={MAX_EVAL}")
    print(f"[init] Q4 base true_cardinality={base_cnt}")
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
    sql_lines.append(f"-- Q4 base true_cardinality: {base_cnt}")
    sql_lines.append(f"-- Generated with seed={RNG_SEED}")
    sql_lines.append("")
    for idx, preds, cnt in accepted:
        pid = f"q4_p{idx:03d}"
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

