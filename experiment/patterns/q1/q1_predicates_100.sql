-- Q1 base true_cardinality: 8751507
-- Generated with seed=20260115

-- q1_p001
-- predicates: p.language = 'zh;en' AND f.creationdate >= 1305589168419
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'zh;en' AND f.creationdate >= 1305589168419; -- || 392936

-- q1_p002
-- predicates: f.title = 'Wall of Rahul Singh' AND co.name = 'Cameroon'
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
  ON tc.id = tht.tagclassid
WHERE f.title = 'Wall of Rahul Singh' AND co.name = 'Cameroon'; -- || 736

-- q1_p003
-- predicates: p.language = 'en' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'en' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313; -- || 384481

-- q1_p004
-- predicates: p.language = 'pt;en' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'pt;en' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313; -- || 181999

-- q1_p005
-- predicates: f.explicitlydeleted = false AND co.name = 'Cameroon'
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
  ON tc.id = tht.tagclassid
WHERE f.explicitlydeleted = false AND co.name = 'Cameroon'; -- || 47857

-- q1_p006
-- predicates: p.gender = 'female' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313; -- || 2366616

-- q1_p007
-- predicates: p.language = 'en' AND f.creationdate >= 1305589168419
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'en' AND f.creationdate >= 1305589168419; -- || 214707

-- q1_p008
-- predicates: p.language = 'es;en' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'es;en' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313; -- || 300785

-- q1_p009
-- predicates: p.gender = 'male' AND f.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND f.explicitlydeleted = false; -- || 4261260

-- q1_p010
-- predicates: f.creationdate >= 1305589168419 AND co.name = 'Venezuela'
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
  ON tc.id = tht.tagclassid
WHERE f.creationdate >= 1305589168419 AND co.name = 'Venezuela'; -- || 5870

-- q1_p011
-- predicates: p.language = 'pt;en' AND f.creationdate >= 1305589168419
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'pt;en' AND f.creationdate >= 1305589168419; -- || 98890

-- q1_p012
-- predicates: p.language = 'pt;en' AND f.title = 'Wall of Rahul Singh'
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'pt;en' AND f.title = 'Wall of Rahul Singh'; -- || 1050

-- q1_p013
-- predicates: p.gender = 'female' AND t.name = 'Mariano_Rivera'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND t.name = 'Mariano_Rivera'; -- || 6796

-- q1_p014
-- predicates: p.gender = 'male' AND c.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND c.explicitlydeleted = false; -- || 4209138

-- q1_p015
-- predicates: p.language = 'pt;en' AND f.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'pt;en' AND f.explicitlydeleted = false; -- || 326435

-- q1_p016
-- predicates: p.language = 'zh;en' AND f.title = 'Wall of Rahul Singh'
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'zh;en' AND f.title = 'Wall of Rahul Singh'; -- || 35413

-- q1_p017
-- predicates: f.creationdate BETWEEN 1287486205605 AND 1342670597313 AND co.name = 'Cameroon'
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
  ON tc.id = tht.tagclassid
WHERE f.creationdate BETWEEN 1287486205605 AND 1342670597313 AND co.name = 'Cameroon'; -- || 27660

-- q1_p018
-- predicates: p.gender = 'male' AND f.creationdate >= 1305589168419
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND f.creationdate >= 1305589168419; -- || 1346341

-- q1_p019
-- predicates: f.explicitlydeleted = false AND ci.name = 'Kochi' AND po.length BETWEEN 0 AND 97
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
  ON tc.id = tht.tagclassid
WHERE f.explicitlydeleted = false AND ci.name = 'Kochi' AND po.length BETWEEN 0 AND 97; -- || 968

-- q1_p020
-- predicates: f.creationdate BETWEEN 1287486205605 AND 1342670597313 AND co.name = 'Italy'
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
  ON tc.id = tht.tagclassid
WHERE f.creationdate BETWEEN 1287486205605 AND 1342670597313 AND co.name = 'Italy'; -- || 15108

-- q1_p021
-- predicates: p.language = 'pt;en' AND f.title = 'Wall of Rahul Khan'
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'pt;en' AND f.title = 'Wall of Rahul Khan'; -- || 60

-- q1_p022
-- predicates: p.gender = 'female' AND f.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND f.explicitlydeleted = false; -- || 4459059

-- q1_p023
-- predicates: p.gender = 'male' AND f.explicitlydeleted = false AND co.name = 'Burma'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND f.explicitlydeleted = false AND co.name = 'Burma'; -- || 81817

