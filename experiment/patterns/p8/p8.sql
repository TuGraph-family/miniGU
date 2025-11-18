SELECT
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
FROM tag AS t
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
 AND pkp.dst = p2.id;

-- true_cardinality: 7741

