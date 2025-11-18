SELECT
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
FROM forum AS f
JOIN forum_containerof_post AS fcp
  ON fcp.src = f.id
JOIN post AS po
  ON po.id = fcp.dst
JOIN forum_hasmember_person AS fhm1
  ON fhm1.src = f.id
JOIN person AS p1
  ON p1.id = fhm1.dst
JOIN forum_hasmember_person AS fhm2
  ON fhm2.src = f.id
JOIN person AS p2
  ON p2.id = fhm2.dst
JOIN person_knows_person AS pkp
  ON pkp.src = p1.id
 AND pkp.dst = p2.id
JOIN person_likes_post AS plp1
  ON plp1.src = p1.id
 AND plp1.dst = po.id
JOIN person_likes_post AS plp2
  ON plp2.src = p2.id
 AND plp2.dst = po.id;

-- true_cardinality: 182818

