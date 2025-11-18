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
 AND pk13.dst = p3.id;

-- true_cardinality: 30456
