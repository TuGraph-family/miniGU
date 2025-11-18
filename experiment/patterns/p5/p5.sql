SELECT
  -- 点表
  p1.*,
  c1.*,
  p2.*,
  c2.*,

  -- 边表
  plc1.*,
  chp1.*,
  rcc.*,
  plc2.*,
  chp2.*
FROM person AS p1
JOIN person_likes_comment AS plc1
  ON plc1.personid = p1.id
JOIN comment AS c1
  ON c1.id = plc1.commentid
JOIN comment_hascreator_person AS chp1
  ON chp1.commentid = c1.id
JOIN person AS p2
  ON p2.id = chp1.personid
JOIN comment_replyof_comment AS rcc
  ON rcc.comment1id = c1.id
JOIN comment AS c2
  ON c2.id = rcc.comment2id
JOIN person_likes_comment AS plc2
  ON plc2.personid = p2.id
 AND plc2.commentid = c2.id
JOIN comment_hascreator_person AS chp2
  ON chp2.commentid = c2.id
 AND chp2.personid = p1.id;

-- true_cardinality: 20