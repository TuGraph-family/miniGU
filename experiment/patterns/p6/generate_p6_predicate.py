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

SQL_OUT = Path("/home/zxz/benchmark/p6/p6_predicates_100.sql")
CSV_OUT = Path("/home/zxz/benchmark/p6/p6_predicates_100.csv")

RNG_SEED = int(os.environ.get("p6_PRED_SEED", "20260115"))
TARGET = int(os.environ.get("p6_PRED_TARGET", "100"))
BATCH = int(os.environ.get("p6_PRED_BATCH", "350"))
MAX_EVAL = int(os.environ.get("p6_PRED_MAX_EVAL", "25000"))


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
    """
    SafeBound supports only column-vs-constant predicates: =, <, <=, >, >=, BETWEEN, IS NULL/IS NOT NULL, LIKE, IN.
    It does NOT support column-to-column predicates or != / <> predicates.
    """
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
  f.*,
  po.*,
  p1.*,
  p2.*,

  -- 边表
  fcp.*,
  fhm1.*,
  fhm2.*,
  pkp.*,
  plp1.*,
  plp2.*
"""

BASE_FROM = """FROM forum AS f
JOIN forum_containerof_post AS fcp
  ON fcp.forumid = f.id
JOIN post AS po
  ON po.id = fcp.postid
JOIN forum_hasmember_person AS fhm1
  ON fhm1.forumid = f.id
JOIN person AS p1
  ON p1.id = fhm1.personid
JOIN forum_hasmember_person AS fhm2
  ON fhm2.forumid = f.id
JOIN person AS p2
  ON p2.id = fhm2.personid
JOIN person_knows_person AS pkp
  ON pkp.src = p1.id
 AND pkp.dst = p2.id
JOIN person_likes_post AS plp1
  ON plp1.personid = p1.id
 AND plp1.postid = po.id
JOIN person_likes_post AS plp2
  ON plp2.personid = p2.id
 AND plp2.postid = po.id
