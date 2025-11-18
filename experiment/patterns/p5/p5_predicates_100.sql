-- p5 base true_cardinality: 20
-- Generated with seed=20260115

-- p5_p001
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.gender = 'female' AND plc2.creationdate BETWEEN 1343776564773 AND 1356763691590
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
 AND chp2.personid = p1.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.gender = 'female' AND plc2.creationdate BETWEEN 1343776564773 AND 1356763691590; -- || 4

-- p5_p002
-- predicates: chp1.explicitlydeleted = false AND p2.gender = 'female'
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
 AND chp2.personid = p1.id
WHERE chp1.explicitlydeleted = false AND p2.gender = 'female'; -- || 9

-- p5_p003
-- predicates: rcc.creationdate >= 1351930981563 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate >= 1351930981563 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 7

-- p5_p004
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.creationdate >= 1350093427787 AND plc2.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.creationdate >= 1350093427787 AND plc2.explicitlydeleted = false; -- || 8

-- p5_p005
-- predicates: chp2.explicitlydeleted = false AND c1.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE chp2.explicitlydeleted = false AND c1.explicitlydeleted = false; -- || 20

-- p5_p006
-- predicates: chp1.explicitlydeleted = false AND p2.gender = 'male'
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
 AND chp2.personid = p1.id
WHERE chp1.explicitlydeleted = false AND p2.gender = 'male'; -- || 11

-- p5_p007
-- predicates: chp2.explicitlydeleted = false AND p1.language = 'zh;en'
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
 AND chp2.personid = p1.id
WHERE chp2.explicitlydeleted = false AND p1.language = 'zh;en'; -- || 4

-- p5_p008
-- predicates: plc2.creationdate >= 1351344255062 AND p2.gender = 'male' AND c2.length >= 79
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
 AND chp2.personid = p1.id
WHERE plc2.creationdate >= 1351344255062 AND p2.gender = 'male' AND c2.length >= 79; -- || 6

-- p5_p009
-- predicates: rcc.explicitlydeleted = false AND c2.length >= 79 AND plc1.creationdate BETWEEN 1343776564773 AND 1356763691590
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
 AND chp2.personid = p1.id
WHERE rcc.explicitlydeleted = false AND c2.length >= 79 AND plc1.creationdate BETWEEN 1343776564773 AND 1356763691590; -- || 10

-- p5_p010
-- predicates: plc2.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c1.creationdate >= 1350093427787 AND rcc.creationdate BETWEEN 1345196662740 AND 1356577357333
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
 AND chp2.personid = p1.id
WHERE plc2.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c1.creationdate >= 1350093427787 AND rcc.creationdate BETWEEN 1345196662740 AND 1356577357333; -- || 8

-- p5_p011
-- predicates: plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND p2.language = 'en'
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
 AND chp2.personid = p1.id
WHERE plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND p2.language = 'en'; -- || 3

-- p5_p012
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.gender = 'female'
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.gender = 'female'; -- || 6

-- p5_p013
-- predicates: rcc.explicitlydeleted = false AND c1.creationdate >= 1350093427787 AND plc2.creationdate BETWEEN 1343776564773 AND 1356763691590
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
 AND chp2.personid = p1.id
WHERE rcc.explicitlydeleted = false AND c1.creationdate >= 1350093427787 AND plc2.creationdate BETWEEN 1343776564773 AND 1356763691590; -- || 8

-- p5_p014
-- predicates: c1.length >= 79 AND p2.language = 'pt;en' AND plc2.creationdate BETWEEN 1343776564773 AND 1356763691590
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
 AND chp2.personid = p1.id
WHERE c1.length >= 79 AND p2.language = 'pt;en' AND plc2.creationdate BETWEEN 1343776564773 AND 1356763691590; -- || 1

-- p5_p015
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c1.length >= 79 AND c2.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c1.length >= 79 AND c2.length BETWEEN 5 AND 79; -- || 3

-- p5_p016
-- predicates: plc1.explicitlydeleted = false AND p1.language = 'pt;en'
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
 AND chp2.personid = p1.id
WHERE plc1.explicitlydeleted = false AND p1.language = 'pt;en'; -- || 1

-- p5_p017
-- predicates: plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 11

-- p5_p018
-- predicates: chp2.explicitlydeleted = false AND p1.gender = 'female'
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
 AND chp2.personid = p1.id
