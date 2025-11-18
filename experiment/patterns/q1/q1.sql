SELECT
  -- 点表
  f.*,
  p.*,
  ci.*,
  co.*,
  po.*,
  c.*,
  t.*,
  tc.*,

  -- 边表
  fhm.*,
  plc.*,
  cipc.*,
  fcp.*,
  crp.*,
  cht.*,
  tht.*
FROM forum AS f
JOIN forum_hasmember_person AS fhm
  ON fhm.forumid = f.id
JOIN person AS p
  ON p.id = fhm.personid
JOIN person_islocatedin_city AS plc
  ON plc.personid = p.id
JOIN city AS ci
  ON ci.id = plc.cityid
JOIN city_ispartof_country AS cipc
  ON cipc.place1id = ci.id
JOIN country AS co
  ON co.id = cipc.place2id
JOIN forum_containerof_post AS fcp
  ON fcp.forumid = f.id
JOIN post AS po
  ON po.id = fcp.postid
JOIN comment_replyof_post AS crp
  ON crp.postid = po.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN tag_hastype_tagclass AS tht
  ON tht.tagid = t.id
JOIN tagclass AS tc
  ON tc.id = tht.tagclassid;

-- true_cardinality: 8751507