"""

BASE_CTE = (
    "WITH base AS (\n"
    "  SELECT\n"
    "    -- node attrs\n"
    "    f.explicitlydeleted   AS f_explicitlydeleted,\n"
    "    f.creationdate        AS f_creationdate,\n"
    "    f.title               AS f_title,\n"
    "    po.explicitlydeleted  AS po_explicitlydeleted,\n"
    "    po.creationdate       AS po_creationdate,\n"
    "    po.length             AS po_length,\n"
    "    po.language           AS po_language,\n"
    "    po.browserused        AS po_browserused,\n"
    "    p1.gender             AS p1_gender,\n"
    "    p1.language           AS p1_language,\n"
    "    p1.browserused        AS p1_browserused,\n"
    "    p1.birthday           AS p1_birthday,\n"
    "    p1.explicitlydeleted  AS p1_explicitlydeleted,\n"
    "    p2.gender             AS p2_gender,\n"
    "    p2.language           AS p2_language,\n"
    "    p2.browserused        AS p2_browserused,\n"
    "    p2.birthday           AS p2_birthday,\n"
    "    p2.explicitlydeleted  AS p2_explicitlydeleted,\n"
    "    -- edge attrs\n"
    "    fcp.creationdate      AS fcp_creationdate,\n"
    "    fcp.deletiondate      AS fcp_deletiondate,\n"
    "    fcp.explicitlydeleted AS fcp_explicitlydeleted,\n"
    "    fhm1.creationdate      AS fhm1_creationdate,\n"
    "    fhm1.deletiondate      AS fhm1_deletiondate,\n"
    "    fhm1.explicitlydeleted AS fhm1_explicitlydeleted,\n"
    "    fhm2.creationdate      AS fhm2_creationdate,\n"
    "    fhm2.deletiondate      AS fhm2_deletiondate,\n"
    "    fhm2.explicitlydeleted AS fhm2_explicitlydeleted,\n"
    "    plp1.creationdate      AS plp1_creationdate,\n"
    "    plp1.deletiondate      AS plp1_deletiondate,\n"
    "    plp1.explicitlydeleted AS plp1_explicitlydeleted,\n"
    "    plp2.creationdate      AS plp2_creationdate,\n"
    "    plp2.deletiondate      AS plp2_deletiondate,\n"
    "    plp2.explicitlydeleted AS plp2_explicitlydeleted,\n"
    "    pkp.src               AS pkp_src,\n"
    "    pkp.dst               AS pkp_dst\n"
    f"{BASE_FROM}"
    ")\n"
)


def fetch_distributions() -> Dict[str, object]:
    d: Dict[str, object] = {}

    d["genders"] = [
        r[0]
        for r in psql_rows("SELECT gender, count(*) FROM person GROUP BY gender ORDER BY count(*) DESC;")
        if r and r[0] != ""
    ]
    d["person_langs"] = [
        r[0]
        for r in psql_rows("SELECT language, count(*) FROM person GROUP BY language ORDER BY count(*) DESC LIMIT 20;")
        if r and r[0] != ""
    ]
    d["person_browsers"] = [
        r[0]
        for r in psql_rows("SELECT browserused, count(*) FROM person GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;")
        if r and r[0] != ""
    ]

    d["post_langs"] = [
        r[0]
        for r in psql_rows("SELECT language, count(*) FROM post GROUP BY language ORDER BY count(*) DESC LIMIT 20;")
        if r
    ]
    d["post_browsers"] = [
        r[0]
        for r in psql_rows("SELECT browserused, count(*) FROM post GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;")
        if r and r[0] != ""
    ]

    d["forum_titles"] = [
        r[0]
        for r in psql_rows("SELECT title, count(*) FROM forum GROUP BY title ORDER BY count(*) DESC LIMIT 40;")
        if r and r[0] != ""
    ]

    pct = "ARRAY[0.1,0.25,0.5,0.75,0.9]"
    rows = psql_rows(
        "SELECT 'person_bday' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY birthday)::text FROM person "
        "UNION ALL "
        "SELECT 'post_len' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY length)::text FROM post "
        "UNION ALL "
        "SELECT 'forum_cd' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY creationdate)::text FROM forum "
        "UNION ALL "
        "SELECT 'post_cd' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY creationdate)::text FROM post "
        "UNION ALL "
        "SELECT 'fhm_cd' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY creationdate)::text FROM forum_hasmember_person "
        "UNION ALL "
        "SELECT 'fcp_cd' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY creationdate)::text FROM forum_containerof_post "
        "UNION ALL "
        "SELECT 'plp_cd' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY creationdate)::text FROM person_likes_post "
        "UNION ALL "
        "SELECT 'pkp_src' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY src)::text FROM person_knows_person "
        "UNION ALL "
        "SELECT 'pkp_dst' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY dst)::text FROM person_knows_person;"
    )
    for k, arr in rows:
        d[k] = parse_pg_array_numbers(arr)

    return d


def build_atoms(d: Dict[str, object]) -> List[Atom]:
    atoms: List[Atom] = []

    # person predicates (p1/p2)
    for g in d["genders"][:2]:
        atoms.append(Atom("p1", f"p1.gender = {sh_quote_sql(g)}", f"p1_gender = {sh_quote_sql(g)}", "demographic"))
        atoms.append(Atom("p2", f"p2.gender = {sh_quote_sql(g)}", f"p2_gender = {sh_quote_sql(g)}", "demographic"))
    for lang in d["person_langs"][:12]:
        atoms.append(Atom("p1", f"p1.language = {sh_quote_sql(lang)}", f"p1_language = {sh_quote_sql(lang)}", "language"))
        atoms.append(Atom("p2", f"p2.language = {sh_quote_sql(lang)}", f"p2_language = {sh_quote_sql(lang)}", "language"))
    for b in d["person_browsers"][:8]:
        atoms.append(Atom("p1", f"p1.browserused = {sh_quote_sql(b)}", f"p1_browserused = {sh_quote_sql(b)}", "client"))
        atoms.append(Atom("p2", f"p2.browserused = {sh_quote_sql(b)}", f"p2_browserused = {sh_quote_sql(b)}", "client"))
    atoms.append(Atom("p1", "p1.explicitlydeleted = false", "p1_explicitlydeleted = false", "lifecycle"))
    atoms.append(Atom("p2", "p2.explicitlydeleted = false", "p2_explicitlydeleted = false", "lifecycle"))

    bday = d.get("person_bday", [])
    if len(bday) >= 5:
        atoms += [
            Atom("p1", f"p1.birthday BETWEEN {int(bday[1])} AND {int(bday[4])}", f"p1_birthday BETWEEN {int(bday[1])} AND {int(bday[4])}", "time"),
            Atom("p2", f"p2.birthday BETWEEN {int(bday[1])} AND {int(bday[4])}", f"p2_birthday BETWEEN {int(bday[1])} AND {int(bday[4])}", "time"),
        ]

    # forum predicates
    atoms.append(Atom("f", "f.explicitlydeleted = false", "f_explicitlydeleted = false", "lifecycle"))
    fcd = d.get("forum_cd", [])
    if len(fcd) >= 5:
        atoms += [
            Atom("f", f"f.creationdate BETWEEN {int(fcd[1])} AND {int(fcd[4])}", f"f_creationdate BETWEEN {int(fcd[1])} AND {int(fcd[4])}", "time"),
        ]
    for title in d["forum_titles"][:12]:
        atoms.append(Atom("f", f"f.title = {sh_quote_sql(title)}", f"f_title = {sh_quote_sql(title)}", "forum"))

    # post predicates
    atoms.append(Atom("po", "po.explicitlydeleted = false", "po_explicitlydeleted = false", "lifecycle"))
    pcd = d.get("post_cd", [])
    if len(pcd) >= 5:
        atoms += [
            Atom("po", f"po.creationdate BETWEEN {int(pcd[1])} AND {int(pcd[4])}", f"po_creationdate BETWEEN {int(pcd[1])} AND {int(pcd[4])}", "time"),
            Atom("po", f"po.creationdate >= {int(pcd[2])}", f"po_creationdate >= {int(pcd[2])}", "time"),
        ]
    plen = d.get("post_len", [])
    if len(plen) >= 5:
        atoms += [
            Atom("po", f"po.length <= {int(plen[1])}", f"po_length <= {int(plen[1])}", "content"),
            Atom("po", f"po.length BETWEEN {int(plen[2])} AND {int(plen[3])}", f"po_length BETWEEN {int(plen[2])} AND {int(plen[3])}", "content"),
            Atom("po", f"po.length >= {int(plen[3])}", f"po_length >= {int(plen[3])}", "content"),
        ]
    for lang in [x for x in d["post_langs"] if x != ""][:10]:
        atoms.append(Atom("po", f"po.language = {sh_quote_sql(lang)}", f"po_language = {sh_quote_sql(lang)}", "language"))
    for b in d["post_browsers"][:8]:
        atoms.append(Atom("po", f"po.browserused = {sh_quote_sql(b)}", f"po_browserused = {sh_quote_sql(b)}", "client"))

    # edge predicates (lifecycle/time)
    atoms += [
        Atom("fcp", "fcp.explicitlydeleted = false", "fcp_explicitlydeleted = false", "lifecycle"),
        Atom("fhm1", "fhm1.explicitlydeleted = false", "fhm1_explicitlydeleted = false", "lifecycle"),
        Atom("fhm2", "fhm2.explicitlydeleted = false", "fhm2_explicitlydeleted = false", "lifecycle"),
        Atom("plp1", "plp1.explicitlydeleted = false", "plp1_explicitlydeleted = false", "lifecycle"),
        Atom("plp2", "plp2.explicitlydeleted = false", "plp2_explicitlydeleted = false", "lifecycle"),
        Atom("fcp", "fcp.deletiondate IS NULL", "fcp_deletiondate IS NULL", "lifecycle"),
        Atom("fhm1", "fhm1.deletiondate IS NULL", "fhm1_deletiondate IS NULL", "lifecycle"),
        Atom("fhm2", "fhm2.deletiondate IS NULL", "fhm2_deletiondate IS NULL", "lifecycle"),
        Atom("plp1", "plp1.deletiondate IS NULL", "plp1_deletiondate IS NULL", "lifecycle"),
        Atom("plp2", "plp2.deletiondate IS NULL", "plp2_deletiondate IS NULL", "lifecycle"),
    ]

    fhm_cd = d.get("fhm_cd", [])
    fcp_cd = d.get("fcp_cd", [])
    plp_cd = d.get("plp_cd", [])
    if len(fhm_cd) >= 5:
        atoms += [
            Atom("fhm1", f"fhm1.creationdate BETWEEN {int(fhm_cd[1])} AND {int(fhm_cd[4])}", f"fhm1_creationdate BETWEEN {int(fhm_cd[1])} AND {int(fhm_cd[4])}", "time"),
            Atom("fhm2", f"fhm2.creationdate BETWEEN {int(fhm_cd[1])} AND {int(fhm_cd[4])}", f"fhm2_creationdate BETWEEN {int(fhm_cd[1])} AND {int(fhm_cd[4])}", "time"),
        ]
    if len(fcp_cd) >= 5:
        atoms += [
            Atom("fcp", f"fcp.creationdate BETWEEN {int(fcp_cd[1])} AND {int(fcp_cd[4])}", f"fcp_creationdate BETWEEN {int(fcp_cd[1])} AND {int(fcp_cd[4])}", "time"),
        ]
    if len(plp_cd) >= 5:
        atoms += [
            Atom("plp1", f"plp1.creationdate BETWEEN {int(plp_cd[1])} AND {int(plp_cd[4])}", f"plp1_creationdate BETWEEN {int(plp_cd[1])} AND {int(plp_cd[4])}", "time"),
            Atom("plp2", f"plp2.creationdate BETWEEN {int(plp_cd[1])} AND {int(plp_cd[4])}", f"plp2_creationdate BETWEEN {int(plp_cd[1])} AND {int(plp_cd[4])}", "time"),
        ]

    # knows edge pkp: id windows over src/dst (SafeBound-friendly)
    ps = d.get("pkp_src", [])
    pd = d.get("pkp_dst", [])
    if len(ps) >= 4:
        atoms += [
            Atom("pkp", f"pkp.src BETWEEN {int(ps[1])} AND {int(ps[3])}", f"pkp_src BETWEEN {int(ps[1])} AND {int(ps[3])}", "id"),
            Atom("pkp", f"pkp.src >= {int(ps[2])}", f"pkp_src >= {int(ps[2])}", "id"),
        ]
    if len(pd) >= 4:
        atoms += [
            Atom("pkp", f"pkp.dst BETWEEN {int(pd[1])} AND {int(pd[3])}", f"pkp_dst BETWEEN {int(pd[1])} AND {int(pd[3])}", "id"),
            Atom("pkp", f"pkp.dst <= {int(pd[2])}", f"pkp_dst <= {int(pd[2])}", "id"),
        ]

    atoms = [a for a in atoms if is_safebound_supported_predicate(a.sql)]
    return atoms


def choose_predicates_balanced(
    atoms_by_group: Dict[str, List[Atom]],
    group_counts: Dict[str, int],
    kind_counts: Dict[str, int],
    combo_counts: Dict[Tuple[str, ...], int],
) -> List[Atom]:
    node_groups = ["f", "po", "p1", "p2"]
    edge_groups = ["fcp", "fhm1", "fhm2", "plp1", "plp2", "pkp"]
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


def p6_base_count() -> int:
    q = f"SELECT count(*) FROM (SELECT 1 {BASE_FROM}) q;"
    return int(psql_at(q))


def main() -> None:
    random.seed(RNG_SEED)
    d = fetch_distributions()
    atoms = build_atoms(d)

    atoms_by_group: Dict[str, List[Atom]] = {}
    for a in atoms:
        atoms_by_group.setdefault(a.group, []).append(a)

    all_groups = ["f", "po", "p1", "p2", "fcp", "fhm1", "fhm2", "plp1", "plp2", "pkp"]
    for g in all_groups:
        atoms_by_group.setdefault(g, [])

    group_counts: Dict[str, int] = {g: 0 for g in atoms_by_group.keys()}
    kind_counts: Dict[str, int] = {}
    combo_counts: Dict[Tuple[str, ...], int] = {}

    base_cnt = p6_base_count()
    if base_cnt <= 0:
        raise SystemExit(f"Unexpected p6 base cardinality: {base_cnt}")

    print(f"[init] db={DB} user={USER} seed={RNG_SEED} target={TARGET} batch={BATCH} max_eval={MAX_EVAL}")
    print(f"[init] p6 base true_cardinality={base_cnt}")
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
    sql_lines.append(f"-- p6 base true_cardinality: {base_cnt}")
    sql_lines.append(f"-- Generated with seed={RNG_SEED}")
    sql_lines.append("")
    for idx, preds, cnt in accepted:
        pid = f"p6_p{idx:03d}"
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

