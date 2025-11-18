-- Q2 base true_cardinality: 82840
-- Generated with seed=20260115

-- q2_p001
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND p1.gender = 'female' AND chp.creationdate >= 1350093427787
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND p1.gender = 'female' AND chp.creationdate >= 1350093427787; -- || 6818

-- q2_p002
-- predicates: chp.creationdate >= 1350093427787 AND c.length <= 4
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
 AND pkp.dst = p2.id
WHERE chp.creationdate >= 1350093427787 AND c.length <= 4; -- || 10069

-- q2_p003
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND c.length <= 4
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND c.length <= 4; -- || 19661

-- q2_p004
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.language = 'en'
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.language = 'en'; -- || 26389

-- q2_p005
-- predicates: chp.explicitlydeleted = false AND po.creationdate >= 1328021100728 AND pkp.src BETWEEN 252947 AND 253343
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
 AND pkp.dst = p2.id
WHERE chp.explicitlydeleted = false AND po.creationdate >= 1328021100728 AND pkp.src BETWEEN 252947 AND 253343; -- || 8062

-- q2_p006
-- predicates: php.explicitlydeleted = false AND p1.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND p1.gender = 'female'; -- || 42507

-- q2_p007
-- predicates: pkp.src BETWEEN 252947 AND 253343 AND p2.language = 'es;en' AND php.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253343 AND p2.language = 'es;en' AND php.explicitlydeleted = false; -- || 1614

-- q2_p008
-- predicates: chp.creationdate >= 1350093427787 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND crp.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE chp.creationdate >= 1350093427787 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND crp.explicitlydeleted = false; -- || 22434

-- q2_p009
-- predicates: pkp.src <= 253760 AND p1.language = 'es;en'
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
 AND pkp.dst = p2.id
WHERE pkp.src <= 253760 AND p1.language = 'es;en'; -- || 5135

-- q2_p010
-- predicates: chp.creationdate >= 1350093427787 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND pkp.dst = p2.id
WHERE chp.creationdate >= 1350093427787 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 22814

-- q2_p011
-- predicates: php.explicitlydeleted = false AND c.length >= 79 AND po.language = 'en'
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND c.length >= 79 AND po.language = 'en'; -- || 10375

-- q2_p012
-- predicates: chp.explicitlydeleted = false AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND php.creationdate BETWEEN 1308052370341 AND 1352177414840
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
 AND pkp.dst = p2.id
WHERE chp.explicitlydeleted = false AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND php.creationdate BETWEEN 1308052370341 AND 1352177414840; -- || 29900

-- q2_p013
-- predicates: pkp.src <= 253760 AND p1.language = 'en' AND chp.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE pkp.src <= 253760 AND p1.language = 'en' AND chp.explicitlydeleted = false; -- || 5272

-- q2_p014
-- predicates: crp.explicitlydeleted = false AND p1.gender = 'male' AND p2.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE crp.explicitlydeleted = false AND p1.gender = 'male' AND p2.gender = 'female'; -- || 21433

-- q2_p015
-- predicates: crp.creationdate >= 1347725413194 AND c.length >= 79 AND php.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND c.length >= 79 AND php.explicitlydeleted = false; -- || 8753

-- q2_p016
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND c.length BETWEEN 5 AND 79 AND crp.creationdate >= 1347725413194
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND c.length BETWEEN 5 AND 79 AND crp.creationdate >= 1347725413194; -- || 8476

-- q2_p017
-- predicates: crp.creationdate >= 1347725413194 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.language = 'pt;en'; -- || 1411

-- q2_p018
-- predicates: crp.explicitlydeleted = false AND p1.language = 'zh;en'
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
 AND pkp.dst = p2.id
WHERE crp.explicitlydeleted = false AND p1.language = 'zh;en'; -- || 10609

-- q2_p019
-- predicates: chp.creationdate >= 1350093427787 AND p1.language = 'en' AND po.language = 'en'
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
 AND pkp.dst = p2.id
WHERE chp.creationdate >= 1350093427787 AND p1.language = 'en' AND po.language = 'en'; -- || 1458

