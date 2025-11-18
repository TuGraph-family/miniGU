-- p8 base true_cardinality: 7741
-- Generated with seed=20260115

-- p8_p001
-- predicates: pkp.src BETWEEN 252947 AND 253760 AND t.name = 'Muammar_Gaddafi' AND chp1.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253760 AND t.name = 'Muammar_Gaddafi' AND chp1.explicitlydeleted = false; -- || 20

-- p8_p002
-- predicates: cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Khalid_Sheikh_Mohammed' AND p1.gender = 'male'
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
  ON cht1.dst = t.id
JOIN comment AS c1
  ON c1.id = cht1.src
JOIN comment_replyof_comment AS rcc
  ON rcc.src = c1.id
JOIN comment AS c2
  ON c2.id = rcc.dst
JOIN comment_hastag_tag AS cht2
  ON cht2.src = c2.id
 AND cht2.dst = t.id
JOIN comment_hascreator_person AS chp1
  ON chp1.src = c1.id
JOIN person AS p1
  ON p1.id = chp1.dst
JOIN comment_hascreator_person AS chp2
  ON chp2.src = c2.id
JOIN person AS p2
  ON p2.id = chp2.dst
JOIN person_knows_person AS pkp
  ON pkp.src = p1.id
 AND pkp.dst = p2.id
WHERE cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Khalid_Sheikh_Mohammed' AND p1.gender = 'male'; -- || 7

-- p8_p003
-- predicates: chp2.explicitlydeleted = false AND c1.length BETWEEN 5 AND 79
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
 AND pkp.dst = p2.id
WHERE chp2.explicitlydeleted = false AND c1.length BETWEEN 5 AND 79; -- || 2290

-- p8_p004
-- predicates: pkp.src BETWEEN 252947 AND 253760 AND t.name = 'Wolfgang_Amadeus_Mozart'
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253760 AND t.name = 'Wolfgang_Amadeus_Mozart'; -- || 7

-- p8_p005
-- predicates: cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p1.gender = 'male' AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND pkp.dst = p2.id
WHERE cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p1.gender = 'male' AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 2476

-- p8_p006
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'
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
 AND pkp.dst = p2.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'; -- || 5

-- p8_p007
-- predicates: cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p2.language = 'en' AND p1.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p2.language = 'en' AND p1.gender = 'male'; -- || 206

-- p8_p008
-- predicates: chp2.explicitlydeleted = false AND p1.language = 'de;en'
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
 AND pkp.dst = p2.id
WHERE chp2.explicitlydeleted = false AND p1.language = 'de;en'; -- || 548

-- p8_p009
-- predicates: pkp.dst <= 253343 AND t.name = 'Sonia_Gandhi'
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
 AND pkp.dst = p2.id
WHERE pkp.dst <= 253343 AND t.name = 'Sonia_Gandhi'; -- || 7

-- p8_p010
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Muammar_Gaddafi'
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
 AND pkp.dst = p2.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Muammar_Gaddafi'; -- || 14

-- p8_p011
-- predicates: chp2.explicitlydeleted = false AND t.name = 'Imelda_Marcos'
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
 AND pkp.dst = p2.id
WHERE chp2.explicitlydeleted = false AND t.name = 'Imelda_Marcos'; -- || 16

-- p8_p012
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND p2.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND p2.gender = 'female'; -- || 2533

-- p8_p013
-- predicates: pkp.src >= 253343 AND c2.length >= 79
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
 AND pkp.dst = p2.id
WHERE pkp.src >= 253343 AND c2.length >= 79; -- || 2897

-- p8_p014
-- predicates: cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c2.length BETWEEN 5 AND 79 AND p1.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c2.length BETWEEN 5 AND 79 AND p1.gender = 'male'; -- || 621

-- p8_p015
-- predicates: chp2.explicitlydeleted = false AND p2.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE chp2.explicitlydeleted = false AND p2.language = 'pt;en'; -- || 411

-- p8_p016
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.explicitlydeleted = false AND rcc.creationdate BETWEEN 1345196662740 AND 1356577357333
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
 AND pkp.dst = p2.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.explicitlydeleted = false AND rcc.creationdate BETWEEN 1345196662740 AND 1356577357333; -- || 4679

-- p8_p017
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND t.name = 'Sonia_Gandhi'
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
 AND pkp.dst = p2.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND t.name = 'Sonia_Gandhi'; -- || 8

-- p8_p018
-- predicates: rcc.explicitlydeleted = false AND t.name = 'Augustine_of_Hippo'
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
 AND pkp.dst = p2.id