WHERE chp2.explicitlydeleted = false AND p1.gender = 'female'; -- || 11

-- p5_p019
-- predicates: chp2.explicitlydeleted = false AND p1.language = 'es;en'
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
 AND chp2.personid = p1.id
WHERE chp2.explicitlydeleted = false AND p1.language = 'es;en'; -- || 1

-- p5_p020
-- predicates: p2.gender = 'female' AND c1.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE p2.gender = 'female' AND c1.length BETWEEN 5 AND 79; -- || 2

-- p5_p021
-- predicates: chp1.explicitlydeleted = false AND p2.language = 'zh;en'
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
 AND chp2.personid = p1.id
WHERE chp1.explicitlydeleted = false AND p2.language = 'zh;en'; -- || 5

-- p5_p022
-- predicates: rcc.explicitlydeleted = false AND c2.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE rcc.explicitlydeleted = false AND c2.length BETWEEN 5 AND 79; -- || 6

-- p5_p023
-- predicates: plc1.explicitlydeleted = false AND p1.gender = 'female' AND c1.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE plc1.explicitlydeleted = false AND p1.gender = 'female' AND c1.length BETWEEN 5 AND 79; -- || 3

-- p5_p024
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.explicitlydeleted = false; -- || 12

-- p5_p025
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.language = 'pt;en'
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
 AND chp2.personid = p1.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.language = 'pt;en'; -- || 1

-- p5_p026
-- predicates: chp1.explicitlydeleted = false AND c1.creationdate >= 1350093427787 AND c2.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE chp1.explicitlydeleted = false AND c1.creationdate >= 1350093427787 AND c2.length BETWEEN 5 AND 79; -- || 3

-- p5_p027
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.length BETWEEN 5 AND 79; -- || 4

-- p5_p028
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 12

-- p5_p029
-- predicates: plc2.creationdate >= 1351344255062 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE plc2.creationdate >= 1351344255062 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 7

-- p5_p030
-- predicates: plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 13

-- p5_p031
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.gender = 'female' AND plc2.creationdate >= 1351344255062
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
 AND chp2.personid = p1.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.gender = 'female' AND plc2.creationdate >= 1351344255062; -- || 2

-- p5_p032
-- predicates: plc2.explicitlydeleted = false AND c1.length >= 79 AND c2.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE plc2.explicitlydeleted = false AND c1.length >= 79 AND c2.explicitlydeleted = false; -- || 17

-- p5_p033
-- predicates: rcc.explicitlydeleted = false AND p1.language = 'es;en' AND c1.creationdate >= 1350093427787
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
 AND chp2.personid = p1.id
WHERE rcc.explicitlydeleted = false AND p1.language = 'es;en' AND c1.creationdate >= 1350093427787; -- || 1

-- p5_p034
-- predicates: plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c1.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c1.length BETWEEN 5 AND 79; -- || 2

-- p5_p035
-- predicates: plc1.creationdate >= 1351344255062 AND c1.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE plc1.creationdate >= 1351344255062 AND c1.length BETWEEN 5 AND 79; -- || 1

-- p5_p036
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.explicitlydeleted = false AND p2.gender = 'male'
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
 AND chp2.personid = p1.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.explicitlydeleted = false AND p2.gender = 'male'; -- || 9

-- p5_p037
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 12

-- p5_p038
-- predicates: plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c1.length >= 79
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
 AND chp2.personid = p1.id
WHERE plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c1.length >= 79; -- || 12

-- p5_p039
-- predicates: plc1.explicitlydeleted = false AND c1.explicitlydeleted = false AND plc2.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE plc1.explicitlydeleted = false AND c1.explicitlydeleted = false AND plc2.explicitlydeleted = false; -- || 19

-- p5_p040
-- predicates: plc2.creationdate BETWEEN 1343776564773 AND 1356763691590 AND p1.language = 'zh;en'
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
 AND chp2.personid = p1.id
WHERE plc2.creationdate BETWEEN 1343776564773 AND 1356763691590 AND p1.language = 'zh;en'; -- || 2

-- p5_p041
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 10

-- p5_p042
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.gender = 'female'
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
 AND chp2.personid = p1.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.gender = 'female'; -- || 7

-- p5_p043
-- predicates: chp1.explicitlydeleted = false AND c2.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE chp1.explicitlydeleted = false AND c2.length BETWEEN 5 AND 79; -- || 6

