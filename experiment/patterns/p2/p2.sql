SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid;

-- true_cardinality: 2183