WHERE rcc.explicitlydeleted = false AND t.name = 'Augustine_of_Hippo'; -- || 28

-- p8_p019
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.language = 'zh;en' AND chp2.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.language = 'zh;en' AND chp2.explicitlydeleted = false; -- || 721

-- p8_p020
-- predicates: chp1.explicitlydeleted = false AND c2.length >= 79 AND pkp.src BETWEEN 252947 AND 253760
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
 AND pkp.dst = p2.id
WHERE chp1.explicitlydeleted = false AND c2.length >= 79 AND pkp.src BETWEEN 252947 AND 253760; -- || 3052

-- p8_p021
-- predicates: cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p1.language = 'en'
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
 AND pkp.dst = p2.id
WHERE cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p1.language = 'en'; -- || 381

-- p8_p022
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Mariano_Rivera' AND p1.language = 'en'
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
 AND pkp.dst = p2.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Mariano_Rivera' AND p1.language = 'en'; -- || 1

-- p8_p023
-- predicates: cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c1.creationdate >= 1350093427787
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
 AND pkp.dst = p2.id
WHERE cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c1.creationdate >= 1350093427787; -- || 3319

-- p8_p024
-- predicates: pkp.src >= 253343 AND p2.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE pkp.src >= 253343 AND p2.language = 'pt;en'; -- || 222

-- p8_p025
-- predicates: cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c1.length >= 79
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
 AND pkp.dst = p2.id
WHERE cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c1.length >= 79; -- || 3404

-- p8_p026
-- predicates: chp2.explicitlydeleted = false AND t.name = 'Muammar_Gaddafi'
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
 AND pkp.dst = p2.id
WHERE chp2.explicitlydeleted = false AND t.name = 'Muammar_Gaddafi'; -- || 34

-- p8_p027
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND t.name = 'Adolf_Hitler' AND p1.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND t.name = 'Adolf_Hitler' AND p1.gender = 'female'; -- || 16

-- p8_p028
-- predicates: chp2.explicitlydeleted = false AND p1.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE chp2.explicitlydeleted = false AND p1.gender = 'male'; -- || 3737

-- p8_p029
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'zh;en'
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
 AND pkp.dst = p2.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'zh;en'; -- || 563

-- p8_p030
-- predicates: p1.language = 'zh;en' AND c1.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE p1.language = 'zh;en' AND c1.explicitlydeleted = false; -- || 929

-- p8_p031
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND p2.language = 'es;en' AND p1.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND p2.language = 'es;en' AND p1.gender = 'male'; -- || 174

-- p8_p032
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.length >= 79 AND cht2.creationdate BETWEEN 1338765911761 AND 1356307644022
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
 AND pkp.dst = p2.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.length >= 79 AND cht2.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 3210

-- p8_p033
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.creationdate >= 1350093427787
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
 AND pkp.dst = p2.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.creationdate >= 1350093427787; -- || 2389

-- p8_p034
-- predicates: pkp.src BETWEEN 252947 AND 253760 AND c2.creationdate >= 1350093427787 AND p1.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253760 AND c2.creationdate >= 1350093427787 AND p1.language = 'pt;en'; -- || 47

-- p8_p035
-- predicates: chp1.explicitlydeleted = false AND p1.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE chp1.explicitlydeleted = false AND p1.gender = 'male'; -- || 3669

-- p8_p036
-- predicates: rcc.explicitlydeleted = false AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Sonia_Gandhi'
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
 AND pkp.dst = p2.id
WHERE rcc.explicitlydeleted = false AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Sonia_Gandhi'; -- || 4

-- p8_p037
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'en'
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
 AND pkp.dst = p2.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'en'; -- || 403

-- p8_p038
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.gender = 'female'; -- || 2357

-- p8_p039
-- predicates: rcc.creationdate >= 1351930981563 AND c2.length BETWEEN 5 AND 79 AND cht1.creationdate BETWEEN 1338765911761 AND 1356307644022
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
 AND pkp.dst = p2.id
WHERE rcc.creationdate >= 1351930981563 AND c2.length BETWEEN 5 AND 79 AND cht1.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 745

-- p8_p040
-- predicates: chp1.explicitlydeleted = false AND t.name = 'Mariano_Rivera'
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
 AND pkp.dst = p2.id
WHERE chp1.explicitlydeleted = false AND t.name = 'Mariano_Rivera'; -- || 11

-- p8_p041
-- predicates: rcc.creationdate >= 1351930981563 AND t.name = 'Adolf_Hitler'
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
 AND pkp.dst = p2.id
