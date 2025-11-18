-- Q3 base true_cardinality: 30456
-- Generated with seed=20260115

-- q3_p001
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.gender = 'female'; -- || 8303

-- q3_p002
-- predicates: plc1.creationdate >= 1309571252085 AND co.name = 'Cameroon'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.creationdate >= 1309571252085 AND co.name = 'Cameroon'; -- || 18

-- q3_p003
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND co.name = 'Italy' AND plc2.creationdate BETWEEN 1285358988937 AND 1347940587243
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND co.name = 'Italy' AND plc2.creationdate BETWEEN 1285358988937 AND 1347940587243; -- || 4

-- q3_p004
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'pt;en'; -- || 710

-- q3_p005
-- predicates: plc1.creationdate >= 1309571252085 AND co.name = 'Burma'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.creationdate >= 1309571252085 AND co.name = 'Burma'; -- || 2

-- q3_p006
-- predicates: cipc1.src_type = 'City' AND p3.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p3.language = 'en'; -- || 762

-- q3_p007
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'pt;en'; -- || 710

-- q3_p008
-- predicates: cipc1.src_type = 'City' AND p1.language = 'zh;en' AND plc2.creationdate BETWEEN 1285358988937 AND 1347940587243
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p1.language = 'zh;en' AND plc2.creationdate BETWEEN 1285358988937 AND 1347940587243; -- || 6590

-- q3_p009
-- predicates: plc2.explicitlydeleted = false AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.explicitlydeleted = false AND p1.language = 'pt;en'; -- || 1668

-- q3_p010
-- predicates: plc3.explicitlydeleted = false AND p1.language = 'pt;en' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.explicitlydeleted = false AND p1.language = 'pt;en' AND pk12.src BETWEEN 252947 AND 253760; -- || 710

-- q3_p011
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND ci1.name = 'Mérida' AND plc3.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk23.dst BETWEEN 252947 AND 253760 AND ci1.name = 'Mérida' AND plc3.explicitlydeleted = false; -- || 14

-- q3_p012
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'en' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'en' AND pk12.src BETWEEN 252947 AND 253760; -- || 218

-- q3_p013
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'zh;en'; -- || 4644

-- q3_p014
-- predicates: p1.language = 'pt;en' AND p2.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE p1.language = 'pt;en' AND p2.language = 'pt;en'; -- || 1668

-- q3_p015
-- predicates: plc1.explicitlydeleted = false AND p3.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.explicitlydeleted = false AND p3.language = 'es;en'; -- || 2190

-- q3_p016
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p1.language = 'pt;en'; -- || 710

-- q3_p017
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'pt;en' AND p3.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'pt;en' AND p3.gender = 'male'; -- || 336

-- q3_p018
-- predicates: plc3.explicitlydeleted = false AND p1.language = 'pt;en' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.explicitlydeleted = false AND p1.language = 'pt;en' AND pk23.dst BETWEEN 252947 AND 253760; -- || 710

-- q3_p019
-- predicates: plc2.explicitlydeleted = false AND p1.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.explicitlydeleted = false AND p1.gender = 'female'; -- || 15579

-- q3_p020
-- predicates: plc3.explicitlydeleted = false AND p2.language = 'en' AND co.name = 'Cameroon'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.explicitlydeleted = false AND p2.language = 'en' AND co.name = 'Cameroon'; -- || 6

-- q3_p021
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p3.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p3.gender = 'male'; -- || 8060

-- q3_p022
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.gender = 'female'; -- || 8000

-- q3_p023
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND ci2.name = 'Intramuros' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND ci2.name = 'Intramuros' AND pk23.dst BETWEEN 252947 AND 253760; -- || 4

-- q3_p024
-- predicates: plc2.explicitlydeleted = false AND p3.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.explicitlydeleted = false AND p3.language = 'en'; -- || 762

-- q3_p025
-- predicates: plc2.explicitlydeleted = false AND p2.gender = 'female' AND p1.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.explicitlydeleted = false AND p2.gender = 'female' AND p1.language = 'en'; -- || 441

-- q3_p026
-- predicates: cipc1.src_type = 'City' AND p2.language = 'en' AND plc3.creationdate <= 1333481787908
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p2.language = 'en' AND plc3.creationdate <= 1333481787908; -- || 556

-- q3_p027
-- predicates: plc1.explicitlydeleted = false AND p3.language = 'en' AND cipc1.src_type = 'City'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.explicitlydeleted = false AND p3.language = 'en' AND cipc1.src_type = 'City'; -- || 762

