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

SQL_OUT = Path("/home/zxz/benchmark/p8/p8_predicates_100.sql")
CSV_OUT = Path("/home/zxz/benchmark/p8/p8_predicates_100.csv")

RNG_SEED = int(os.environ.get("p8_PRED_SEED", "20260115"))
TARGET = int(os.environ.get("p8_PRED_TARGET", "100"))
# This pattern can spill temp files; keep default batch moderate and auto-split on disk-pressure errors.
BATCH = int(os.environ.get("p8_PRED_BATCH", "120"))
MAX_EVAL = int(os.environ.get("p8_PRED_MAX_EVAL", "25000"))
WORK_MEM = os.environ.get("p8_PRED_WORK_MEM", "").strip()  # e.g. "128MB"


def sh_quote_sql(s: str) -> str:
    return "'" + s.replace("'", "''") + "'"


def psql_at(sql: str) -> str:
    env = os.environ.copy()
    env["PGPASSWORD"] = PASSWORD
    cmd = ["psql", "-U", USER, "-d", DB, "-v", "ON_ERROR_STOP=1", "-At", "-c", sql]
    out = subprocess.check_output(cmd, env=env, text=True, stderr=subprocess.STDOUT)
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
  t.*,
  c1.*,
  c2.*,
  p1.*,
  p2.*,

  -- 边表
  cht1.*,
  cht2.*,
  rcc.*,
  chp1.*,
  chp2.*,
  pkp.*
"""

BASE_FROM = """FROM tag AS t
JOIN comment_hastag_tag AS cht1
  ON cht1.tagid = t.id
JOIN comment AS c1
  ON c1.id = cht1.commentid
JOIN comment_replyof_comment AS rcc
  ON rcc.comment1id = c1.id
JOIN comment AS c2
  ON c2.id = rcc.comment2id
JOIN comment_hastag_tag AS cht2
  ON cht2.commentid = c2.id
 AND cht2.tagid = t.id
JOIN comment_hascreator_person AS chp1
  ON chp1.commentid = c1.id
JOIN person AS p1
  ON p1.id = chp1.personid
JOIN comment_hascreator_person AS chp2
  ON chp2.commentid = c2.id
JOIN person AS p2
  ON p2.id = chp2.personid
JOIN person_knows_person AS pkp
  ON pkp.src = p1.id
 AND pkp.dst = p2.id