WHERE rcc.creationdate >= 1351930981563 AND t.name = 'Adolf_Hitler'; -- || 18

-- p8_p042
-- predicates: cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c1.creationdate >= 1350093427787 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND pkp.dst = p2.id
WHERE cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c1.creationdate >= 1350093427787 AND c2.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 2959

-- p8_p043
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND p2.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND p2.language = 'pt;en'; -- || 87

-- p8_p044
-- predicates: cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p2.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p2.gender = 'female'; -- || 2979

-- p8_p045
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.explicitlydeleted = false AND chp2.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND pkp.dst = p2.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.explicitlydeleted = false AND chp2.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 4324

-- p8_p046
-- predicates: pkp.src >= 253343 AND t.name = 'Genghis_Khan'
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
 AND pkp.dst = p2.id
WHERE pkp.src >= 253343 AND t.name = 'Genghis_Khan'; -- || 6

-- p8_p047
-- predicates: cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p1.language = 'es;en' AND chp1.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p1.language = 'es;en' AND chp1.explicitlydeleted = false; -- || 480

-- p8_p048
-- predicates: c1.length >= 79 AND t.name = 'Adolf_Hitler'
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
 AND pkp.dst = p2.id
WHERE c1.length >= 79 AND t.name = 'Adolf_Hitler'; -- || 26

-- p8_p049
-- predicates: chp1.explicitlydeleted = false AND c2.creationdate >= 1350093427787 AND c1.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE chp1.explicitlydeleted = false AND c2.creationdate >= 1350093427787 AND c1.explicitlydeleted = false; -- || 2836

-- p8_p050
-- predicates: chp2.explicitlydeleted = false AND p2.language = 'zh;en' AND p1.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE chp2.explicitlydeleted = false AND p2.language = 'zh;en' AND p1.gender = 'male'; -- || 433

-- p8_p051
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND t.name = 'Wolfgang_Amadeus_Mozart'
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
 AND pkp.dst = p2.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND t.name = 'Wolfgang_Amadeus_Mozart'; -- || 8

-- p8_p052
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.creationdate >= 1350093427787 AND p2.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.creationdate >= 1350093427787 AND p2.gender = 'male'; -- || 1123

-- p8_p053
-- predicates: chp2.explicitlydeleted = false AND t.name = 'Imelda_Marcos' AND chp1.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND pkp.dst = p2.id
WHERE chp2.explicitlydeleted = false AND t.name = 'Imelda_Marcos' AND chp1.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 15

-- p8_p054
-- predicates: cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p2.language = 'zh;en'
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
 AND pkp.dst = p2.id
WHERE cht1.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p2.language = 'zh;en'; -- || 719

-- p8_p055
-- predicates: rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c1.length >= 79
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
 AND pkp.dst = p2.id
WHERE rcc.creationdate BETWEEN 1345196662740 AND 1356577357333 AND c1.length >= 79; -- || 3642

-- p8_p056
-- predicates: chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Georg_Wilhelm_Friedrich_Hegel' AND p1.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE chp1.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Georg_Wilhelm_Friedrich_Hegel' AND p1.gender = 'female'; -- || 2

-- p8_p057
-- predicates: rcc.creationdate >= 1351930981563 AND p1.language = 'zh;en'
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
 AND pkp.dst = p2.id
WHERE rcc.creationdate >= 1351930981563 AND p1.language = 'zh;en'; -- || 453

-- p8_p058
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Morocco' AND p2.language = 'pt;en'
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
 AND pkp.dst = p2.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Morocco' AND p2.language = 'pt;en'; -- || 1

-- p8_p059
-- predicates: cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p1.language = 'en' AND rcc.creationdate BETWEEN 1345196662740 AND 1356577357333
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
 AND pkp.dst = p2.id
WHERE cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p1.language = 'en' AND rcc.creationdate BETWEEN 1345196662740 AND 1356577357333; -- || 259

-- p8_p060
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.length BETWEEN 5 AND 79
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
 AND pkp.dst = p2.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c2.length BETWEEN 5 AND 79; -- || 1342

-- p8_p061
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND c1.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND c1.explicitlydeleted = false; -- || 3973

-- p8_p062
-- predicates: cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c1.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c1.explicitlydeleted = false; -- || 4608

-- p8_p063
-- predicates: chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629
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
 AND pkp.dst = p2.id
WHERE chp2.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c1.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 4324

-- p8_p064
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND c2.creationdate >= 1350093427787 AND c1.explicitlydeleted = false
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND c2.creationdate >= 1350093427787 AND c1.explicitlydeleted = false; -- || 1247

