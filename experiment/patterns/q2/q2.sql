SELECT
  -- 点表
  p1.*,
  p2.*,
  c.*,
  po.*,

  -- 边表
  pkp.*,
  chp.*,
  crp.*,
  php.*
FROM comment AS c
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p1
  ON p1.id = chp.personid
JOIN comment_replyof_post AS crp
  ON crp.commentid = c.id
JOIN post AS po
  ON po.id = crp.postid
JOIN post_hascreator_person AS php
  ON php.postid = po.id
JOIN person AS p2
  ON p2.id = php.personid
JOIN person_knows_person AS pkp
  ON pkp.src = p1.id
 AND pkp.dst = p2.id;

-- true_cardinality: 82840