"""

BASE_CTE = (
    "WITH base AS (\n"
    "  SELECT\n"
    "    -- node attrs\n"
    "    t.name               AS t_name,\n"
    "    p1.gender            AS p1_gender,\n"
    "    p1.language          AS p1_language,\n"
    "    p1.browserused       AS p1_browserused,\n"
    "    p1.birthday          AS p1_birthday,\n"
    "    p1.explicitlydeleted AS p1_explicitlydeleted,\n"
    "    p2.gender            AS p2_gender,\n"
    "    p2.language          AS p2_language,\n"
    "    p2.browserused       AS p2_browserused,\n"
    "    p2.birthday          AS p2_birthday,\n"
    "    p2.explicitlydeleted AS p2_explicitlydeleted,\n"
    "    c1.explicitlydeleted AS c1_explicitlydeleted,\n"
    "    c1.creationdate      AS c1_creationdate,\n"
    "    c1.length            AS c1_length,\n"
    "    c1.browserused       AS c1_browserused,\n"
    "    c2.explicitlydeleted AS c2_explicitlydeleted,\n"
    "    c2.creationdate      AS c2_creationdate,\n"
    "    c2.length            AS c2_length,\n"
    "    c2.browserused       AS c2_browserused,\n"
    "    -- edge attrs\n"
    "    cht1.creationdate    AS cht1_creationdate,\n"
    "    cht1.deletiondate    AS cht1_deletiondate,\n"
    "    cht2.creationdate    AS cht2_creationdate,\n"
    "    cht2.deletiondate    AS cht2_deletiondate,\n"
    "    chp1.creationdate      AS chp1_creationdate,\n"
    "    chp1.deletiondate      AS chp1_deletiondate,\n"
    "    chp1.explicitlydeleted AS chp1_explicitlydeleted,\n"
    "    chp2.creationdate      AS chp2_creationdate,\n"
    "    chp2.deletiondate      AS chp2_deletiondate,\n"
    "    chp2.explicitlydeleted AS chp2_explicitlydeleted,\n"
    "    rcc.creationdate       AS rcc_creationdate,\n"
    "    rcc.deletiondate       AS rcc_deletiondate,\n"
    "    rcc.explicitlydeleted  AS rcc_explicitlydeleted,\n"
    "    pkp.src              AS pkp_src,\n"
    "    pkp.dst              AS pkp_dst\n"
    f"{BASE_FROM}"
    ")\n"
)


def fetch_distributions() -> Dict[str, object]:
    d: Dict[str, object] = {}

    # people
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

    # comments
    d["comment_browsers"] = [
        r[0]
        for r in psql_rows("SELECT browserused, count(*) FROM comment GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;")
        if r and r[0] != ""
    ]

    # tags popular in comment_hastag_tag (best chance of non-zero)
    d["hot_comment_tags"] = [
        r[0]
        for r in psql_rows(
            "SELECT t.name, count(*) FROM comment_hastag_tag cht JOIN tag t ON t.id=cht.tagid "
            "GROUP BY t.name ORDER BY count(*) DESC LIMIT 120;"
        )
        if r and r[0] != ""
    ]

    pct = "ARRAY[0.1,0.25,0.5,0.75,0.9]"
    rows = psql_rows(
        "SELECT 'person_bday' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY birthday)::text FROM person "
        "UNION ALL "
        "SELECT 'comment_len' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY length)::text FROM comment "
        "UNION ALL "
        "SELECT 'comment_cd' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY creationdate)::text FROM comment "
        "UNION ALL "
        "SELECT 'cht_cd' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY creationdate)::text FROM comment_hastag_tag "
        "UNION ALL "
        "SELECT 'chp_cd' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY creationdate)::text FROM comment_hascreator_person "
        "UNION ALL "
        "SELECT 'rcc_cd' as k, percentile_cont("
        + pct
        + ") WITHIN GROUP (ORDER BY creationdate)::text FROM comment_replyof_comment "
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

    # tag predicates
    for name in d.get("hot_comment_tags", [])[:40]:
        atoms.append(Atom("t", f"t.name = {sh_quote_sql(name)}", f"t_name = {sh_quote_sql(name)}", "tag"))

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

    # comment predicates (c1/c2)
    atoms.append(Atom("c1", "c1.explicitlydeleted = false", "c1_explicitlydeleted = false", "lifecycle"))
    atoms.append(Atom("c2", "c2.explicitlydeleted = false", "c2_explicitlydeleted = false", "lifecycle"))

    ccd = d.get("comment_cd", [])
    if len(ccd) >= 5:
        atoms += [
            Atom("c1", f"c1.creationdate BETWEEN {int(ccd[1])} AND {int(ccd[4])}", f"c1_creationdate BETWEEN {int(ccd[1])} AND {int(ccd[4])}", "time"),
            Atom("c2", f"c2.creationdate BETWEEN {int(ccd[1])} AND {int(ccd[4])}", f"c2_creationdate BETWEEN {int(ccd[1])} AND {int(ccd[4])}", "time"),
            Atom("c1", f"c1.creationdate >= {int(ccd[2])}", f"c1_creationdate >= {int(ccd[2])}", "time"),
            Atom("c2", f"c2.creationdate >= {int(ccd[2])}", f"c2_creationdate >= {int(ccd[2])}", "time"),
        ]

    clen = d.get("comment_len", [])
    if len(clen) >= 5:
        atoms += [
            Atom("c1", f"c1.length <= {int(clen[1])}", f"c1_length <= {int(clen[1])}", "content"),
            Atom("c2", f"c2.length <= {int(clen[1])}", f"c2_length <= {int(clen[1])}", "content"),
            Atom("c1", f"c1.length BETWEEN {int(clen[2])} AND {int(clen[3])}", f"c1_length BETWEEN {int(clen[2])} AND {int(clen[3])}", "content"),
            Atom("c2", f"c2.length BETWEEN {int(clen[2])} AND {int(clen[3])}", f"c2_length BETWEEN {int(clen[2])} AND {int(clen[3])}", "content"),
            Atom("c1", f"c1.length >= {int(clen[3])}", f"c1_length >= {int(clen[3])}", "content"),
            Atom("c2", f"c2.length >= {int(clen[3])}", f"c2_length >= {int(clen[3])}", "content"),
        ]

    for b in d["comment_browsers"][:8]:
        atoms.append(Atom("c1", f"c1.browserused = {sh_quote_sql(b)}", f"c1_browserused = {sh_quote_sql(b)}", "client"))
        atoms.append(Atom("c2", f"c2.browserused = {sh_quote_sql(b)}", f"c2_browserused = {sh_quote_sql(b)}", "client"))

    # edges: lifecycle/time (cht has no explicitlydeleted)
    atoms += [
        Atom("chp1", "chp1.explicitlydeleted = false", "chp1_explicitlydeleted = false", "lifecycle"),
        Atom("chp2", "chp2.explicitlydeleted = false", "chp2_explicitlydeleted = false", "lifecycle"),
        Atom("rcc", "rcc.explicitlydeleted = false", "rcc_explicitlydeleted = false", "lifecycle"),
        Atom("chp1", "chp1.deletiondate IS NULL", "chp1_deletiondate IS NULL", "lifecycle"),
        Atom("chp2", "chp2.deletiondate IS NULL", "chp2_deletiondate IS NULL", "lifecycle"),
        Atom("rcc", "rcc.deletiondate IS NULL", "rcc_deletiondate IS NULL", "lifecycle"),
        Atom("cht1", "cht1.deletiondate IS NULL", "cht1_deletiondate IS NULL", "lifecycle"),
        Atom("cht2", "cht2.deletiondate IS NULL", "cht2_deletiondate IS NULL", "lifecycle"),
    ]

    cht_cd = d.get("cht_cd", [])
    chp_cd = d.get("chp_cd", [])
    rcc_cd = d.get("rcc_cd", [])
    if len(cht_cd) >= 5:
        atoms += [
            Atom("cht1", f"cht1.creationdate BETWEEN {int(cht_cd[1])} AND {int(cht_cd[4])}", f"cht1_creationdate BETWEEN {int(cht_cd[1])} AND {int(cht_cd[4])}", "time"),
            Atom("cht2", f"cht2.creationdate BETWEEN {int(cht_cd[1])} AND {int(cht_cd[4])}", f"cht2_creationdate BETWEEN {int(cht_cd[1])} AND {int(cht_cd[4])}", "time"),
        ]
    if len(chp_cd) >= 5:
        atoms += [
            Atom("chp1", f"chp1.creationdate BETWEEN {int(chp_cd[1])} AND {int(chp_cd[4])}", f"chp1_creationdate BETWEEN {int(chp_cd[1])} AND {int(chp_cd[4])}", "time"),
            Atom("chp2", f"chp2.creationdate BETWEEN {int(chp_cd[1])} AND {int(chp_cd[4])}", f"chp2_creationdate BETWEEN {int(chp_cd[1])} AND {int(chp_cd[4])}", "time"),
        ]
    if len(rcc_cd) >= 5:
        atoms += [
            Atom("rcc", f"rcc.creationdate BETWEEN {int(rcc_cd[1])} AND {int(rcc_cd[4])}", f"rcc_creationdate BETWEEN {int(rcc_cd[1])} AND {int(rcc_cd[4])}", "time"),
            Atom("rcc", f"rcc.creationdate >= {int(rcc_cd[2])}", f"rcc_creationdate >= {int(rcc_cd[2])}", "time"),
        ]

    # knows edge pkp: id windows
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
    node_groups = ["t", "c1", "c2", "p1", "p2"]
    edge_groups = ["cht1", "cht2", "rcc", "chp1", "chp2", "pkp"]
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
    def run_chunk(conds: List[str]) -> Tuple[int, List[int]]:
        selects = ["count(*) AS base_cnt"]
        for i, cond in enumerate(conds, start=1):
            alias = f"c{i:03d}"
            selects.append(f"count(*) FILTER (WHERE {cond}) AS {alias}")
        prefix = ""
        if WORK_MEM:
            prefix = f"SET work_mem TO {sh_quote_sql(WORK_MEM)}; "
        q = prefix + BASE_CTE + "SELECT " + ", ".join(selects) + " FROM base;"
        row = psql_at(q)
        parts = row.split("|") if row else []
        if len(parts) != 1 + len(conds):
            raise RuntimeError(f"Unexpected batch_counts output columns: got {len(parts)} expected {1+len(conds)}")
        base_cnt = int(parts[0])
        counts = [int(x) for x in parts[1:]]
        return base_cnt, counts

    try:
        return run_chunk(where_base_sql_list)
    except subprocess.CalledProcessError as e:
        msg = (e.output or "").lower()
        if ("no space left on device" in msg or "could not write to file" in msg) and len(where_base_sql_list) > 1:
            mid = max(1, len(where_base_sql_list) // 2)
            base1, c1 = batch_counts(where_base_sql_list[:mid])
            base2, c2 = batch_counts(where_base_sql_list[mid:])
            if base1 != base2:
                raise RuntimeError(f"Base count changed across split batches: {base1} vs {base2}")
            return base1, c1 + c2
        raise RuntimeError(f"psql failed: {e.output}") from e


def p8_base_count() -> int:
    q = f"SELECT count(*) FROM (SELECT 1 {BASE_FROM}) q;"
    return int(psql_at(q))


def main() -> None:
    random.seed(RNG_SEED)
    d = fetch_distributions()
    atoms = build_atoms(d)

    atoms_by_group: Dict[str, List[Atom]] = {}
    for a in atoms:
        atoms_by_group.setdefault(a.group, []).append(a)

    all_groups = ["t", "c1", "c2", "p1", "p2", "cht1", "cht2", "rcc", "chp1", "chp2", "pkp"]
    for g in all_groups:
        atoms_by_group.setdefault(g, [])

    group_counts: Dict[str, int] = {g: 0 for g in atoms_by_group.keys()}
    kind_counts: Dict[str, int] = {}
    combo_counts: Dict[Tuple[str, ...], int] = {}

    base_cnt = p8_base_count()
    if base_cnt <= 0:
        raise SystemExit(f"Unexpected p8 base cardinality: {base_cnt}")

    print(f"[init] db={DB} user={USER} seed={RNG_SEED} target={TARGET} batch={BATCH} max_eval={MAX_EVAL}")
    print(f"[init] p8 base true_cardinality={base_cnt}")
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

    sql_lines: List[str] = []
    sql_lines.append(f"-- p8 base true_cardinality: {base_cnt}")
    sql_lines.append(f"-- Generated with seed={RNG_SEED}")
    sql_lines.append("")
    for idx, preds, cnt in accepted:
        pid = f"p8_p{idx:03d}"
        preds_str = " AND ".join(p.sql for p in preds)
        sql_lines.append(f"-- {pid}")
        sql_lines.append(f"-- predicates: {preds_str}")
        sql_lines.append(BASE_SELECT.rstrip())
        sql_lines.append(BASE_FROM.rstrip())
        sql_lines.append(f"WHERE {preds_str}; -- || {cnt}")
        sql_lines.append("")
    SQL_OUT.write_text("\n".join(sql_lines) + "\n")

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

