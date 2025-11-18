SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
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
  ON t.id = phi.tagid;

-- true_cardinality: 55607896