-- q3_p028
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'female' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'female' AND pk12.src BETWEEN 252947 AND 253760; -- || 6103

-- q3_p029
-- predicates: plc2.explicitlydeleted = false AND p3.language = 'es;en' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.explicitlydeleted = false AND p3.language = 'es;en' AND pk12.src BETWEEN 252947 AND 253760; -- || 942

-- q3_p030
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND co.name = 'Cameroon'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND co.name = 'Cameroon'; -- || 30

-- q3_p031
-- predicates: plc3.explicitlydeleted = false AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.explicitlydeleted = false AND p2.language = 'en'; -- || 762

-- q3_p032
-- predicates: cipc1.src_type = 'City' AND p3.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p3.gender = 'female'; -- || 15584

-- q3_p033
-- predicates: plc2.explicitlydeleted = false AND p3.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.explicitlydeleted = false AND p3.language = 'zh;en'; -- || 8346

-- q3_p034
-- predicates: plc3.explicitlydeleted = false AND p3.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.explicitlydeleted = false AND p3.language = 'zh;en'; -- || 8346

-- q3_p035
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p3.gender = 'female' AND plc3.creationdate <= 1333481787908
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p3.gender = 'female' AND plc3.creationdate <= 1333481787908; -- || 8000

-- q3_p036
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p2.gender = 'male' AND plc3.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p2.gender = 'male' AND plc3.explicitlydeleted = false; -- || 7757

-- q3_p037
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'es;en' AND plc1.creationdate >= 1309571252085
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'es;en' AND plc1.creationdate >= 1309571252085; -- || 456

-- q3_p038
-- predicates: plc1.explicitlydeleted = false AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.explicitlydeleted = false AND p1.language = 'pt;en'; -- || 1668

-- q3_p039
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND co.name = 'Burma'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND co.name = 'Burma'; -- || 2

-- q3_p040
-- predicates: plc3.creationdate <= 1333481787908 AND p1.language = 'es;en' AND cipc1.src_type = 'City'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.creationdate <= 1333481787908 AND p1.language = 'es;en' AND cipc1.src_type = 'City'; -- || 1774

-- q3_p041
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'zh;en'; -- || 4646

-- q3_p042
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p3.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p3.language = 'pt;en'; -- || 710

-- q3_p043
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'pt;en' AND plc3.creationdate <= 1333481787908
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'pt;en' AND plc3.creationdate <= 1333481787908; -- || 449

-- q3_p044
-- predicates: plc1.creationdate >= 1309571252085 AND p1.language = 'es;en' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.creationdate >= 1309571252085 AND p1.language = 'es;en' AND pk23.dst BETWEEN 252947 AND 253760; -- || 378

-- q3_p045
-- predicates: cipc1.src_type = 'City' AND p1.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p1.language = 'zh;en'; -- || 8346

-- q3_p046
-- predicates: cipc1.src_type = 'City' AND p2.language = 'pt;en' AND plc3.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p2.language = 'pt;en' AND plc3.explicitlydeleted = false; -- || 1668

-- q3_p047
-- predicates: plc1.explicitlydeleted = false AND p3.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.explicitlydeleted = false AND p3.gender = 'female'; -- || 15579

-- q3_p048
-- predicates: plc2.explicitlydeleted = false AND p2.gender = 'female' AND p1.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.explicitlydeleted = false AND p2.gender = 'female' AND p1.language = 'zh;en'; -- || 4070

-- q3_p049
-- predicates: plc1.creationdate >= 1309571252085 AND ci2.name = 'Izmir'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.creationdate >= 1309571252085 AND ci2.name = 'Izmir'; -- || 13

-- q3_p050
-- predicates: cipc1.src_type = 'City' AND ci3.name = 'Intramuros'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND ci3.name = 'Intramuros'; -- || 4

-- q3_p051
-- predicates: plc3.explicitlydeleted = false AND p2.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.explicitlydeleted = false AND p2.gender = 'male'; -- || 14869

-- q3_p052
-- predicates: cipc1.src_type = 'City' AND p2.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p2.language = 'es;en'; -- || 2192

-- q3_p053
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'pt;en' AND plc1.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'pt;en' AND plc1.explicitlydeleted = false; -- || 710

-- q3_p054
-- predicates: cipc1.src_type = 'City' AND p1.language = 'pt;en' AND plc1.creationdate >= 1309571252085
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p1.language = 'pt;en' AND plc1.creationdate >= 1309571252085; -- || 890

-- q3_p055
-- predicates: plc2.explicitlydeleted = false AND p1.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.explicitlydeleted = false AND p1.language = 'en'; -- || 762