-- q2_p020
-- predicates: php.explicitlydeleted = false AND p2.language = 'en'
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND p2.language = 'en'; -- || 7421

-- q2_p021
-- predicates: crp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79
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
 AND pkp.dst = p2.id
WHERE crp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79; -- || 31582

-- q2_p022
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.explicitlydeleted = false; -- || 29900

-- q2_p023
-- predicates: crp.creationdate >= 1347725413194 AND c.length BETWEEN 5 AND 79
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND c.length BETWEEN 5 AND 79; -- || 13392

-- q2_p024
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND c.creationdate >= 1350093427787 AND p1.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND c.creationdate >= 1350093427787 AND p1.gender = 'female'; -- || 6818

-- q2_p025
-- predicates: chp.creationdate >= 1350093427787 AND p2.language = 'zh;en' AND p1.language = 'en'
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
 AND pkp.dst = p2.id
WHERE chp.creationdate >= 1350093427787 AND p2.language = 'zh;en' AND p1.language = 'en'; -- || 318

-- q2_p026
-- predicates: php.explicitlydeleted = false AND p1.language = 'zh;en'
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND p1.language = 'zh;en'; -- || 10790

-- q2_p027
-- predicates: php.explicitlydeleted = false AND p2.language = 'es;en'
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND p2.language = 'es;en'; -- || 7308

-- q2_p028
-- predicates: crp.creationdate >= 1347725413194 AND po.creationdate BETWEEN 1308052370341 AND 1343818164989
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND po.creationdate BETWEEN 1308052370341 AND 1343818164989; -- || 13992

-- q2_p029
-- predicates: crp.explicitlydeleted = false AND p1.gender = 'male' AND p2.language = 'es;en'
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
 AND pkp.dst = p2.id
WHERE crp.explicitlydeleted = false AND p1.gender = 'male' AND p2.language = 'es;en'; -- || 3518

-- q2_p030
-- predicates: crp.creationdate >= 1347725413194 AND p1.language = 'en'
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND p1.language = 'en'; -- || 3373

-- q2_p031
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND p2.language = 'zh;en' AND crp.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND p2.language = 'zh;en' AND crp.explicitlydeleted = false; -- || 6030

-- q2_p032
-- predicates: php.explicitlydeleted = false AND c.length <= 4
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND c.length <= 4; -- || 31122

-- q2_p033
-- predicates: crp.creationdate >= 1347725413194 AND po.length > 0
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND po.length > 0; -- || 34604

-- q2_p034
-- predicates: php.explicitlydeleted = false AND p1.gender = 'male' AND po.length BETWEEN 0 AND 97
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND p1.gender = 'male' AND po.length BETWEEN 0 AND 97; -- || 12072

-- q2_p035
-- predicates: php.explicitlydeleted = false AND po.creationdate >= 1328021100728 AND p2.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND po.creationdate >= 1328021100728 AND p2.language = 'pt;en'; -- || 1323

-- q2_p036
-- predicates: pkp.src <= 253760 AND c.length BETWEEN 5 AND 79
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
 AND pkp.dst = p2.id
WHERE pkp.src <= 253760 AND c.length BETWEEN 5 AND 79; -- || 24339

-- q2_p037
-- predicates: chp.explicitlydeleted = false AND p2.language = 'en'
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
 AND pkp.dst = p2.id
WHERE chp.explicitlydeleted = false AND p2.language = 'en'; -- || 7282

-- q2_p038
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p1.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p1.language = 'pt;en'; -- || 2591

-- q2_p039
-- predicates: pkp.src <= 253760 AND c.creationdate >= 1350093427787
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
 AND pkp.dst = p2.id
WHERE pkp.src <= 253760 AND c.creationdate >= 1350093427787; -- || 13512

-- q2_p040
-- predicates: crp.creationdate >= 1347725413194 AND p1.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND p1.language = 'pt;en'; -- || 2009

-- q2_p041
-- predicates: chp.explicitlydeleted = false AND po.length > 0
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
 AND pkp.dst = p2.id