-- q1_p024
-- predicates: p.language = 'zh;en' AND f.title = 'Wall of Rahul Khan'
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'zh;en' AND f.title = 'Wall of Rahul Khan'; -- || 84

-- q1_p025
-- predicates: p.gender = 'female' AND f.creationdate >= 1305589168419
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND f.creationdate >= 1305589168419; -- || 1368663

-- q1_p026
-- predicates: p.language = 'en' AND f.title = 'Wall of Rahul Singh'
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'en' AND f.title = 'Wall of Rahul Singh'; -- || 2777

-- q1_p027
-- predicates: f.creationdate >= 1305589168419 AND co.name = 'Cameroon'
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
  ON tc.id = tht.tagclassid
WHERE f.creationdate >= 1305589168419 AND co.name = 'Cameroon'; -- || 16678

-- q1_p028
-- predicates: p.gender = 'female' AND p.language = 'en;tl'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND p.language = 'en;tl'; -- || 34569

-- q1_p029
-- predicates: p.gender = 'male' AND p.language = 'es;en'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND p.language = 'es;en'; -- || 299781

-- q1_p030
-- predicates: f.explicitlydeleted = false AND co.name = 'Burma'
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
  ON tc.id = tht.tagclassid
WHERE f.explicitlydeleted = false AND co.name = 'Burma'; -- || 132420

-- q1_p031
-- predicates: p.gender = 'female' AND p.language = 'en'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND p.language = 'en'; -- || 371302

-- q1_p032
-- predicates: co.name = 'Venezuela' AND po.length BETWEEN 0 AND 97 AND c.length BETWEEN 5 AND 79
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
  ON tc.id = tht.tagclassid
WHERE co.name = 'Venezuela' AND po.length BETWEEN 0 AND 97 AND c.length BETWEEN 5 AND 79; -- || 992

-- q1_p033
-- predicates: p.language = 'en' AND f.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'en' AND f.explicitlydeleted = false; -- || 724027

-- q1_p034
-- predicates: p.gender = 'female' AND f.title = 'Wall of Rahul Singh' AND co.name = 'Cameroon'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND f.title = 'Wall of Rahul Singh' AND co.name = 'Cameroon'; -- || 736

-- q1_p035
-- predicates: p.gender = 'male' AND tc.name = 'OfficeHolder'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND tc.name = 'OfficeHolder'; -- || 350276

-- q1_p036
-- predicates: p.gender = 'male' AND t.name = 'Sonia_Gandhi'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND t.name = 'Sonia_Gandhi'; -- || 21249

-- q1_p037
-- predicates: p.gender = 'male' AND co.name = 'Italy'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND co.name = 'Italy'; -- || 16704

-- q1_p038
-- predicates: p.gender = 'male' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313; -- || 2313480

-- q1_p039
-- predicates: p.language = 'es;en' AND f.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'es;en' AND f.explicitlydeleted = false; -- || 569448

-- q1_p040
-- predicates: p.language = 'es;en' AND f.title = 'Wall of Rahul Singh'
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'es;en' AND f.title = 'Wall of Rahul Singh'; -- || 1472

-- q1_p041
-- predicates: p.language = 'zh;en' AND f.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'zh;en' AND f.explicitlydeleted = false; -- || 1291963

-- q1_p042
-- predicates: p.gender = 'female' AND t.name = 'Augustine_of_Hippo'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND t.name = 'Augustine_of_Hippo'; -- || 12802

-- q1_p043
-- predicates: p.gender = 'female' AND ci.name = 'Thirthahalli'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND ci.name = 'Thirthahalli'; -- || 7926

-- q1_p044
-- predicates: ci.name = 'Mérida' AND po.length > 0
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
  ON tc.id = tht.tagclassid
WHERE ci.name = 'Mérida' AND po.length > 0; -- || 4148

-- q1_p045
-- predicates: p.language = 'zh;en' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'zh;en' AND f.creationdate BETWEEN 1287486205605 AND 1342670597313; -- || 723786

-- q1_p046
-- predicates: p.language = 'pt;en' AND po.length > 0 AND c.length BETWEEN 5 AND 79
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'pt;en' AND po.length > 0 AND c.length BETWEEN 5 AND 79; -- || 94715

-- q1_p047
-- predicates: p.gender = 'male' AND fhm.creationdate >= 1338094917792
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND fhm.creationdate >= 1338094917792; -- || 2486100

