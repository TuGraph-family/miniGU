SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id;

-- true_cardinality: 3323