WHERE chp.explicitlydeleted = false AND po.length > 0; -- || 81480

-- q2_p042
-- predicates: pkp.src BETWEEN 252947 AND 253343 AND p2.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253343 AND p2.language = 'pt;en'; -- || 876

-- q2_p043
-- predicates: crp.explicitlydeleted = false AND p2.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE crp.explicitlydeleted = false AND p2.gender = 'female'; -- || 43684

-- q2_p044
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND p2.language = 'zh;en'
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND p2.language = 'zh;en'; -- || 6126

-- q2_p045
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.length BETWEEN 0 AND 97 AND crp.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.length BETWEEN 0 AND 97 AND crp.explicitlydeleted = false; -- || 15245

-- q2_p046
-- predicates: pkp.src BETWEEN 252947 AND 253343 AND c.length <= 4 AND crp.creationdate >= 1347725413194
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253343 AND c.length <= 4 AND crp.creationdate >= 1347725413194; -- || 2198

-- q2_p047
-- predicates: c.length <= 4 AND p1.language = 'es;en'
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
 AND pkp.dst = p2.id
WHERE c.length <= 4 AND p1.language = 'es;en'; -- || 2511

-- q2_p048
-- predicates: crp.creationdate >= 1347725413194 AND p1.language = 'zh;en'
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND p1.language = 'zh;en'; -- || 4695

-- q2_p049
-- predicates: pkp.src BETWEEN 252947 AND 253343 AND po.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253343 AND po.explicitlydeleted = false; -- || 20880

-- q2_p050
-- predicates: pkp.src <= 253760 AND p1.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE pkp.src <= 253760 AND p1.gender = 'male'; -- || 29367

-- q2_p051
-- predicates: crp.explicitlydeleted = false AND p1.language = 'es;en'
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
 AND pkp.dst = p2.id
WHERE crp.explicitlydeleted = false AND p1.language = 'es;en'; -- || 6461

-- q2_p052
-- predicates: php.explicitlydeleted = false AND po.creationdate >= 1328021100728
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND po.creationdate >= 1328021100728; -- || 32386

-- q2_p053
-- predicates: chp.explicitlydeleted = false AND p1.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE chp.explicitlydeleted = false AND p1.gender = 'male'; -- || 39616

-- q2_p054
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p1.gender = 'female' AND po.creationdate >= 1328021100728
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p1.gender = 'female' AND po.creationdate >= 1328021100728; -- || 13951

-- q2_p055
-- predicates: php.explicitlydeleted = false AND p2.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND p2.gender = 'male'; -- || 38420

-- q2_p056
-- predicates: crp.explicitlydeleted = false AND p2.language = 'pt;en' AND chp.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE crp.explicitlydeleted = false AND p2.language = 'pt;en' AND chp.explicitlydeleted = false; -- || 3003

-- q2_p057
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND c.explicitlydeleted = false AND pkp.dst BETWEEN 252947 AND 253760
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND c.explicitlydeleted = false AND pkp.dst BETWEEN 252947 AND 253760; -- || 33397

-- q2_p058
-- predicates: pkp.src BETWEEN 252947 AND 253343 AND po.creationdate >= 1328021100728
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253343 AND po.creationdate >= 1328021100728; -- || 8205

-- q2_p059
-- predicates: chp.creationdate >= 1350093427787 AND p1.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE chp.creationdate >= 1350093427787 AND p1.gender = 'female'; -- || 13154

-- q2_p060
-- predicates: pkp.src <= 253760 AND c.length <= 4
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
 AND pkp.dst = p2.id
WHERE pkp.src <= 253760 AND c.length <= 4; -- || 23511

-- q2_p061
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p2.language = 'zh;en' AND po.language = 'en'
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p2.language = 'zh;en' AND po.language = 'en'; -- || 2455

-- q2_p062
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.explicitlydeleted = false; -- || 52311

-- q2_p063
-- predicates: chp.explicitlydeleted = false AND po.creationdate BETWEEN 1308052370341 AND 1343818164989 AND p2.language = 'es;en'
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
 AND pkp.dst = p2.id
