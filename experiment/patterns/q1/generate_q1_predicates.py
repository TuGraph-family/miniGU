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

SQL_OUT = Path("/home/zxz/benchmark/q1/q1_predicates_100.sql")
CSV_OUT = Path("/home/zxz/benchmark/q1/q1_predicates_100.csv")

RNG_SEED = int(os.environ.get("Q1_PRED_SEED", "20260115"))
TARGET = int(os.environ.get("Q1_PRED_TARGET", "100"))
BATCH = int(os.environ.get("Q1_PRED_BATCH", "250"))


def sh_quote_sql(s: str) -> str:
    # SQL single-quote escaping
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
    # "{1,2,3}" -> [1.0,2.0,3.0]
    arr = arr.strip()
    if not (arr.startswith("{") and arr.endswith("}")):
        raise ValueError(f"Unexpected array format: {arr}")
    inner = arr[1:-1].strip()
    if inner == "":
        return []
    return [float(x) for x in inner.split(",")]


@dataclass(frozen=True)
class Atom:
    group: str  # which node/edge table this predicate targets
    sql: str    # predicate SQL (against original join aliases)
    base_sql: str  # predicate SQL against CTE column aliases (for batch counting)
    kind: str   # semantic category (time/geo/tag/etc) for balancing


BASE_SELECT = """SELECT
  -- 点表
  f.*,
  p.*,
  ci.*,
  co.*,
  po.*,
  c.*,
  t.*,
  tc.*,

  -- 边表
  fhm.*,
  plc.*,
  cipc.*,
  fcp.*,
  crp.*,
  cht.*,
  tht.*
"""

BASE_FROM = """FROM forum AS f
JOIN forum_hasmember_person AS fhm
  ON fhm.forumid = f.id
JOIN person AS p
  ON p.id = fhm.personid
JOIN person_islocatedin_city AS plc
  ON plc.personid = p.id
JOIN city AS ci
  ON ci.id = plc.cityid
JOIN city_ispartof_country AS cipc
  ON cipc.place1id = ci.id
JOIN country AS co
  ON co.id = cipc.place2id
JOIN forum_containerof_post AS fcp
  ON fcp.forumid = f.id
JOIN post AS po
  ON po.id = fcp.postid
JOIN comment_replyof_post AS crp
  ON crp.postid = po.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN tag_hastype_tagclass AS tht
  ON tht.tagid = t.id
JOIN tagclass AS tc
  ON tc.id = tht.tagclassid
"""

BASE_CTE = (
    "WITH base AS (\n"
    "  SELECT\n"
    "    -- node attrs\n"
    "    p.gender            AS p_gender,\n"
    "    p.language          AS p_language,\n"
    "    p.browserused       AS p_browserused,\n"
    "    f.explicitlydeleted AS f_explicitlydeleted,\n"
    "    f.creationdate      AS f_creationdate,\n"
    "    f.title             AS f_title,\n"
    "    ci.name             AS ci_name,\n"
    "    co.name             AS co_name,\n"
    "    po.explicitlydeleted AS po_explicitlydeleted,\n"
    "    po.creationdate      AS po_creationdate,\n"
    "    po.length            AS po_length,\n"
    "    po.language          AS po_language,\n"
    "    c.explicitlydeleted  AS c_explicitlydeleted,\n"
    "    c.creationdate       AS c_creationdate,\n"
    "    c.length             AS c_length,\n"
    "    c.browserused        AS c_browserused,\n"
    "    t.name              AS t_name,\n"
    "    tc.name             AS tc_name,\n"
    "    -- edge attrs\n"
    "    fhm.explicitlydeleted AS fhm_explicitlydeleted,\n"
    "    fhm.creationdate      AS fhm_creationdate,\n"
    "    plc.explicitlydeleted AS plc_explicitlydeleted,\n"
    "    plc.creationdate      AS plc_creationdate,\n"
    "    cipc.src_type         AS cipc_src_type,\n"
    "    fcp.explicitlydeleted AS fcp_explicitlydeleted,\n"
    "    fcp.creationdate      AS fcp_creationdate,\n"
    "    crp.explicitlydeleted AS crp_explicitlydeleted,\n"
    "    crp.creationdate      AS crp_creationdate,\n"
    "    cht.creationdate      AS cht_creationdate,\n"
    "    tht.tagclassid        AS tht_tagclassid\n"
    f"{BASE_FROM}"
    ")\n"
)