-- q1_p048
-- predicates: p.language = 'en' AND f.title = 'Wall of Rahul Khan'
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'en' AND f.title = 'Wall of Rahul Khan'; -- || 782

-- q1_p049
-- predicates: p.gender = 'male' AND fcp.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND fcp.explicitlydeleted = false; -- || 4274995

-- q1_p050
-- predicates: f.creationdate >= 1305589168419 AND tc.name = 'MusicalArtist' AND fhm.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE f.creationdate >= 1305589168419 AND tc.name = 'MusicalArtist' AND fhm.explicitlydeleted = false; -- || 245138

-- q1_p051
-- predicates: po.length BETWEEN 0 AND 97 AND t.name = 'Charlemagne'
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
  ON tc.id = tht.tagclassid
WHERE po.length BETWEEN 0 AND 97 AND t.name = 'Charlemagne'; -- || 3353

-- q1_p052
-- predicates: f.creationdate >= 1305589168419 AND co.name = 'Italy'
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
  ON tc.id = tht.tagclassid
WHERE f.creationdate >= 1305589168419 AND co.name = 'Italy'; -- || 6651

-- q1_p053
-- predicates: p.gender = 'male' AND ci.name = 'Gomel'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND ci.name = 'Gomel'; -- || 3193

-- q1_p054
-- predicates: f.creationdate BETWEEN 1287486205605 AND 1342670597313 AND co.name = 'Venezuela'
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
  ON tc.id = tht.tagclassid
WHERE f.creationdate BETWEEN 1287486205605 AND 1342670597313 AND co.name = 'Venezuela'; -- || 8710

-- q1_p055
-- predicates: c.creationdate >= 1350093427787 AND t.name = 'Sonia_Gandhi'
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
  ON tc.id = tht.tagclassid
WHERE c.creationdate >= 1350093427787 AND t.name = 'Sonia_Gandhi'; -- || 11417

-- q1_p056
-- predicates: p.gender = 'female' AND co.name = 'Czech_Republic'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND co.name = 'Czech_Republic'; -- || 39837

-- q1_p057
-- predicates: f.title = 'Wall of Rahul Khan' AND co.name = 'Burma'
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
  ON tc.id = tht.tagclassid
WHERE f.title = 'Wall of Rahul Khan' AND co.name = 'Burma'; -- || 295

-- q1_p058
-- predicates: po.creationdate >= 1328021100728 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
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
  ON tc.id = tht.tagclassid
WHERE po.creationdate >= 1328021100728 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 2991262

-- q1_p059
-- predicates: p.gender = 'female' AND f.creationdate >= 1305589168419 AND co.name = 'Burma'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND f.creationdate >= 1305589168419 AND co.name = 'Burma'; -- || 9756

-- q1_p060
-- predicates: f.creationdate >= 1305589168419 AND po.creationdate BETWEEN 1308052370341 AND 1343818164989 AND c.length BETWEEN 5 AND 79
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
  ON tc.id = tht.tagclassid
WHERE f.creationdate >= 1305589168419 AND po.creationdate BETWEEN 1308052370341 AND 1343818164989 AND c.length BETWEEN 5 AND 79; -- || 401932

-- q1_p061
-- predicates: p.language = 'es;en' AND f.creationdate >= 1305589168419
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'es;en' AND f.creationdate >= 1305589168419; -- || 182423

-- q1_p062
-- predicates: p.gender = 'female' AND f.explicitlydeleted = false AND co.name = 'Cameroon'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND f.explicitlydeleted = false AND co.name = 'Cameroon'; -- || 19712

-- q1_p063
-- predicates: f.creationdate BETWEEN 1287486205605 AND 1342670597313 AND co.name = 'Burma'
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
  ON tc.id = tht.tagclassid
WHERE f.creationdate BETWEEN 1287486205605 AND 1342670597313 AND co.name = 'Burma'; -- || 63050

-- q1_p064
-- predicates: f.creationdate >= 1305589168419 AND ci.name = 'Rạch_Giá' AND po.creationdate >= 1328021100728
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
  ON tc.id = tht.tagclassid
WHERE f.creationdate >= 1305589168419 AND ci.name = 'Rạch_Giá' AND po.creationdate >= 1328021100728; -- || 2451

-- q1_p065
-- predicates: p.language = 'es;en' AND f.creationdate >= 1305589168419 AND co.name = 'Venezuela'
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'es;en' AND f.creationdate >= 1305589168419 AND co.name = 'Venezuela'; -- || 5870

