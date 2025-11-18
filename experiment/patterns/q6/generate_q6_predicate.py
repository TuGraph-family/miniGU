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

SQL_OUT = Path("/home/zxz/benchmark/q6/q6_predicates_100.sql")
CSV_OUT = Path("/home/zxz/benchmark/q6/q6_predicates_100.csv")

RNG_SEED = int(os.environ.get("Q6_PRED_SEED", "20260115"))
TARGET = int(os.environ.get("Q6_PRED_TARGET", "100"))
BATCH = int(os.environ.get("Q6_PRED_BATCH", "350"))
MAX_EVAL = int(os.environ.get("Q6_PRED_MAX_EVAL", "20000"))


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
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
"""

BASE_FROM = """FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
"""

BASE_CTE = (
    "WITH base AS (\n"
    "  SELECT\n"
    "    -- node attrs\n"
    "    p1.gender      AS p1_gender,\n"
    "    p1.language    AS p1_language,\n"
    "    p1.browserused AS p1_browserused,\n"
    "    p2.gender      AS p2_gender,\n"
    "    p2.language    AS p2_language,\n"
    "    p2.browserused AS p2_browserused,\n"
    "    p3.gender      AS p3_gender,\n"
    "    p3.language    AS p3_language,\n"
    "    p3.browserused AS p3_browserused,\n"
    "    t.name         AS t_name,\n"
    "    -- edge attrs\n"
    "    pk12.src       AS pk12_src,\n"
    "    pk12.dst       AS pk12_dst,\n"
    "    pk23.src       AS pk23_src,\n"
    "    pk23.dst       AS pk23_dst,\n"
    "    phi.creationdate AS phi_creationdate,\n"
    "    phi.deletiondate AS phi_deletiondate\n"
    f"{BASE_FROM}"
    ")\n"
)


def fetch_distributions() -> Dict[str, object]:
    d: Dict[str, object] = {}

    d["genders"] = [r[0] for r in psql_rows("SELECT gender, count(*) FROM person GROUP BY gender ORDER BY count(*) DESC;") if r and r[0] != ""]
    d["person_langs"] = [r[0] for r in psql_rows("SELECT language, count(*) FROM person GROUP BY language ORDER BY count(*) DESC LIMIT 20;") if r and r[0] != ""]
    d["person_browsers"] = [r[0] for r in psql_rows("SELECT browserused, count(*) FROM person GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;") if r and r[0] != ""]

    # hot interest tags (many-to-many usage)
    d["hot_interest_tags"] = [r[0] for r in psql_rows(
        "SELECT t.name, count(*) FROM person_hasinterest_tag phi JOIN tag t ON t.id=phi.tagid "
        "GROUP BY t.name ORDER BY count(*) DESC LIMIT 80;"
    ) if r and r[0] != ""]

    # edge dates for phi
    phi_cd_rows = psql_rows(
        "SELECT 'phi_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text "
        "FROM person_hasinterest_tag;"
    )
    for k, arr in phi_cd_rows:
        d[k] = parse_pg_array_numbers(arr)

    # id percentiles for knows
    pk_rows = psql_rows(
        "SELECT 'src' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY src)::text FROM person_knows_person "
        "UNION ALL "
        "SELECT 'dst' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY dst)::text FROM person_knows_person;"
    )
    for k, arr in pk_rows:
        d[f"pk_{k}"] = parse_pg_array_numbers(arr)

    return d


def build_atoms(d: Dict[str, object]) -> List[Atom]:
    atoms: List[Atom] = []

    # p1/p2/p3 demographics & client
    for g in d["genders"][:2]:
        atoms.append(Atom("p1", f"p1.gender = {sh_quote_sql(g)}", f"p1_gender = {sh_quote_sql(g)}", "demographic"))
        atoms.append(Atom("p2", f"p2.gender = {sh_quote_sql(g)}", f"p2_gender = {sh_quote_sql(g)}", "demographic"))
        atoms.append(Atom("p3", f"p3.gender = {sh_quote_sql(g)}", f"p3_gender = {sh_quote_sql(g)}", "demographic"))

    for lang in d["person_langs"][:12]:
        atoms.append(Atom("p1", f"p1.language = {sh_quote_sql(lang)}", f"p1_language = {sh_quote_sql(lang)}", "language"))
        atoms.append(Atom("p2", f"p2.language = {sh_quote_sql(lang)}", f"p2_language = {sh_quote_sql(lang)}", "language"))
        atoms.append(Atom("p3", f"p3.language = {sh_quote_sql(lang)}", f"p3_language = {sh_quote_sql(lang)}", "language"))

    for b in d["person_browsers"][:8]:
        atoms.append(Atom("p1", f"p1.browserused = {sh_quote_sql(b)}", f"p1_browserused = {sh_quote_sql(b)}", "client"))
        atoms.append(Atom("p2", f"p2.browserused = {sh_quote_sql(b)}", f"p2_browserused = {sh_quote_sql(b)}", "client"))
        atoms.append(Atom("p3", f"p3.browserused = {sh_quote_sql(b)}", f"p3_browserused = {sh_quote_sql(b)}", "client"))


    # tag predicates: interest tags
    for name in d["hot_interest_tags"][:30]:
        atoms.append(Atom("t", f"t.name = {sh_quote_sql(name)}", f"t_name = {sh_quote_sql(name)}", "tag"))

    # edge predicates: phi time window
    phi_cd = d.get("phi_cd", [])
    if phi_cd:
        atoms += [
            Atom("phi", f"phi.creationdate >= {int(phi_cd[2])}", f"phi_creationdate >= {int(phi_cd[2])}", "time"),
            Atom("phi", f"phi.creationdate BETWEEN {int(phi_cd[1])} AND {int(phi_cd[4])}", f"phi_creationdate BETWEEN {int(phi_cd[1])} AND {int(phi_cd[4])}", "time"),
        ]
    # deletiondate sometimes null/0; include a safe predicate
    atoms.append(Atom("phi", "phi.deletiondate IS NULL", "phi_deletiondate IS NULL", "lifecycle"))

    # knows edges: id windows for coverage
    pk_src = d["pk_src"]
    pk_dst = d["pk_dst"]
    atoms += [
        Atom("pk12", f"pk12.src BETWEEN {int(pk_src[1])} AND {int(pk_src[3])}", f"pk12_src BETWEEN {int(pk_src[1])} AND {int(pk_src[3])}", "id"),
        Atom("pk23", f"pk23.dst BETWEEN {int(pk_dst[1])} AND {int(pk_dst[3])}", f"pk23_dst BETWEEN {int(pk_dst[1])} AND {int(pk_dst[3])}", "id"),
    ]

    atoms = [a for a in atoms if is_safebound_supported_predicate(a.sql)]
    return atoms


def choose_predicates_balanced(
    atoms_by_group: Dict[str, List[Atom]],
    group_counts: Dict[str, int],
    kind_counts: Dict[str, int],
    combo_counts: Dict[Tuple[str, ...], int],
) -> List[Atom]:
    node_groups = ["p1", "p2", "p3", "t"]
    edge_groups = ["pk12", "pk23", "phi"]
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

        # include at least one node predicate (often tag)
        g_node_pool = node_groups
        g_node = pick_group(g_node_pool, used)
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


def q6_base_count() -> int:
    q = f"SELECT count(*) FROM (SELECT 1 {BASE_FROM}) q;"
    return int(psql_at(q))


def main() -> None:
    random.seed(RNG_SEED)
    d = fetch_distributions()
    atoms = build_atoms(d)

    atoms_by_group: Dict[str, List[Atom]] = {}
    for a in atoms:
        atoms_by_group.setdefault(a.group, []).append(a)

    all_groups = ["p1", "p2", "p3", "t", "pk12", "pk23", "phi"]
    for g in all_groups:
        atoms_by_group.setdefault(g, [])

    group_counts: Dict[str, int] = {g: 0 for g in atoms_by_group.keys()}
    kind_counts: Dict[str, int] = {}
    combo_counts: Dict[Tuple[str, ...], int] = {}

    base_cnt = q6_base_count()
    if base_cnt <= 0:
        raise SystemExit(f"Unexpected Q6 base cardinality: {base_cnt}")

    print(f"[init] db={DB} user={USER} seed={RNG_SEED} target={TARGET} batch={BATCH} max_eval={MAX_EVAL}")
    print(f"[init] Q6 base true_cardinality={base_cnt}")
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
    sql_lines.append(f"-- Q6 base true_cardinality: {base_cnt}")
    sql_lines.append(f"-- Generated with seed={RNG_SEED}")
    sql_lines.append("")
    for idx, preds, cnt in accepted:
        pid = f"q6_p{idx:03d}"
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