-- q3_p056
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND ci3.name = 'Intramuros' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND ci3.name = 'Intramuros' AND pk23.dst BETWEEN 252947 AND 253760; -- || 4

-- q3_p057
-- predicates: plc1.creationdate >= 1309571252085 AND p2.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.creationdate >= 1309571252085 AND p2.language = 'zh;en'; -- || 4286

-- q3_p058
-- predicates: plc3.creationdate <= 1333481787908 AND p1.language = 'zh;en' AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.creationdate <= 1333481787908 AND p1.language = 'zh;en' AND p2.gender = 'female'; -- || 2923

-- q3_p059
-- predicates: plc3.explicitlydeleted = false AND p2.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.explicitlydeleted = false AND p2.language = 'es;en'; -- || 2190

-- q3_p060
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'es;en'; -- || 916

-- q3_p061
-- predicates: p2.language = 'en' AND p1.language = 'es;en' AND plc3.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE p2.language = 'en' AND p1.language = 'es;en' AND plc3.explicitlydeleted = false; -- || 52

-- q3_p062
-- predicates: cipc1.src_type = 'City' AND ci3.name = 'Mérida'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND ci3.name = 'Mérida'; -- || 28

-- q3_p063
-- predicates: plc3.creationdate <= 1333481787908 AND p1.gender = 'male' AND p2.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.creationdate <= 1333481787908 AND p1.gender = 'male' AND p2.language = 'zh;en'; -- || 2985

-- q3_p064
-- predicates: plc3.creationdate <= 1333481787908 AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.creationdate <= 1333481787908 AND p1.language = 'pt;en'; -- || 1000

-- q3_p065
-- predicates: cipc1.src_type = 'City' AND p3.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p3.language = 'es;en'; -- || 2192

-- q3_p066
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND co.name = 'Italy'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk23.dst BETWEEN 252947 AND 253760 AND co.name = 'Italy'; -- || 4

-- q3_p067
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.language = 'en'; -- || 522

-- q3_p068
-- predicates: plc1.creationdate >= 1309571252085 AND p3.gender = 'male' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.creationdate >= 1309571252085 AND p3.gender = 'male' AND pk23.dst BETWEEN 252947 AND 253760; -- || 4045

-- q3_p069
-- predicates: plc1.explicitlydeleted = false AND p1.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.explicitlydeleted = false AND p1.gender = 'female'; -- || 15578

-- q3_p070
-- predicates: cipc1.src_type = 'City' AND ci1.name = 'Izmir'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND ci1.name = 'Izmir'; -- || 16

-- q3_p071
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'female'; -- || 10646

-- q3_p072
-- predicates: cipc1.src_type = 'City' AND ci1.name = 'Intramuros' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND ci1.name = 'Intramuros' AND pk23.dst BETWEEN 252947 AND 253760; -- || 4

-- q3_p073
-- predicates: cipc1.src_type = 'City' AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p1.language = 'pt;en'; -- || 1668

-- q3_p074
-- predicates: cipc1.src_type = 'City' AND ci2.name = 'Mérida'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND ci2.name = 'Mérida'; -- || 28

-- q3_p075
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND ci1.name = 'Intramuros'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND ci1.name = 'Intramuros'; -- || 4

-- q3_p076
-- predicates: plc3.creationdate <= 1333481787908 AND p2.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.creationdate <= 1333481787908 AND p2.language = 'pt;en'; -- || 1000

-- q3_p077
-- predicates: plc1.explicitlydeleted = false AND p2.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.explicitlydeleted = false AND p2.language = 'es;en'; -- || 2190

-- q3_p078
-- predicates: p1.gender = 'female' AND ci3.name = 'Mérida'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE p1.gender = 'female' AND ci3.name = 'Mérida'; -- || 9

-- q3_p079
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'pt;en' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'pt;en' AND pk12.src BETWEEN 252947 AND 253760; -- || 306

-- q3_p080
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND ci2.name = 'Izmir' AND p1.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk23.dst BETWEEN 252947 AND 253760 AND ci2.name = 'Izmir' AND p1.gender = 'female'; -- || 6

-- q3_p081
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.gender = 'female' AND plc3.creationdate <= 1333481787908
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.gender = 'female' AND plc3.creationdate <= 1333481787908; -- || 6402

-- q3_p082
-- predicates: ci3.name = 'Mérida' AND p3.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE ci3.name = 'Mérida' AND p3.gender = 'male'; -- || 28

