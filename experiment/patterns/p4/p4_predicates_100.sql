-- p4 base true_cardinality: 8128515
-- Generated with seed=20260115

-- p4_p001
-- predicates: fhm.explicitlydeleted = false AND p_creator.language = 'zh;en' AND f.creationdate BETWEEN 1305589168419 AND 1351610285871
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND p_creator.language = 'zh;en' AND f.creationdate BETWEEN 1305589168419 AND 1351610285871; -- || 352941

-- p4_p002
-- predicates: fcp.explicitlydeleted = false AND p_member.gender = 'male'
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND p_member.gender = 'male'; -- || 3974869

-- p4_p003
-- predicates: crp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79 AND po.language = 'en'
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79 AND po.language = 'en'; -- || 1512309

-- p4_p004
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND f.title = 'Wall of Rahul Singh'
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
  ON p_member.id = fhm.personid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND f.title = 'Wall of Rahul Singh'; -- || 25434

-- p4_p005
-- predicates: fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p_member.language = 'pt;en' AND chp.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p_member.language = 'pt;en' AND chp.explicitlydeleted = false; -- || 191365

-- p4_p006
-- predicates: chp.explicitlydeleted = false AND c.length <= 4
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND c.length <= 4; -- || 2999556

-- p4_p007
-- predicates: fcp.explicitlydeleted = false AND p_member.language = 'en' AND crp.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND p_member.language = 'en' AND crp.explicitlydeleted = false; -- || 665664

-- p4_p008
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_member.language = 'zh;en'
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_member.language = 'zh;en'; -- || 828377

-- p4_p009
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false; -- || 4818057

-- p4_p010
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND p_member.language = 'pt;en' AND fcp.creationdate BETWEEN 1308052370341 AND 1352177414840
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
  ON p_member.id = fhm.personid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND p_member.language = 'pt;en' AND fcp.creationdate BETWEEN 1308052370341 AND 1352177414840; -- || 138012

-- p4_p011
-- predicates: fcp.explicitlydeleted = false AND po.length BETWEEN 0 AND 97 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND po.length BETWEEN 0 AND 97 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 1478403

-- p4_p012
-- predicates: fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND f.title = 'Wall of Yang Li' AND po.length > 0
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
  ON p_member.id = fhm.personid
WHERE fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND f.title = 'Wall of Yang Li' AND po.length > 0; -- || 282

-- p4_p013
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND f.explicitlydeleted = false AND c.length BETWEEN 5 AND 79
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND f.explicitlydeleted = false AND c.length BETWEEN 5 AND 79; -- || 2175056

-- p4_p014
-- predicates: fhm.explicitlydeleted = false AND po.length > 0 AND fcp.creationdate BETWEEN 1308052370341 AND 1352177414840
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND po.length > 0 AND fcp.creationdate BETWEEN 1308052370341 AND 1352177414840; -- || 4966405

-- p4_p015
-- predicates: fhm.explicitlydeleted = false AND p_creator.gender = 'male'
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND p_creator.gender = 'male'; -- || 3917973

-- p4_p016
-- predicates: fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p_creator.gender = 'male'
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
  ON p_member.id = fhm.personid
WHERE fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p_creator.gender = 'male'; -- || 2476831

-- p4_p017
-- predicates: crp.explicitlydeleted = false AND p_member.language = 'pt;en' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND p_member.language = 'pt;en' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 182081

-- p4_p018
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_creator.language = 'zh;en'
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_creator.language = 'zh;en'; -- || 804709

-- p4_p019
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND po.language = 'zh' AND f.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND po.language = 'zh' AND f.explicitlydeleted = false; -- || 297785

-- p4_p020
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.length <= 4
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
  ON p_member.id = fhm.personid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.length <= 4; -- || 1845818

-- p4_p021
-- predicates: fhm.explicitlydeleted = false AND p_member.gender = 'female' AND po.language = 'en'
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND p_member.gender = 'female' AND po.language = 'en'; -- || 1985560

-- p4_p022
-- predicates: fcp.explicitlydeleted = false AND f.creationdate BETWEEN 1305589168419 AND 1351610285871 AND p_member.gender = 'female'
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND f.creationdate BETWEEN 1305589168419 AND 1351610285871 AND p_member.gender = 'female'; -- || 1265145

-- p4_p023
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND p_creator.gender = 'male' AND f.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND p_creator.gender = 'male' AND f.explicitlydeleted = false; -- || 2662654

-- p4_p024
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_creator.gender = 'male' AND p_member.language = 'zh;en'
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_creator.gender = 'male' AND p_member.language = 'zh;en'; -- || 409038

-- p4_p025
-- predicates: crp.explicitlydeleted = false AND c.length >= 79
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND c.length >= 79; -- || 2015960