-- p8_p065
-- predicates: cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c2.length BETWEEN 5 AND 79 AND pkp.src >= 253343
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
 AND pkp.dst = p2.id
WHERE cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c2.length BETWEEN 5 AND 79 AND pkp.src >= 253343; -- || 673

-- p8_p066
-- predicates: p2.browserused = 'Safari' AND c1.browserused = 'Opera'
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
 AND pkp.dst = p2.id
WHERE p2.browserused = 'Safari' AND c1.browserused = 'Opera'; -- || 23

-- p8_p067
-- predicates: c2.browserused = 'Safari' AND c1.browserused = 'Chrome'
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
 AND pkp.dst = p2.id
WHERE c2.browserused = 'Safari' AND c1.browserused = 'Chrome'; -- || 116

-- p8_p068
-- predicates: c1.browserused = 'Safari' AND c2.browserused = 'Opera'
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
 AND pkp.dst = p2.id
WHERE c1.browserused = 'Safari' AND c2.browserused = 'Opera'; -- || 24

-- p8_p069
-- predicates: c2.browserused = 'Chrome' AND c1.browserused = 'Safari'
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
 AND pkp.dst = p2.id
WHERE c2.browserused = 'Chrome' AND c1.browserused = 'Safari'; -- || 125

-- p8_p070
-- predicates: pkp.src >= 253343 AND p1.browserused = 'Opera'
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
 AND pkp.dst = p2.id
WHERE pkp.src >= 253343 AND p1.browserused = 'Opera'; -- || 122

-- p8_p071
-- predicates: p2.browserused = 'Opera' AND c1.browserused = 'Opera'
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
 AND pkp.dst = p2.id
WHERE p2.browserused = 'Opera' AND c1.browserused = 'Opera'; -- || 16

-- p8_p072
-- predicates: cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c2.browserused = 'Internet Explorer'
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
 AND pkp.dst = p2.id
WHERE cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c2.browserused = 'Internet Explorer'; -- || 1444

-- p8_p073
-- predicates: pkp.dst <= 253343 AND p1.gender = 'male'
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
 AND pkp.dst = p2.id
WHERE pkp.dst <= 253343 AND p1.gender = 'male'; -- || 1916

-- p8_p074
-- predicates: c2.browserused = 'Opera' AND p2.browserused = 'Opera'
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
 AND pkp.dst = p2.id
WHERE c2.browserused = 'Opera' AND p2.browserused = 'Opera'; -- || 239

-- p8_p075
-- predicates: c1.browserused = 'Safari' AND p2.browserused = 'Chrome'
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
 AND pkp.dst = p2.id
WHERE c1.browserused = 'Safari' AND p2.browserused = 'Chrome'; -- || 125

-- p8_p076
-- predicates: c2.browserused = 'Firefox' AND p2.browserused = 'Chrome'
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
 AND pkp.dst = p2.id
WHERE c2.browserused = 'Firefox' AND p2.browserused = 'Chrome'; -- || 24

-- p8_p077
-- predicates: p2.browserused = 'Firefox' AND c1.browserused = 'Safari'
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
 AND pkp.dst = p2.id
WHERE p2.browserused = 'Firefox' AND c1.browserused = 'Safari'; -- || 160

-- p8_p078
-- predicates: p2.browserused = 'Opera' AND c1.browserused = 'Internet Explorer'
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
 AND pkp.dst = p2.id
WHERE p2.browserused = 'Opera' AND c1.browserused = 'Internet Explorer'; -- || 66

-- p8_p079
-- predicates: p2.browserused = 'Safari' AND c1.browserused = 'Chrome'
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
 AND pkp.dst = p2.id
WHERE p2.browserused = 'Safari' AND c1.browserused = 'Chrome'; -- || 118

-- p8_p080
-- predicates: pkp.dst BETWEEN 252947 AND 253760 AND p1.browserused = 'Chrome'
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
 AND pkp.dst = p2.id
WHERE pkp.dst BETWEEN 252947 AND 253760 AND p1.browserused = 'Chrome'; -- || 1096

-- p8_p081
-- predicates: pkp.dst <= 253343 AND p1.browserused = 'Chrome'
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
 AND pkp.dst = p2.id
WHERE pkp.dst <= 253343 AND p1.browserused = 'Chrome'; -- || 979

-- p8_p082
-- predicates: pkp.src >= 253343 AND p1.browserused = 'Firefox'
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
 AND pkp.dst = p2.id
WHERE pkp.src >= 253343 AND p1.browserused = 'Firefox'; -- || 1443