-- q3_p083
-- predicates: plc2.explicitlydeleted = false AND ci1.name = 'Izmir' AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.explicitlydeleted = false AND ci1.name = 'Izmir' AND p2.gender = 'female'; -- || 9

-- q3_p084
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND ci1.name = 'Mérida'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND ci1.name = 'Mérida'; -- || 23

-- q3_p085
-- predicates: cipc1.src_type = 'City' AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p2.language = 'en'; -- || 762

-- q3_p086
-- predicates: plc1.explicitlydeleted = false AND co.name = 'Burma'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.explicitlydeleted = false AND co.name = 'Burma'; -- || 12

-- q3_p087
-- predicates: cipc1.src_type = 'City' AND p2.language = 'es;en' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p2.language = 'es;en' AND pk23.dst BETWEEN 252947 AND 253760; -- || 944

-- q3_p088
-- predicates: plc3.creationdate <= 1333481787908 AND p1.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.creationdate <= 1333481787908 AND p1.gender = 'female'; -- || 11912

-- q3_p089
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND ci2.name = 'Izmir'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk12.src BETWEEN 252947 AND 253760 AND ci2.name = 'Izmir'; -- || 10

-- q3_p090
-- predicates: plc1.explicitlydeleted = false AND p3.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.explicitlydeleted = false AND p3.language = 'en'; -- || 762

-- q3_p091
-- predicates: cipc1.src_type = 'City' AND p2.language = 'en' AND p1.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p2.language = 'en' AND p1.language = 'es;en'; -- || 52

-- q3_p092
-- predicates: plc3.creationdate <= 1333481787908 AND ci1.name = 'Intramuros'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.creationdate <= 1333481787908 AND ci1.name = 'Intramuros'; -- || 4

-- q3_p093
-- predicates: cipc1.src_type = 'City' AND p3.gender = 'female' AND plc3.creationdate <= 1333481787908
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND p3.gender = 'female' AND plc3.creationdate <= 1333481787908; -- || 12126

-- q3_p094
-- predicates: plc1.explicitlydeleted = false AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.explicitlydeleted = false AND p2.gender = 'female'; -- || 15579

-- q3_p095
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p3.language = 'es;en' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p3.language = 'es;en' AND pk12.src BETWEEN 252947 AND 253760; -- || 372

-- q3_p096
-- predicates: plc1.creationdate >= 1309571252085 AND p3.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc1.creationdate >= 1309571252085 AND p3.language = 'zh;en'; -- || 4286

-- q3_p097
-- predicates: cipc1.src_type = 'City' AND ci1.name = 'Mérida' AND p2.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE cipc1.src_type = 'City' AND ci1.name = 'Mérida' AND p2.gender = 'male'; -- || 19

-- q3_p098
-- predicates: plc3.creationdate <= 1333481787908 AND p3.language = 'en' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc3.creationdate <= 1333481787908 AND p3.language = 'en' AND pk23.dst BETWEEN 252947 AND 253760; -- || 484

-- q3_p099
-- predicates: plc2.explicitlydeleted = false AND p1.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.explicitlydeleted = false AND p1.language = 'es;en'; -- || 2190

-- q3_p100
-- predicates: plc2.explicitlydeleted = false AND ci3.name = 'Izmir'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  ci1.*,
  ci2.*,
  ci3.*,
  co.*,

  -- 边表
  plc1.*,
  plc2.*,
  plc3.*,
  cipc1.*,
  cipc2.*,
  cipc3.*,
  pk12.*,
  pk23.*,
  pk13.*
FROM person AS p1
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN city AS ci1
  ON ci1.id = plc1.cityid
JOIN city_ispartof_country AS cipc1
  ON cipc1.place1id = ci1.id
JOIN country AS co
  ON co.id = cipc1.place2id

JOIN person AS p2
  ON TRUE
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
JOIN city AS ci2
  ON ci2.id = plc2.cityid
JOIN city_ispartof_country AS cipc2
  ON cipc2.place1id = ci2.id
 AND cipc2.place2id = co.id

JOIN person AS p3
  ON TRUE
JOIN person_islocatedin_city AS plc3
  ON plc3.personid = p3.id
JOIN city AS ci3
  ON ci3.id = plc3.cityid
JOIN city_ispartof_country AS cipc3
  ON cipc3.place1id = ci3.id
 AND cipc3.place2id = co.id

JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
 AND pk12.dst = p2.id
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
 AND pk23.dst = p3.id
JOIN person_knows_person AS pk13
  ON pk13.src = p1.id
 AND pk13.dst = p3.id
WHERE plc2.explicitlydeleted = false AND ci3.name = 'Izmir'; -- || 16