WHERE chp.explicitlydeleted = false AND po.creationdate BETWEEN 1308052370341 AND 1343818164989 AND p2.language = 'es;en'; -- || 3474

-- q2_p064
-- predicates: pkp.src BETWEEN 252947 AND 253343 AND po.length > 0
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253343 AND po.length > 0; -- || 20882

-- q2_p065
-- predicates: crp.explicitlydeleted = false AND po.language = 'en'
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
 AND pkp.dst = p2.id
WHERE crp.explicitlydeleted = false AND po.language = 'en'; -- || 40018

-- q2_p066
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.explicitlydeleted = false AND p2.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.explicitlydeleted = false AND p2.gender = 'female'; -- || 28043

-- q2_p067
-- predicates: chp.explicitlydeleted = false AND po.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE chp.explicitlydeleted = false AND po.explicitlydeleted = false; -- || 81474

-- q2_p068
-- predicates: pkp.src BETWEEN 252947 AND 253343 AND p2.language = 'pt;en' AND crp.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253343 AND p2.language = 'pt;en' AND crp.explicitlydeleted = false; -- || 866

-- q2_p069
-- predicates: pkp.src <= 253760 AND p1.language = 'es;en' AND p2.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE pkp.src <= 253760 AND p1.language = 'es;en' AND p2.gender = 'female'; -- || 2747

-- q2_p070
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND po.length > 0
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND po.length > 0; -- || 41772

-- q2_p071
-- predicates: crp.explicitlydeleted = false AND po.language = 'en' AND p2.language = 'zh;en'
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
 AND pkp.dst = p2.id
WHERE crp.explicitlydeleted = false AND po.language = 'en' AND p2.language = 'zh;en'; -- || 3899

-- q2_p072
-- predicates: chp.explicitlydeleted = false AND po.creationdate BETWEEN 1308052370341 AND 1343818164989
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
 AND pkp.dst = p2.id
WHERE chp.explicitlydeleted = false AND po.creationdate BETWEEN 1308052370341 AND 1343818164989; -- || 41526

-- q2_p073
-- predicates: pkp.src <= 253760 AND po.length BETWEEN 0 AND 97
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
 AND pkp.dst = p2.id
WHERE pkp.src <= 253760 AND po.length BETWEEN 0 AND 97; -- || 18638

-- q2_p074
-- predicates: php.explicitlydeleted = false AND p1.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND p1.language = 'pt;en'; -- || 4069

-- q2_p075
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND c.length BETWEEN 5 AND 79 AND p2.language = 'en'
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND c.length BETWEEN 5 AND 79 AND p2.language = 'en'; -- || 1696

-- q2_p076
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.creationdate >= 1328021100728 AND crp.creationdate >= 1347725413194
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.creationdate >= 1328021100728 AND crp.creationdate >= 1347725413194; -- || 14142

-- q2_p077
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.length BETWEEN 0 AND 97 AND c.length <= 4
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.length BETWEEN 0 AND 97 AND c.length <= 4; -- || 5796

-- q2_p078
-- predicates: pkp.src <= 253760 AND p2.language = 'en' AND chp.creationdate >= 1350093427787
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
 AND pkp.dst = p2.id
WHERE pkp.src <= 253760 AND p2.language = 'en' AND chp.creationdate >= 1350093427787; -- || 1278

-- q2_p079
-- predicates: crp.explicitlydeleted = false AND p1.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE crp.explicitlydeleted = false AND p1.language = 'pt;en'; -- || 4005

-- q2_p080
-- predicates: pkp.src BETWEEN 252947 AND 253343 AND c.length BETWEEN 5 AND 79 AND chp.creationdate >= 1350093427787
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253343 AND c.length BETWEEN 5 AND 79 AND chp.creationdate >= 1350093427787; -- || 1604

-- q2_p081
-- predicates: crp.creationdate >= 1347725413194 AND p2.language = 'en' AND c.length <= 4
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND p2.language = 'en' AND c.length <= 4; -- || 1258

