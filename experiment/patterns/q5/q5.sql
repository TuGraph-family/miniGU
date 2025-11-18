SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id;

-- true_cardinality: 492452