-- p4_p026
-- predicates: fhm.explicitlydeleted = false AND po.length BETWEEN 0 AND 97
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND po.length BETWEEN 0 AND 97; -- || 2433939

-- p4_p027
-- predicates: fhm.explicitlydeleted = false AND f.title = 'Wall of Rahul Singh' AND p_creator.language = 'en'
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND f.title = 'Wall of Rahul Singh' AND p_creator.language = 'en'; -- || 1672

-- p4_p028
-- predicates: fcp.explicitlydeleted = false AND p_creator.language = 'en' AND crp.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND p_creator.language = 'en' AND crp.explicitlydeleted = false; -- || 670956

-- p4_p029
-- predicates: p_member.language = 'pt;en' AND f.creationdate BETWEEN 1305589168419 AND 1351610285871
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
  ON p_member.id = fhm.personid
WHERE p_member.language = 'pt;en' AND f.creationdate BETWEEN 1305589168419 AND 1351610285871; -- || 88679

-- p4_p030
-- predicates: chp.explicitlydeleted = false AND po.language = 'en'
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND po.language = 'en'; -- || 3880642

-- p4_p031
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_creator.gender = 'male' AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_creator.gender = 'male' AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082; -- || 1896527

-- p4_p032
-- predicates: fcp.explicitlydeleted = false AND po.language = 'zh'
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND po.language = 'zh'; -- || 513016

-- p4_p033
-- predicates: chp.explicitlydeleted = false AND c.creationdate >= 1350093427787
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND c.creationdate >= 1350093427787; -- || 3268700

-- p4_p034
-- predicates: fcp.explicitlydeleted = false AND f.title = 'Wall of Yang Li'
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND f.title = 'Wall of Yang Li'; -- || 890

-- p4_p035
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND p_member.gender = 'male'
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
  ON p_member.id = fhm.personid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND p_member.gender = 'male'; -- || 2641061

-- p4_p036
-- predicates: fhm.explicitlydeleted = false AND p_creator.gender = 'male' AND fcp.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND p_creator.gender = 'male' AND fcp.explicitlydeleted = false; -- || 3917443

-- p4_p037
-- predicates: chp.explicitlydeleted = false AND c.creationdate >= 1350093427787 AND po.length > 0
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND c.creationdate >= 1350093427787 AND po.length > 0; -- || 3268700

-- p4_p038
-- predicates: f.title = 'Wall of Rahul Singh' AND po.language = 'en' AND chp.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE f.title = 'Wall of Rahul Singh' AND po.language = 'en' AND chp.explicitlydeleted = false; -- || 42075

-- p4_p039
-- predicates: fcp.explicitlydeleted = false AND f.title = 'Wall of Rahul Singh'
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND f.title = 'Wall of Rahul Singh'; -- || 50992

-- p4_p040
-- predicates: fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.length > 0 AND fhm.creationdate BETWEEN 1321789849404 AND 1354857790702
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
  ON p_member.id = fhm.personid
WHERE fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.length > 0 AND fhm.creationdate BETWEEN 1321789849404 AND 1354857790702; -- || 3570153

-- p4_p041
-- predicates: fcp.explicitlydeleted = false AND p_member.language = 'es;en'
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND p_member.language = 'es;en'; -- || 540206

-- p4_p042
-- predicates: crp.explicitlydeleted = false AND po.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND po.explicitlydeleted = false; -- || 7993542

-- p4_p043
-- predicates: fcp.explicitlydeleted = false AND c.length <= 4
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND c.length <= 4; -- || 3049836

-- p4_p044
-- predicates: fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.language = 'en' AND p_member.language = 'en'
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
  ON p_member.id = fhm.personid
WHERE fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.language = 'en' AND p_member.language = 'en'; -- || 209584

-- p4_p045
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND po.length BETWEEN 0 AND 97
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND po.length BETWEEN 0 AND 97; -- || 1696780

-- p4_p046
-- predicates: fcp.explicitlydeleted = false AND p_creator.language = 'en'
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND p_creator.language = 'en'; -- || 683906

-- p4_p047
-- predicates: fhm.explicitlydeleted = false AND po.explicitlydeleted = false AND chp.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND po.explicitlydeleted = false AND chp.explicitlydeleted = false; -- || 7885217

-- p4_p048
-- predicates: chp.explicitlydeleted = false AND po.language = 'en' AND fhm.creationdate BETWEEN 1321789849404 AND 1354857790702
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND po.language = 'en' AND fhm.creationdate BETWEEN 1321789849404 AND 1354857790702; -- || 2725035

-- p4_p049
-- predicates: fcp.explicitlydeleted = false AND po.language = 'zh' AND chp.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND po.language = 'zh' AND chp.explicitlydeleted = false; -- || 505692