-- q1_p066
-- predicates: p.gender = 'male' AND f.explicitlydeleted = false AND co.name = 'Italy'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND f.explicitlydeleted = false AND co.name = 'Italy'; -- || 16700

-- q1_p067
-- predicates: p.language = 'en' AND f.explicitlydeleted = false AND co.name = 'Cameroon'
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
  ON tc.id = tht.tagclassid
WHERE p.language = 'en' AND f.explicitlydeleted = false AND co.name = 'Cameroon'; -- || 16869

-- q1_p068
-- predicates: p.gender = 'female' AND f.creationdate >= 1305589168419 AND co.name = 'Italy'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND f.creationdate >= 1305589168419 AND co.name = 'Italy'; -- || 2932

-- q1_p069
-- predicates: p.gender = 'male' AND crp.creationdate >= 1347725413194
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND crp.creationdate >= 1347725413194; -- || 2062702

-- q1_p070
-- predicates: f.title = 'Wall of Rahul Khan' AND po.length > 0 AND c.length BETWEEN 5 AND 79
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
  ON tc.id = tht.tagclassid
WHERE f.title = 'Wall of Rahul Khan' AND po.length > 0 AND c.length BETWEEN 5 AND 79; -- || 2330

-- q1_p071
-- predicates: p.gender = 'female' AND f.explicitlydeleted = false AND co.name = 'Venezuela'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND f.explicitlydeleted = false AND co.name = 'Venezuela'; -- || 3986

-- q1_p072
-- predicates: p.gender = 'male' AND p.language = 'en'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND p.language = 'en'; -- || 355105

-- q1_p073
-- predicates: p.gender = 'female' AND f.explicitlydeleted = false AND co.name = 'Italy'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND f.explicitlydeleted = false AND co.name = 'Italy'; -- || 12986

-- q1_p074
-- predicates: p.gender = 'female' AND f.explicitlydeleted = false AND co.name = 'Burma'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND f.explicitlydeleted = false AND co.name = 'Burma'; -- || 50603

-- q1_p075
-- predicates: p.gender = 'male' AND t.name = 'Charlemagne'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND t.name = 'Charlemagne'; -- || 5937

-- q1_p076
-- predicates: po.creationdate BETWEEN 1308052370341 AND 1343818164989 AND c.length BETWEEN 5 AND 79
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
  ON tc.id = tht.tagclassid
WHERE po.creationdate BETWEEN 1308052370341 AND 1343818164989 AND c.length BETWEEN 5 AND 79; -- || 1169041

-- q1_p077
-- predicates: f.explicitlydeleted = false AND ci.name = 'Rạch_Giá' AND po.length BETWEEN 0 AND 97
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
  ON tc.id = tht.tagclassid
WHERE f.explicitlydeleted = false AND ci.name = 'Rạch_Giá' AND po.length BETWEEN 0 AND 97; -- || 3383

-- q1_p078
-- predicates: p.gender = 'female' AND f.creationdate >= 1305589168419 AND co.name = 'Venezuela'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'female' AND f.creationdate >= 1305589168419 AND co.name = 'Venezuela'; -- || 2394

-- q1_p079
-- predicates: p.gender = 'male' AND po.creationdate >= 1328021100728
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND po.creationdate >= 1328021100728; -- || 1977657

-- q1_p080
-- predicates: p.gender = 'male' AND f.explicitlydeleted = false AND co.name = 'Venezuela'
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
  ON tc.id = tht.tagclassid
WHERE p.gender = 'male' AND f.explicitlydeleted = false AND co.name = 'Venezuela'; -- || 7574

-- q1_p081
-- predicates: po.length BETWEEN 0 AND 97 AND tc.name = 'Single' AND fhm.creationdate >= 1338094917792
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
  ON tc.id = tht.tagclassid
WHERE po.length BETWEEN 0 AND 97 AND tc.name = 'Single' AND fhm.creationdate >= 1338094917792; -- || 182120

-- q1_p082
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND cipc.src_type = 'City'
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
  ON tc.id = tht.tagclassid
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND cipc.src_type = 'City'; -- || 5221146

-- q1_p083
-- predicates: plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND cipc.src_type = 'City'
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
  ON tc.id = tht.tagclassid
WHERE plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND cipc.src_type = 'City'; -- || 3460800

-- q1_p084
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND cipc.src_type = 'City' AND tht.tagclassid IS NOT NULL
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
  ON tc.id = tht.tagclassid
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND cipc.src_type = 'City' AND tht.tagclassid IS NOT NULL; -- || 5221146