-- p5_p044
-- predicates: chp2.explicitlydeleted = false AND p2.language = 'zh;en'
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
 AND chp2.personid = p1.id
WHERE chp2.explicitlydeleted = false AND p2.language = 'zh;en'; -- || 5

-- p5_p045
-- predicates: plc1.explicitlydeleted = false AND p2.gender = 'male' AND rcc.creationdate BETWEEN 1345196662740 AND 1356577357333
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
 AND chp2.personid = p1.id
WHERE plc1.explicitlydeleted = false AND p2.gender = 'male' AND rcc.creationdate BETWEEN 1345196662740 AND 1356577357333; -- || 8

-- p5_p046
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'pt;en'
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
 AND chp2.personid = p1.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'pt;en'; -- || 1

-- p5_p047
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 12

-- p5_p048
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND p1.gender = 'female' AND chp2.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND p1.gender = 'female' AND chp2.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 6

-- p5_p049
-- predicates: p2.gender = 'male' AND c1.explicitlydeleted = false AND p1.gender = 'female'
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
 AND chp2.personid = p1.id
WHERE p2.gender = 'male' AND c1.explicitlydeleted = false AND p1.gender = 'female'; -- || 7

-- p5_p050
-- predicates: rcc.creationdate >= 1351930981563 AND p2.gender = 'female'
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate >= 1351930981563 AND p2.gender = 'female'; -- || 4

-- p5_p051
-- predicates: rcc.creationdate >= 1351930981563 AND p2.language = 'en' AND plc2.creationdate >= 1351344255062
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate >= 1351930981563 AND p2.language = 'en' AND plc2.creationdate >= 1351344255062; -- || 3

-- p5_p052
-- predicates: plc1.creationdate >= 1351344255062 AND p2.gender = 'male' AND rcc.creationdate >= 1351930981563
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
 AND chp2.personid = p1.id
WHERE plc1.creationdate >= 1351344255062 AND p2.gender = 'male' AND rcc.creationdate >= 1351930981563; -- || 6

-- p5_p053
-- predicates: chp1.explicitlydeleted = false AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.language = 'zh;en'
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
 AND chp2.personid = p1.id
WHERE chp1.explicitlydeleted = false AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.language = 'zh;en'; -- || 1

-- p5_p054
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'de;en'
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
 AND chp2.personid = p1.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'de;en'; -- || 2

-- p5_p055
-- predicates: plc1.explicitlydeleted = false AND c2.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE plc1.explicitlydeleted = false AND c2.explicitlydeleted = false; -- || 19

-- p5_p056
-- predicates: plc1.explicitlydeleted = false AND p1.gender = 'female'
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
 AND chp2.personid = p1.id
WHERE plc1.explicitlydeleted = false AND p1.gender = 'female'; -- || 10

-- p5_p057
-- predicates: rcc.explicitlydeleted = false AND p2.language = 'zh;en' AND plc1.creationdate >= 1351344255062
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
 AND chp2.personid = p1.id
WHERE rcc.explicitlydeleted = false AND p2.language = 'zh;en' AND plc1.creationdate >= 1351344255062; -- || 3

-- p5_p058
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.length BETWEEN 5 AND 79; -- || 1

-- p5_p059
-- predicates: chp1.explicitlydeleted = false AND c2.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE chp1.explicitlydeleted = false AND c2.explicitlydeleted = false; -- || 20

-- p5_p060
-- predicates: plc2.explicitlydeleted = false AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND plc1.creationdate >= 1351344255062
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
 AND chp2.personid = p1.id
WHERE plc2.explicitlydeleted = false AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND plc1.creationdate >= 1351344255062; -- || 7

-- p5_p061
-- predicates: rcc.creationdate >= 1351930981563 AND p1.language = 'es;en' AND c1.length >= 79
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate >= 1351930981563 AND p1.language = 'es;en' AND c1.length >= 79; -- || 1

-- p5_p062
-- predicates: chp1.explicitlydeleted = false AND p1.language = 'es;en'
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
 AND chp2.personid = p1.id
WHERE chp1.explicitlydeleted = false AND p1.language = 'es;en'; -- || 1

-- p5_p063
-- predicates: plc2.explicitlydeleted = false AND c1.length >= 79
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
 AND chp2.personid = p1.id