-- p4_p050
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false AND fcp.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false AND fcp.explicitlydeleted = false; -- || 4817561

-- p4_p051
-- predicates: fhm.explicitlydeleted = false AND po.language = 'en'
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND po.language = 'en'; -- || 3892700

-- p4_p052
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND po.creationdate BETWEEN 1308052370341 AND 1352177414840
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND po.creationdate BETWEEN 1308052370341 AND 1352177414840; -- || 3570153

-- p4_p053
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_creator.gender = 'female'
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
  ON p_member.id = fhm.personid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_creator.gender = 'female'; -- || 2454971

-- p4_p054
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND c.explicitlydeleted = false AND po.language = 'zh'
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND c.explicitlydeleted = false AND po.language = 'zh'; -- || 354295

-- p4_p055
-- predicates: fcp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79; -- || 3161856

-- p4_p056
-- predicates: fcp.explicitlydeleted = false AND p_creator.language = 'zh;en'
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
  ON p_member.id = fhm.personid
WHERE fcp.explicitlydeleted = false AND p_creator.language = 'zh;en'; -- || 1156736

-- p4_p057
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND f.title = 'Wall of Rahul Khan'
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
  ON p_member.id = fhm.personid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND f.title = 'Wall of Rahul Khan'; -- || 3349

-- p4_p058
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.explicitlydeleted = false; -- || 5284606

-- p4_p059
-- predicates: chp.explicitlydeleted = false AND p_creator.language = 'en'
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND p_creator.language = 'en'; -- || 670970

-- p4_p060
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_creator.language = 'pt;en'
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
  ON p_member.id = fhm.personid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_creator.language = 'pt;en'; -- || 191007

-- p4_p061
-- predicates: fhm.explicitlydeleted = false AND po.explicitlydeleted = false AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND po.explicitlydeleted = false AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 4823820

-- p4_p062
-- predicates: po.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p_member.language = 'zh;en'
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
  ON p_member.id = fhm.personid
WHERE po.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p_member.language = 'zh;en'; -- || 742701

-- p4_p063
-- predicates: fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.explicitlydeleted = false; -- || 5025058

-- p4_p064
-- predicates: fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.explicitlydeleted = false AND p_member.gender = 'male'
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
  ON p_member.id = fhm.personid
WHERE fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.explicitlydeleted = false AND p_member.gender = 'male'; -- || 2468783

-- p4_p065
-- predicates: crp.explicitlydeleted = false AND p_creator.language = 'zh;en' AND po.language = 'zh'
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND p_creator.language = 'zh;en' AND po.language = 'zh'; -- || 171067

-- p4_p066
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND po.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p_member.language = 'en'
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
  ON p_member.id = fhm.personid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND po.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p_member.language = 'en'; -- || 305761

-- p4_p067
-- predicates: c.creationdate >= 1350093427787 AND po.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE c.creationdate >= 1350093427787 AND po.explicitlydeleted = false; -- || 3323580

-- p4_p068
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_member.gender = 'male'
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
  ON p_member.id = fhm.personid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_member.gender = 'male'; -- || 2410546

-- p4_p069
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND f.title = 'Wall of Yang Li'
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND f.title = 'Wall of Yang Li'; -- || 555

-- p4_p070
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_creator.language = 'en'
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
  ON p_member.id = fhm.personid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_creator.language = 'en'; -- || 401685

-- p4_p071
-- predicates: fhm.explicitlydeleted = false AND c.creationdate >= 1350093427787 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND c.creationdate >= 1350093427787 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 2642137

-- p4_p072
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_member.language = 'en' AND c.length BETWEEN 5 AND 79
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
  ON p_member.id = fhm.personid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_member.language = 'en' AND c.length BETWEEN 5 AND 79; -- || 158815

-- p4_p073
-- predicates: chp.explicitlydeleted = false AND f.title = 'Wall of Yang Li'
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND f.title = 'Wall of Yang Li'; -- || 848

-- p4_p074
-- predicates: crp.explicitlydeleted = false AND f.title = 'Wall of Yang Li'
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND f.title = 'Wall of Yang Li'; -- || 848

-- p4_p075
-- predicates: p_creator.gender = 'male' AND f.title = 'Wall of Rahul Khan'
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
  ON p_member.id = fhm.personid
WHERE p_creator.gender = 'male' AND f.title = 'Wall of Rahul Khan'; -- || 2256

-- p4_p076
-- predicates: crp.explicitlydeleted = false AND p_creator.gender = 'male' AND fhm.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND p_creator.gender = 'male' AND fhm.explicitlydeleted = false; -- || 3849716

-- p4_p077
-- predicates: fhm.explicitlydeleted = false AND p_creator.gender = 'female'
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND p_creator.gender = 'female'; -- || 4100259