def fetch_distributions() -> Dict[str, object]:
    d: Dict[str, object] = {}

    d["genders"] = [r[0] for r in psql_rows("SELECT gender, count(*) FROM person GROUP BY gender ORDER BY count(*) DESC;") if r and r[0] != ""]
    d["person_langs"] = [r[0] for r in psql_rows("SELECT language, count(*) FROM person GROUP BY language ORDER BY count(*) DESC LIMIT 20;") if r and r[0] != ""]
    d["post_langs"] = [r[0] for r in psql_rows("SELECT language, count(*) FROM post GROUP BY language ORDER BY count(*) DESC LIMIT 20;") if r and r[0] != ""]

    d["countries"] = [r[0] for r in psql_rows("SELECT name, count(*) FROM country GROUP BY name ORDER BY count(*) DESC LIMIT 40;") if r and r[0] != ""]
    d["cities"] = [r[0] for r in psql_rows("SELECT name, count(*) FROM city GROUP BY name ORDER BY count(*) DESC LIMIT 40;") if r and r[0] != ""]
    d["forum_titles"] = [r[0] for r in psql_rows("SELECT title, count(*) FROM forum GROUP BY title ORDER BY count(*) DESC LIMIT 40;") if r and r[0] != ""]

    # tags used on comments (important for Q1 path)
    d["hot_comment_tags"] = [r[0] for r in psql_rows(
        "SELECT t.name, count(*) FROM comment_hastag_tag cht JOIN tag t ON t.id=cht.tagid "
        "GROUP BY t.name ORDER BY count(*) DESC LIMIT 60;"
    ) if r and r[0] != ""]

    d["hot_comment_tagclasses"] = [r[0] for r in psql_rows(
        "SELECT tc.name, count(*) FROM comment_hastag_tag cht "
        "JOIN tag_hastype_tagclass tht ON tht.tagid=cht.tagid "
        "JOIN tagclass tc ON tc.id=tht.tagclassid "
        "GROUP BY tc.name ORDER BY count(*) DESC LIMIT 40;"
    ) if r and r[0] != ""]

    # length percentiles
    len_rows = psql_rows(
        "SELECT 'post_len' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY length)::text FROM post "
        "UNION ALL "
        "SELECT 'comment_len' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY length)::text FROM comment;"
    )
    for k, arr in len_rows:
        d[k] = parse_pg_array_numbers(arr)

    # creationdate percentiles (nodes)
    cd_rows = psql_rows(
        "SELECT 'forum_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM forum "
        "UNION ALL "
        "SELECT 'post_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM post "
        "UNION ALL "
        "SELECT 'comment_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM comment;"
    )
    for k, arr in cd_rows:
        d[k] = parse_pg_array_numbers(arr)

    # creationdate percentiles (edges used in Q1)
    ecd_rows = psql_rows(
        "SELECT 'fhm_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM forum_hasmember_person "
        "UNION ALL "
        "SELECT 'plc_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM person_islocatedin_city "
        "UNION ALL "
        "SELECT 'fcp_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM forum_containerof_post "
        "UNION ALL "
        "SELECT 'crp_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM comment_replyof_post "
        "UNION ALL "
        "SELECT 'cht_cd' as k, percentile_cont(ARRAY[0.1,0.25,0.5,0.75,0.9]) WITHIN GROUP (ORDER BY creationdate)::text FROM comment_hastag_tag;"
    )
    for k, arr in ecd_rows:
        d[k] = parse_pg_array_numbers(arr)

    # browserused distributions (person, post, comment)
    d["person_browsers"] = [r[0] for r in psql_rows("SELECT browserused, count(*) FROM person GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;") if r and r[0] != ""]
    d["post_browsers"] = [r[0] for r in psql_rows("SELECT browserused, count(*) FROM post GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;") if r and r[0] != ""]
    d["comment_browsers"] = [r[0] for r in psql_rows("SELECT browserused, count(*) FROM comment GROUP BY browserused ORDER BY count(*) DESC LIMIT 15;") if r and r[0] != ""]

    return d


