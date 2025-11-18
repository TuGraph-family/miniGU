SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid;

-- true_cardinality: 290601