-- q2_p082
-- predicates: chp.creationdate >= 1350093427787 AND p1.language = 'en'
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
 AND pkp.dst = p2.id
WHERE chp.creationdate >= 1350093427787 AND p1.language = 'en'; -- || 2749

-- q2_p083
-- predicates: crp.creationdate >= 1347725413194 AND po.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND po.explicitlydeleted = false; -- || 34603

-- q2_p084
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND p2.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND p2.gender = 'male'; -- || 20506

-- q2_p085
-- predicates: c.length BETWEEN 5 AND 79 AND po.language = 'en'
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
 AND pkp.dst = p2.id
WHERE c.length BETWEEN 5 AND 79 AND po.language = 'en'; -- || 15807

-- q2_p086
-- predicates: crp.explicitlydeleted = false AND p2.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE crp.explicitlydeleted = false AND p2.language = 'pt;en'; -- || 3003

-- q2_p087
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND p1.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND p1.gender = 'male'; -- || 20273

-- q2_p088
-- predicates: crp.creationdate >= 1347725413194 AND p2.gender = 'male' AND php.creationdate BETWEEN 1308052370341 AND 1352177414840
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND p2.gender = 'male' AND php.creationdate BETWEEN 1308052370341 AND 1352177414840; -- || 9798

-- q2_p089
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.creationdate BETWEEN 1308052370341 AND 1343818164989 AND pkp.src <= 253760
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND po.creationdate BETWEEN 1308052370341 AND 1343818164989 AND pkp.src <= 253760; -- || 31892

-- q2_p090
-- predicates: pkp.src <= 253760 AND po.creationdate >= 1328021100728
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
 AND pkp.dst = p2.id
WHERE pkp.src <= 253760 AND po.creationdate >= 1328021100728; -- || 24385

-- q2_p091
-- predicates: php.explicitlydeleted = false AND p1.language = 'es;en'
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND p1.language = 'es;en'; -- || 6573

-- q2_p092
-- predicates: pkp.src <= 253760 AND p2.language = 'pt;en' AND c.length >= 79
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
 AND pkp.dst = p2.id
WHERE pkp.src <= 253760 AND p2.language = 'pt;en' AND c.length >= 79; -- || 553

-- q2_p093
-- predicates: crp.creationdate >= 1347725413194 AND p2.language = 'en' AND chp.creationdate >= 1350093427787
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND p2.language = 'en' AND chp.creationdate >= 1350093427787; -- || 2510

-- q2_p094
-- predicates: php.explicitlydeleted = false AND po.creationdate BETWEEN 1308052370341 AND 1343818164989 AND p1.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND po.creationdate BETWEEN 1308052370341 AND 1343818164989 AND p1.gender = 'male'; -- || 20538

-- q2_p095
-- predicates: crp.creationdate >= 1347725413194 AND p1.gender = 'male' AND po.language = 'en'
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
 AND pkp.dst = p2.id
WHERE crp.creationdate >= 1347725413194 AND p1.gender = 'male' AND po.language = 'en'; -- || 9007

-- q2_p096
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p1.language = 'es;en'
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
 AND pkp.dst = p2.id
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND p1.language = 'es;en'; -- || 3992

-- q2_p097
-- predicates: chp.creationdate >= 1350093427787 AND po.length BETWEEN 0 AND 97
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
 AND pkp.dst = p2.id
WHERE chp.creationdate >= 1350093427787 AND po.length BETWEEN 0 AND 97; -- || 7895

-- q2_p098
-- predicates: chp.creationdate >= 1350093427787 AND c.length >= 79
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
 AND pkp.dst = p2.id
WHERE chp.creationdate >= 1350093427787 AND c.length >= 79; -- || 6854

-- q2_p099
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND po.length > 0 AND c.creationdate >= 1350093427787
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND po.length > 0 AND c.creationdate >= 1350093427787; -- || 13701

-- q2_p100
-- predicates: php.explicitlydeleted = false AND p2.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE php.explicitlydeleted = false AND p2.language = 'pt;en'; -- || 3049