-- q1_p085
-- predicates: plc.explicitlydeleted = false AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
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
  ON tc.id = tht.tagclassid
WHERE plc.explicitlydeleted = false AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 5147930

-- q1_p086
-- predicates: plc.explicitlydeleted = false AND cipc.src_type = 'City' AND tht.tagclassid IS NOT NULL
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
  ON tc.id = tht.tagclassid
WHERE plc.explicitlydeleted = false AND cipc.src_type = 'City' AND tht.tagclassid IS NOT NULL; -- || 8630018

-- q1_p087
-- predicates: plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
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
  ON tc.id = tht.tagclassid
WHERE plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 3460800

-- q1_p088
-- predicates: cipc.src_type = 'City' AND tht.tagclassid IS NOT NULL AND fcp.creationdate BETWEEN 1308052370341 AND 1343818164989
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
  ON tc.id = tht.tagclassid
WHERE cipc.src_type = 'City' AND tht.tagclassid IS NOT NULL AND fcp.creationdate BETWEEN 1308052370341 AND 1343818164989; -- || 4082621

-- q1_p089
-- predicates: plc.explicitlydeleted = false AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND cipc.src_type = 'City'
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
  ON tc.id = tht.tagclassid
WHERE plc.explicitlydeleted = false AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND cipc.src_type = 'City'; -- || 5147930

-- q1_p090
-- predicates: plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND cipc.src_type = 'City' AND tht.tagclassid IS NOT NULL
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
  ON tc.id = tht.tagclassid
WHERE plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND cipc.src_type = 'City' AND tht.tagclassid IS NOT NULL; -- || 5781797

-- q1_p091
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND tht.tagclassid IS NOT NULL AND fcp.creationdate BETWEEN 1308052370341 AND 1343818164989
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
  ON tc.id = tht.tagclassid
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND tht.tagclassid IS NOT NULL AND fcp.creationdate BETWEEN 1308052370341 AND 1343818164989; -- || 2150491

-- q1_p092
-- predicates: cipc.src_type = 'City' AND plc.creationdate BETWEEN 1285358988937 AND 1347940587243
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
  ON tc.id = tht.tagclassid
WHERE cipc.src_type = 'City' AND plc.creationdate BETWEEN 1285358988937 AND 1347940587243; -- || 5781797

-- q1_p093
-- predicates: plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND tht.tagclassid IS NOT NULL AND fcp.creationdate BETWEEN 1308052370341 AND 1343818164989
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
  ON tc.id = tht.tagclassid
WHERE plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND tht.tagclassid IS NOT NULL AND fcp.creationdate BETWEEN 1308052370341 AND 1343818164989; -- || 2712479

-- q1_p094
-- predicates: plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND crp.creationdate >= 1347725413194 AND tc.name = 'MusicalArtist'
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
  ON tc.id = tht.tagclassid
WHERE plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND crp.creationdate >= 1347725413194 AND tc.name = 'MusicalArtist'; -- || 263537

-- q1_p095
-- predicates: plc.explicitlydeleted = false AND tht.tagclassid IS NOT NULL AND fcp.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE plc.explicitlydeleted = false AND tht.tagclassid IS NOT NULL AND fcp.explicitlydeleted = false; -- || 8628146

-- q1_p096
-- predicates: fcp.explicitlydeleted = false AND crp.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE fcp.explicitlydeleted = false AND crp.explicitlydeleted = false; -- || 8611389

-- q1_p097
-- predicates: cipc.src_type = 'City' AND tc.name = 'Song'
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
  ON tc.id = tht.tagclassid
WHERE cipc.src_type = 'City' AND tc.name = 'Song'; -- || 134127

-- q1_p098
-- predicates: tht.tagclassid IS NOT NULL AND fcp.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE tht.tagclassid IS NOT NULL AND fcp.explicitlydeleted = false; -- || 8749614

-- q1_p099
-- predicates: plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND tht.tagclassid IS NOT NULL AND fcp.explicitlydeleted = false
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
  ON tc.id = tht.tagclassid
WHERE plc.creationdate BETWEEN 1285358988937 AND 1347940587243 AND tht.tagclassid IS NOT NULL AND fcp.explicitlydeleted = false; -- || 5780602

-- q1_p100
-- predicates: cipc.src_type = 'City' AND po.language = 'en'
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
  ON tc.id = tht.tagclassid
WHERE cipc.src_type = 'City' AND po.language = 'en'; -- || 4298822