def build_atoms(d: Dict[str, object]) -> List[Atom]:
    atoms: List[Atom] = []

    # ---- Node predicates (LDBC-ish: geo, language, time, content length, deleted flags) ----
    for g in d["genders"][:2]:
        atoms.append(Atom("p", f"p.gender = {sh_quote_sql(g)}", f"p_gender = {sh_quote_sql(g)}", "demographic"))

    for lang in d["person_langs"][:12]:
        atoms.append(Atom("p", f"p.language = {sh_quote_sql(lang)}", f"p_language = {sh_quote_sql(lang)}", "language"))

    for b in d["person_browsers"][:8]:
        atoms.append(Atom("p", f"p.browserused = {sh_quote_sql(b)}", f"p_browserused = {sh_quote_sql(b)}", "client"))

    # forum
    atoms.append(Atom("f", "f.explicitlydeleted = false", "f_explicitlydeleted = false", "lifecycle"))
    fcd = d["forum_cd"]
    atoms += [
        Atom("f", f"f.creationdate >= {int(fcd[1])}", f"f_creationdate >= {int(fcd[1])}", "time"),
        Atom("f", f"f.creationdate BETWEEN {int(fcd[0])} AND {int(fcd[3])}", f"f_creationdate BETWEEN {int(fcd[0])} AND {int(fcd[3])}", "time"),
    ]
    for title in d["forum_titles"][:10]:
        atoms.append(Atom("f", f"f.title = {sh_quote_sql(title)}", f"f_title = {sh_quote_sql(title)}", "forum"))

    # city/country
    for name in d["countries"][:20]:
        atoms.append(Atom("co", f"co.name = {sh_quote_sql(name)}", f"co_name = {sh_quote_sql(name)}", "geo"))
    for name in d["cities"][:20]:
        atoms.append(Atom("ci", f"ci.name = {sh_quote_sql(name)}", f"ci_name = {sh_quote_sql(name)}", "geo"))

    # post
    atoms.append(Atom("po", "po.explicitlydeleted = false", "po_explicitlydeleted = false", "lifecycle"))
    pcd = d["post_cd"]
    atoms += [
        Atom("po", f"po.creationdate >= {int(pcd[2])}", f"po_creationdate >= {int(pcd[2])}", "time"),
        Atom("po", f"po.creationdate BETWEEN {int(pcd[1])} AND {int(pcd[3])}", f"po_creationdate BETWEEN {int(pcd[1])} AND {int(pcd[3])}", "time"),
    ]
    plen = d["post_len"]
    atoms += [
        Atom("po", "po.length > 0", "po_length > 0", "content"),
        Atom("po", f"po.length BETWEEN {int(plen[0])} AND {int(plen[4])}", f"po_length BETWEEN {int(plen[0])} AND {int(plen[4])}", "content"),
    ]
    for lang in d["post_langs"][:10]:
        atoms.append(Atom("po", f"po.language = {sh_quote_sql(lang)}", f"po_language = {sh_quote_sql(lang)}", "language"))

    # comment
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

    # tag / tagclass (use popular tags on comments)
    for name in d["hot_comment_tags"][:25]:
        atoms.append(Atom("t", f"t.name = {sh_quote_sql(name)}", f"t_name = {sh_quote_sql(name)}", "tag"))
    for name in d["hot_comment_tagclasses"][:15]:
        atoms.append(Atom("tc", f"tc.name = {sh_quote_sql(name)}", f"tc_name = {sh_quote_sql(name)}", "tagclass"))

    # ---- Edge predicates ----
    # edges have lifecycle/time attrs; pick time windows similar to node ones
    fhm_cd = d["fhm_cd"]
    plc_cd = d["plc_cd"]
    fcp_cd = d["fcp_cd"]
    crp_cd = d["crp_cd"]
    cht_cd = d["cht_cd"]

    atoms.append(Atom("fhm", "fhm.explicitlydeleted = false", "fhm_explicitlydeleted = false", "lifecycle"))
    atoms.append(Atom("plc", "plc.explicitlydeleted = false", "plc_explicitlydeleted = false", "lifecycle"))
    atoms.append(Atom("fcp", "fcp.explicitlydeleted = false", "fcp_explicitlydeleted = false", "lifecycle"))
    atoms.append(Atom("crp", "crp.explicitlydeleted = false", "crp_explicitlydeleted = false", "lifecycle"))

    atoms += [
        Atom("fhm", f"fhm.creationdate >= {int(fhm_cd[2])}", f"fhm_creationdate >= {int(fhm_cd[2])}", "time"),
        Atom("plc", f"plc.creationdate BETWEEN {int(plc_cd[1])} AND {int(plc_cd[4])}", f"plc_creationdate BETWEEN {int(plc_cd[1])} AND {int(plc_cd[4])}", "time"),
        Atom("fcp", f"fcp.creationdate BETWEEN {int(fcp_cd[1])} AND {int(fcp_cd[3])}", f"fcp_creationdate BETWEEN {int(fcp_cd[1])} AND {int(fcp_cd[3])}", "time"),
        Atom("crp", f"crp.creationdate >= {int(crp_cd[2])}", f"crp_creationdate >= {int(crp_cd[2])}", "time"),
        Atom("cht", f"cht.creationdate BETWEEN {int(cht_cd[1])} AND {int(cht_cd[4])}", f"cht_creationdate BETWEEN {int(cht_cd[1])} AND {int(cht_cd[4])}", "time"),
    ]

    # cipc has only src_type besides ids; constant but we can still include once in a while
    atoms.append(Atom("cipc", "cipc.src_type = 'City'", "cipc_src_type = 'City'", "edge"))

    # tht has only ids; use tc predicate primarily, but add a lightweight numeric filter too
    atoms.append(Atom("tht", "tht.tagclassid IS NOT NULL", "tht_tagclassid IS NOT NULL", "edge"))

    return atoms