-- p4_p078
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND po.length > 0
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
  ON p_member.id = fhm.personid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND po.length > 0; -- || 5375361

-- p4_p079
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.length <= 4 AND p_member.language = 'pt;en'
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
  ON p_member.id = fhm.personid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.length <= 4 AND p_member.language = 'pt;en'; -- || 76840

-- p4_p080
-- predicates: crp.explicitlydeleted = false AND p_member.gender = 'male'
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND p_member.gender = 'male'; -- || 3910222

-- p4_p081
-- predicates: crp.explicitlydeleted = false AND p_creator.language = 'es;en'
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND p_creator.language = 'es;en'; -- || 532698

-- p4_p082
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND f.title = 'Wall of Yang Li' AND c.length >= 79
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
  ON p_member.id = fhm.personid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND f.title = 'Wall of Yang Li' AND c.length >= 79; -- || 120

-- p4_p083
-- predicates: chp.explicitlydeleted = false AND c.explicitlydeleted = false AND fhm.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND c.explicitlydeleted = false AND fhm.explicitlydeleted = false; -- || 7886334

-- p4_p084
-- predicates: chp.explicitlydeleted = false AND po.creationdate BETWEEN 1308052370341 AND 1352177414840
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND po.creationdate BETWEEN 1308052370341 AND 1352177414840; -- || 4942233

-- p4_p085
-- predicates: fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p_member.gender = 'male'
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
  ON p_member.id = fhm.personid
WHERE fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p_member.gender = 'male'; -- || 2468959

-- p4_p086
-- predicates: crp.explicitlydeleted = false AND po.language = 'en'
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND po.language = 'en'; -- || 3880642

-- p4_p087
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND p_member.language = 'en' AND c.length <= 4
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
  ON p_member.id = fhm.personid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND p_member.language = 'en' AND c.length <= 4; -- || 168854

-- p4_p088
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND c.creationdate >= 1350093427787 AND p_creator.language = 'zh;en'
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND c.creationdate >= 1350093427787 AND p_creator.language = 'zh;en'; -- || 337398

-- p4_p089
-- predicates: crp.explicitlydeleted = false AND p_member.gender = 'female' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND p_member.gender = 'female' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 2448079

-- p4_p090
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_member.gender = 'female' AND f.creationdate BETWEEN 1305589168419 AND 1351610285871
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_member.gender = 'female' AND f.creationdate BETWEEN 1305589168419 AND 1351610285871; -- || 1029811

-- p4_p091
-- predicates: fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND c.length <= 4 AND po.length > 0
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
  ON p_member.id = fhm.personid
WHERE fcp.creationdate BETWEEN 1308052370341 AND 1352177414840 AND c.length <= 4 AND po.length > 0; -- || 1895187

-- p4_p092
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND f.title = 'Wall of Yang Li' AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND f.title = 'Wall of Yang Li' AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082; -- || 257

-- p4_p093
-- predicates: chp.explicitlydeleted = false AND p_creator.gender = 'male' AND po.length BETWEEN 0 AND 97
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND p_creator.gender = 'male' AND po.length BETWEEN 0 AND 97; -- || 1177851

-- p4_p094
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_member.gender = 'female'
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
  ON p_member.id = fhm.personid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p_member.gender = 'female'; -- || 2490273

-- p4_p095
-- predicates: fhm.explicitlydeleted = false AND f.title = 'Wall of Rahul Singh' AND po.length BETWEEN 0 AND 97
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
  ON p_member.id = fhm.personid
WHERE fhm.explicitlydeleted = false AND f.title = 'Wall of Rahul Singh' AND po.length BETWEEN 0 AND 97; -- || 16723

-- p4_p096
-- predicates: crp.explicitlydeleted = false AND po.length > 0
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND po.length > 0; -- || 7994677

-- p4_p097
-- predicates: chp.explicitlydeleted = false AND p_creator.language = 'zh;en'
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND p_creator.language = 'zh;en'; -- || 1139791

-- p4_p098
-- predicates: chp.explicitlydeleted = false AND f.title = 'Wall of Yang Li' AND po.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE chp.explicitlydeleted = false AND f.title = 'Wall of Yang Li' AND po.explicitlydeleted = false; -- || 848

-- p4_p099
-- predicates: crp.explicitlydeleted = false AND po.length BETWEEN 0 AND 97 AND chp.explicitlydeleted = false
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
  ON p_member.id = fhm.personid
WHERE crp.explicitlydeleted = false AND po.length BETWEEN 0 AND 97 AND chp.explicitlydeleted = false; -- || 2430142

-- p4_p100
-- predicates: fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_creator.language = 'pt;en'
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
  ON p_member.id = fhm.personid
WHERE fhm.creationdate BETWEEN 1321789849404 AND 1354857790702 AND p_creator.language = 'pt;en'; -- || 216187

