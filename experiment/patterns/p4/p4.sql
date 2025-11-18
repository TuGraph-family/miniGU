SELECT
  -- 点表
  p_creator.*,
  c.*,
  po.*,
  f.*,
  p_member.*,

  -- 边表
  chp.*,
  crp.*,
  fcp.*,
  fhm.*
FROM comment AS c
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p_creator
  ON p_creator.id = chp.personid
JOIN comment_replyof_post AS crp
  ON crp.commentid = c.id
JOIN post AS po
  ON po.id = crp.postid
JOIN forum_containerof_post AS fcp
  ON fcp.postid = po.id
JOIN forum AS f
  ON f.id = fcp.forumid
JOIN forum_hasmember_person AS fhm
  ON fhm.forumid = f.id
JOIN person AS p_member
  ON p_member.id = fhm.personid;

-- true_cardinality: 8128515