def choose_predicates(atoms_by_group: Dict[str, List[Atom]], group_counts: Dict[str, int], kind_counts: Dict[str, int]) -> List[Atom]:
    # pick 2 or 3 predicates, biasing toward under-covered groups and under-covered semantic kinds
    k = 2 if random.random() < 0.55 else 3

    # prioritize least-used groups, but keep some randomness
    groups = list(atoms_by_group.keys())
    groups.sort(key=lambda g: group_counts.get(g, 0))

    chosen_groups: List[str] = []
    for g in groups:
        if len(chosen_groups) >= k:
            break
        if not atoms_by_group[g]:
            continue
        # probabilistic pick among low-count groups
        if len(chosen_groups) < (k - 1) and random.random() < 0.75:
            chosen_groups.append(g)
        elif len(chosen_groups) == (k - 1):
            chosen_groups.append(g)

    # fallback if we didn't pick enough
    while len(chosen_groups) < k:
        g = random.choice(groups)
        if g not in chosen_groups and atoms_by_group[g]:
            chosen_groups.append(g)

    preds: List[Atom] = []
    for g in chosen_groups:
        candidates = atoms_by_group[g]
        # bias toward under-used kinds
        candidates_sorted = sorted(candidates, key=lambda a: kind_counts.get(a.kind, 0))
        top = candidates_sorted[: max(5, len(candidates_sorted) // 3)]
        preds.append(random.choice(top))

    # LDBC-ish shaping: try to include at least one of {time, geo, tag/tagclass, language, lifecycle}
    kinds = {a.kind for a in preds}
    desired = ["time", "geo", "tag", "tagclass", "language", "lifecycle"]
    if len(preds) == 2 and not any(k in kinds for k in desired):
        # replace second with something more realistic
        pool = [a for a in sum(atoms_by_group.values(), []) if a.kind in desired]
        if pool:
            preds[1] = random.choice(pool)

    # avoid duplicates
    uniq: Dict[str, Atom] = {}
    for a in preds:
        uniq[a.sql] = a
    return list(uniq.values())


def batch_counts(where_base_sql_list: List[str]) -> Tuple[int, List[int]]:
    # One join, many counts via FILTER.
    # Returns (base_cnt, counts[i]) aligned with where_base_sql_list
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


def q1_base_count() -> int:
    # keep a cheap standalone base count (used before batch loop)
    q = f"SELECT count(*) FROM (SELECT 1 {BASE_FROM}) q;"
    return int(psql_at(q))


def main() -> None:
    random.seed(RNG_SEED)

    d = fetch_distributions()
    atoms = build_atoms(d)

    atoms_by_group: Dict[str, List[Atom]] = {}
    for a in atoms:
        atoms_by_group.setdefault(a.group, []).append(a)

    # groups we want to cover
    all_groups = ["f", "p", "ci", "co", "po", "c", "t", "tc", "fhm", "plc", "cipc", "fcp", "crp", "cht", "tht"]
    for g in all_groups:
        atoms_by_group.setdefault(g, [])

    group_counts: Dict[str, int] = {g: 0 for g in atoms_by_group.keys()}
    kind_counts: Dict[str, int] = {}

    accepted: List[Tuple[int, List[Atom], int]] = []  # (idx, atoms, count)
    seen_where: set[str] = set()

    base_cnt = q1_base_count()
    if base_cnt <= 0:
        raise SystemExit(f"Unexpected Q1 base cardinality: {base_cnt}")

    print(f"[init] db={DB} user={USER} seed={RNG_SEED} target={TARGET}")
    print(f"[init] Q1 base true_cardinality={base_cnt}")
    print(f"[init] atoms_total={len(atoms)} groups={len(atoms_by_group)}")

    # generate until we have TARGET valid queries; cap attempts to avoid infinite loops
    attempts = 0  # candidate where-clauses evaluated
    max_attempts = 8000
    batch_no = 0
    while len(accepted) < TARGET and attempts < max_attempts:
        batch_no += 1

        candidates: List[Tuple[List[Atom], str, str]] = []  # (atoms, join_where, base_where)
        while len(candidates) < BATCH and attempts < max_attempts:
            preds = choose_predicates(atoms_by_group, group_counts, kind_counts)
            if not (2 <= len(preds) <= 3):
                continue
            join_where = " AND ".join(f"({p.sql})" for p in preds)
            if join_where in seen_where:
                continue
            seen_where.add(join_where)
            base_where = " AND ".join(f"({p.base_sql})" for p in preds)
            candidates.append((preds, join_where, base_where))
            attempts += 1

        if not candidates:
            break

        # compute counts for the whole batch in one shot
        base_cnt2, counts = batch_counts([c[2] for c in candidates])
        if base_cnt2 != base_cnt:
            # not fatal, but indicates data changed; update selectivity denominator
            base_cnt = base_cnt2

        newly = 0
        for (preds, join_where, _), cnt in zip(candidates, counts):
            if cnt <= 0:
                continue
            if len(accepted) >= TARGET:
                break
            idx = len(accepted) + 1
            accepted.append((idx, preds, cnt))
            newly += 1
            for p in preds:
                group_counts[p.group] = group_counts.get(p.group, 0) + 1
                kind_counts[p.kind] = kind_counts.get(p.kind, 0) + 1

        rate = (len(accepted) / attempts) if attempts else 0.0
        print(f"[batch {batch_no}] +{newly} accepted, total={len(accepted)}/{TARGET} (evaluated={attempts}, pass_rate={rate:.3f})")

    if len(accepted) < TARGET:
        raise SystemExit(f"Only collected {len(accepted)} valid queries after {attempts} attempts.")

    # write combined SQL file: 100 statements, each ends with `-- || <truecard>`
    sql_lines: List[str] = []
    sql_lines.append(f"-- Q1 base true_cardinality: {base_cnt}")
    sql_lines.append(f"-- Generated with seed={RNG_SEED}")
    sql_lines.append("")
    for idx, preds, cnt in accepted:
        pid = f"q1_p{idx:03d}"
        preds_str = " AND ".join(p.sql for p in preds)
        sql_lines.append(f"-- {pid}")
        sql_lines.append(f"-- predicates: {preds_str}")
        sql_lines.append(BASE_SELECT.rstrip())
        sql_lines.append(BASE_FROM.rstrip())
        sql_lines.append(f"WHERE {preds_str}; -- || {cnt}")
        sql_lines.append("")
    SQL_OUT.write_text("\n".join(sql_lines) + "\n")

    # write CSV: predicate1,predicate2,predicate3,truecard,selectivity
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