WHERE plc2.explicitlydeleted = false AND c1.length >= 79; -- || 17

-- p5_p064
-- predicates: chp1.explicitlydeleted = false AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND rcc.creationdate >= 1351930981563
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
 AND chp2.personid = p1.id
WHERE chp1.explicitlydeleted = false AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND rcc.creationdate >= 1351930981563; -- || 7

-- p5_p065
-- predicates: chp2.explicitlydeleted = false AND p1.language = 'es;en' AND plc2.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE chp2.explicitlydeleted = false AND p1.language = 'es;en' AND plc2.explicitlydeleted = false; -- || 1

-- p5_p066
-- predicates: rcc.explicitlydeleted = false AND p2.language = 'en' AND plc1.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE rcc.explicitlydeleted = false AND p2.language = 'en' AND plc1.explicitlydeleted = false; -- || 3

-- p5_p067
-- predicates: p1.language = 'es;en' AND p2.gender = 'female'
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
 AND chp2.personid = p1.id
WHERE p1.language = 'es;en' AND p2.gender = 'female'; -- || 1

-- p5_p068
-- predicates: rcc.explicitlydeleted = false AND p1.gender = 'female' AND chp2.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE rcc.explicitlydeleted = false AND p1.gender = 'female' AND chp2.explicitlydeleted = false; -- || 11

-- p5_p069
-- predicates: plc2.explicitlydeleted = false AND c2.creationdate >= 1350093427787
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
 AND chp2.personid = p1.id
WHERE plc2.explicitlydeleted = false AND c2.creationdate >= 1350093427787; -- || 11

-- p5_p070
-- predicates: rcc.creationdate >= 1351930981563 AND c1.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate >= 1351930981563 AND c1.length BETWEEN 5 AND 79; -- || 1

-- p5_p071
-- predicates: plc1.explicitlydeleted = false AND c1.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE plc1.explicitlydeleted = false AND c1.length BETWEEN 5 AND 79; -- || 3

-- p5_p072
-- predicates: plc1.creationdate >= 1351344255062 AND p2.gender = 'male' AND p1.gender = 'male'
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
 AND chp2.personid = p1.id
WHERE plc1.creationdate >= 1351344255062 AND p2.gender = 'male' AND p1.gender = 'male'; -- || 2

-- p5_p073
-- predicates: plc2.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c1.explicitlydeleted = false AND p2.gender = 'female'
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
 AND chp2.personid = p1.id
WHERE plc2.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c1.explicitlydeleted = false AND p2.gender = 'female'; -- || 4

-- p5_p074
-- predicates: plc2.explicitlydeleted = false AND c2.length >= 79
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
 AND chp2.personid = p1.id
WHERE plc2.explicitlydeleted = false AND c2.length >= 79; -- || 16

-- p5_p075
-- predicates: plc2.creationdate >= 1351344255062 AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND rcc.creationdate >= 1351930981563
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
 AND chp2.personid = p1.id
WHERE plc2.creationdate >= 1351344255062 AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND rcc.creationdate >= 1351930981563; -- || 7

-- p5_p076
-- predicates: plc2.creationdate >= 1351344255062 AND c1.length >= 79
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
 AND chp2.personid = p1.id
WHERE plc2.creationdate >= 1351344255062 AND c1.length >= 79; -- || 9

-- p5_p077
-- predicates: chp1.explicitlydeleted = false AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE chp1.explicitlydeleted = false AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 14

-- p5_p078
-- predicates: chp2.explicitlydeleted = false AND c2.length BETWEEN 5 AND 79 AND rcc.creationdate BETWEEN 1345196662740 AND 1356577357333
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
 AND chp2.personid = p1.id
WHERE chp2.explicitlydeleted = false AND c2.length BETWEEN 5 AND 79 AND rcc.creationdate BETWEEN 1345196662740 AND 1356577357333; -- || 4

-- p5_p079
-- predicates: plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND p2.language = 'pt;en'
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
 AND chp2.personid = p1.id
WHERE plc1.creationdate BETWEEN 1343776564773 AND 1356763691590 AND p2.language = 'pt;en'; -- || 1

-- p5_p080
-- predicates: plc2.creationdate >= 1351344255062 AND p2.language = 'en'
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
 AND chp2.personid = p1.id
WHERE plc2.creationdate >= 1351344255062 AND p2.language = 'en'; -- || 3