-- p8_p083
-- predicates: c2.browserused = 'Chrome' AND p2.browserused = 'Safari'
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
 AND pkp.dst = p2.id
WHERE c2.browserused = 'Chrome' AND p2.browserused = 'Safari'; -- || 2

-- p8_p084
-- predicates: c2.browserused = 'Firefox' AND p2.browserused = 'Safari'
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
 AND pkp.dst = p2.id
WHERE c2.browserused = 'Firefox' AND p2.browserused = 'Safari'; -- || 3

-- p8_p085
-- predicates: cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p2.browserused = 'Internet Explorer'
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
 AND pkp.dst = p2.id
WHERE cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p2.browserused = 'Internet Explorer'; -- || 1438

-- p8_p086
-- predicates: c1.browserused = 'Opera' AND p2.gender = 'female'
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
 AND pkp.dst = p2.id
WHERE c1.browserused = 'Opera' AND p2.gender = 'female'; -- || 149

-- p8_p087
-- predicates: p2.browserused = 'Chrome' AND c2.browserused = 'Firefox'
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
 AND pkp.dst = p2.id
WHERE p2.browserused = 'Chrome' AND c2.browserused = 'Firefox'; -- || 24

-- p8_p088
-- predicates: pkp.src >= 253343 AND p1.browserused = 'Chrome'
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
 AND pkp.dst = p2.id
WHERE pkp.src >= 253343 AND p1.browserused = 'Chrome'; -- || 1046

-- p8_p089
-- predicates: cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c2.browserused = 'Safari'
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
 AND pkp.dst = p2.id
WHERE cht2.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c2.browserused = 'Safari'; -- || 235

-- p8_p090
-- predicates: c2.browserused = 'Firefox' AND c1.browserused = 'Opera'
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
 AND pkp.dst = p2.id
WHERE c2.browserused = 'Firefox' AND c1.browserused = 'Opera'; -- || 87

-- p8_p091
-- predicates: p2.browserused = 'Chrome' AND c1.browserused = 'Firefox'
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
 AND pkp.dst = p2.id
WHERE p2.browserused = 'Chrome' AND c1.browserused = 'Firefox'; -- || 654

-- p8_p092
-- predicates: p2.browserused = 'Internet Explorer' AND c1.browserused = 'Safari'
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
 AND pkp.dst = p2.id
WHERE p2.browserused = 'Internet Explorer' AND c1.browserused = 'Safari'; -- || 214

-- p8_p093
-- predicates: p2.browserused = 'Internet Explorer' AND c1.browserused = 'Chrome'
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
 AND pkp.dst = p2.id
WHERE p2.browserused = 'Internet Explorer' AND c1.browserused = 'Chrome'; -- || 612

-- p8_p094
-- predicates: c2.browserused = 'Opera' AND c1.browserused = 'Safari'
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
 AND pkp.dst = p2.id
WHERE c2.browserused = 'Opera' AND c1.browserused = 'Safari'; -- || 24

-- p8_p095
-- predicates: c1.browserused = 'Internet Explorer' AND c2.browserused = 'Safari'
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
 AND pkp.dst = p2.id
WHERE c1.browserused = 'Internet Explorer' AND c2.browserused = 'Safari'; -- || 146

-- p8_p096
-- predicates: rcc.explicitlydeleted = false AND c2.browserused = 'Internet Explorer'
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
 AND pkp.dst = p2.id
WHERE rcc.explicitlydeleted = false AND c2.browserused = 'Internet Explorer'; -- || 2332

-- p8_p097
-- predicates: c1.browserused = 'Firefox' AND p2.browserused = 'Opera'
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
 AND pkp.dst = p2.id
WHERE c1.browserused = 'Firefox' AND p2.browserused = 'Opera'; -- || 90

-- p8_p098
-- predicates: c2.browserused = 'Safari' AND p2.browserused = 'Internet Explorer'
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
 AND pkp.dst = p2.id
WHERE c2.browserused = 'Safari' AND p2.browserused = 'Internet Explorer'; -- || 1

-- p8_p099
-- predicates: pkp.src BETWEEN 252947 AND 253760 AND p1.browserused = 'Internet Explorer'
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
 AND pkp.dst = p2.id
WHERE pkp.src BETWEEN 252947 AND 253760 AND p1.browserused = 'Internet Explorer'; -- || 1237

-- p8_p100
-- predicates: c2.browserused = 'Chrome' AND p2.browserused = 'Chrome'
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
 AND pkp.dst = p2.id
WHERE c2.browserused = 'Chrome' AND p2.browserused = 'Chrome'; -- || 1894