-- p5_p081
-- predicates: rcc.creationdate >= 1351930981563 AND c1.length >= 79 AND chp2.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate >= 1351930981563 AND c1.length >= 79 AND chp2.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 7

-- p5_p082
-- predicates: c1.explicitlydeleted = false AND c2.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE c1.explicitlydeleted = false AND c2.explicitlydeleted = false; -- || 20

-- p5_p083
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.creationdate >= 1350093427787
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
 AND chp2.personid = p1.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.creationdate >= 1350093427787; -- || 8

-- p5_p084
-- predicates: chp2.explicitlydeleted = false AND p1.language = 'de;en'
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
 AND chp2.personid = p1.id
WHERE chp2.explicitlydeleted = false AND p1.language = 'de;en'; -- || 3

-- p5_p085
-- predicates: plc2.explicitlydeleted = false AND c2.length >= 79 AND plc1.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE plc2.explicitlydeleted = false AND c2.length >= 79 AND plc1.explicitlydeleted = false; -- || 15

-- p5_p086
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'zh;en'
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
 AND chp2.personid = p1.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'zh;en'; -- || 3

-- p5_p087
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c1.explicitlydeleted = false AND p1.language = 'zh;en'
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
 AND chp2.personid = p1.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c1.explicitlydeleted = false AND p1.language = 'zh;en'; -- || 3

-- p5_p088
-- predicates: plc1.explicitlydeleted = false AND c2.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE plc1.explicitlydeleted = false AND c2.length BETWEEN 5 AND 79; -- || 6

-- p5_p089
-- predicates: plc2.explicitlydeleted = false AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE plc2.explicitlydeleted = false AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 12

-- p5_p090
-- predicates: chp2.explicitlydeleted = false AND p1.gender = 'female' AND c1.length BETWEEN 5 AND 79
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
 AND chp2.personid = p1.id
WHERE chp2.explicitlydeleted = false AND p1.gender = 'female' AND c1.length BETWEEN 5 AND 79; -- || 4

-- p5_p091
-- predicates: plc2.creationdate >= 1351344255062 AND p1.gender = 'male' AND c2.length >= 79
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
 AND chp2.personid = p1.id
WHERE plc2.creationdate >= 1351344255062 AND p1.gender = 'male' AND c2.length >= 79; -- || 3

-- p5_p092
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.creationdate >= 1350093427787 AND plc1.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.creationdate >= 1350093427787 AND plc1.explicitlydeleted = false; -- || 8

-- p5_p093
-- predicates: plc2.creationdate BETWEEN 1343776564773 AND 1356763691590 AND p2.language = 'pt;en' AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND chp2.personid = p1.id
WHERE plc2.creationdate BETWEEN 1343776564773 AND 1356763691590 AND p2.language = 'pt;en' AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 1

-- p5_p094
-- predicates: chp2.explicitlydeleted = false AND p2.gender = 'female' AND plc2.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE chp2.explicitlydeleted = false AND p2.gender = 'female' AND plc2.explicitlydeleted = false; -- || 9

-- p5_p095
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'pt;en'
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
 AND chp2.personid = p1.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'pt;en'; -- || 1

-- p5_p096
-- predicates: plc1.explicitlydeleted = false AND c1.explicitlydeleted = false
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
 AND chp2.personid = p1.id
WHERE plc1.explicitlydeleted = false AND c1.explicitlydeleted = false; -- || 19

-- p5_p097
-- predicates: chp1.explicitlydeleted = false AND c1.creationdate >= 1350093427787
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
 AND chp2.personid = p1.id
WHERE chp1.explicitlydeleted = false AND c1.creationdate >= 1350093427787; -- || 12

-- p5_p098
-- predicates: plc1.explicitlydeleted = false AND p1.language = 'de;en'
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
 AND chp2.personid = p1.id
WHERE plc1.explicitlydeleted = false AND p1.language = 'de;en'; -- || 3

-- p5_p099
-- predicates: chp2.explicitlydeleted = false AND p2.language = 'en'
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
 AND chp2.personid = p1.id
WHERE chp2.explicitlydeleted = false AND p2.language = 'en'; -- || 3

-- p5_p100
-- predicates: plc2.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c2.length >= 79
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
 AND chp2.personid = p1.id
WHERE plc2.creationdate BETWEEN 1343776564773 AND 1356763691590 AND c2.length >= 79; -- || 9

