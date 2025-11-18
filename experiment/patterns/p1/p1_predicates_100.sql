-- p1 base true_cardinality: 3323
-- Generated with seed=20260115

-- p1_p001
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.explicitlydeleted = false; -- || 1468

-- p1_p002
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Augustine_of_Hippo' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Augustine_of_Hippo' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 103

-- p1_p003
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'pt;en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'pt;en'; -- || 99

-- p1_p004
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Plato'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Plato'; -- || 23

-- p1_p005
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'pt;en' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'pt;en' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240; -- || 63

-- p1_p006
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.gender = 'male'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.gender = 'male'; -- || 1180

-- p1_p007
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'George_W._Bush' AND p.language = 'en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'George_W._Bush' AND p.language = 'en'; -- || 1

-- p1_p008
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate >= 1350093427787 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate >= 1350093427787 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 1234

-- p1_p009
-- predicates: chp.explicitlydeleted = false AND c.explicitlydeleted = false AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND c.explicitlydeleted = false AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240; -- || 2150

-- p1_p010
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.explicitlydeleted = false; -- || 2078

-- p1_p011
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.gender = 'female' AND c.length >= 79
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.gender = 'female' AND c.length >= 79; -- || 796

-- p1_p012
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'en'; -- || 211

-- p1_p013
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 2078

-- p1_p014
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 2109

-- p1_p015
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'en' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'en' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 202

-- p1_p016
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'George_W._Bush'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'George_W._Bush'; -- || 19

-- p1_p017
-- predicates: p.language = 'zh;en' AND t.name = 'Jesus'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE p.language = 'zh;en' AND t.name = 'Jesus'; -- || 6

-- p1_p018
-- predicates: chp.explicitlydeleted = false AND t.name = 'Augustine_of_Hippo' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND t.name = 'Augustine_of_Hippo' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 106

-- p1_p019
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.length >= 79
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.length >= 79; -- || 1636

-- p1_p020
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'William_Shakespeare'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'William_Shakespeare'; -- || 18

-- p1_p021
-- predicates: chp.explicitlydeleted = false AND t.name = 'Jesus'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND t.name = 'Jesus'; -- || 21

-- p1_p022
-- predicates: chp.explicitlydeleted = false AND c.length >= 79
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND c.length >= 79; -- || 2513

-- p1_p023
-- predicates: t.name = 'Elizabeth_II' AND c.explicitlydeleted = false AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE t.name = 'Elizabeth_II' AND c.explicitlydeleted = false AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 13

-- p1_p024
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length >= 79 AND t.name = 'Plato'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length >= 79 AND t.name = 'Plato'; -- || 17

-- p1_p025
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Plato'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Plato'; -- || 24

-- p1_p026
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'es;en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'es;en'; -- || 114

-- p1_p027
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p.language = 'zh;en' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p.language = 'zh;en' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240; -- || 304

-- p1_p028
-- predicates: chp.explicitlydeleted = false AND p.gender = 'male'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND p.gender = 'male'; -- || 1714

-- p1_p029
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p.language = 'es;en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p.language = 'es;en'; -- || 108

-- p1_p030
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Plato' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Plato' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240; -- || 18

-- p1_p031
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'es;en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'es;en'; -- || 101

-- p1_p032
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Plato'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Plato'; -- || 24

-- p1_p033
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.length BETWEEN 5 AND 79; -- || 610

-- p1_p034
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 2142

-- p1_p035
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'zh;en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.language = 'zh;en'; -- || 399

-- p1_p036
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Jesus' AND p.gender = 'female'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Jesus' AND p.gender = 'female'; -- || 8

-- p1_p037
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Jesus'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Jesus'; -- || 16

-- p1_p038
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'William_Shakespeare'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'William_Shakespeare'; -- || 14

-- p1_p039
-- predicates: chp.explicitlydeleted = false AND t.name = 'Elizabeth_II'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND t.name = 'Elizabeth_II'; -- || 22

-- p1_p040
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'es;en' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'es;en' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 74

-- p1_p041
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'zh;en' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'zh;en' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 304

-- p1_p042
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Aristotle' AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Aristotle' AND c.length BETWEEN 5 AND 79; -- || 2

-- p1_p043
-- predicates: chp.explicitlydeleted = false AND c.explicitlydeleted = false AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND c.explicitlydeleted = false AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 2169

-- p1_p044
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Elizabeth_II'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Elizabeth_II'; -- || 13

-- p1_p045
-- predicates: chp.explicitlydeleted = false AND p.language = 'pt;en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND p.language = 'pt;en'; -- || 143

-- p1_p046
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Napoleon'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Napoleon'; -- || 18

-- p1_p047
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.gender = 'female'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.gender = 'female'; -- || 1023

-- p1_p048
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.gender = 'male'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.gender = 'male'; -- || 1207

-- p1_p049
-- predicates: c.length >= 79 AND p.gender = 'male'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE c.length >= 79 AND p.gender = 'male'; -- || 1314

-- p1_p050
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'pt;en' AND c.length >= 79
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'pt;en' AND c.length >= 79; -- || 63

-- p1_p051
-- predicates: chp.explicitlydeleted = false AND t.name = 'Elizabeth_II' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND t.name = 'Elizabeth_II' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240; -- || 17

-- p1_p052
-- predicates: chp.explicitlydeleted = false AND p.language = 'zh;en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND p.language = 'zh;en'; -- || 579

-- p1_p053
-- predicates: c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p.gender = 'female' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p.gender = 'female' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 982

-- p1_p054
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Jesus'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Jesus'; -- || 14

-- p1_p055
-- predicates: t.name = 'Jesus' AND c.length BETWEEN 5 AND 79 AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE t.name = 'Jesus' AND c.length BETWEEN 5 AND 79 AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 3

-- p1_p056
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.gender = 'female'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.gender = 'female'; -- || 976

-- p1_p057
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'John_F._Kennedy' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'John_F._Kennedy' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240; -- || 5

-- p1_p058
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Bill_Clinton'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Bill_Clinton'; -- || 9

-- p1_p059
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.gender = 'female' AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.gender = 'female' AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 662

-- p1_p060
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'William_Shakespeare' AND chp.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'William_Shakespeare' AND chp.explicitlydeleted = false; -- || 14

-- p1_p061
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.explicitlydeleted = false; -- || 2150

-- p1_p062
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate >= 1350093427787; -- || 1234

-- p1_p063
-- predicates: chp.explicitlydeleted = false AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 2110

-- p1_p064
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length >= 79
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length >= 79; -- || 1673

-- p1_p065
-- predicates: c.explicitlydeleted = false AND p.language = 'en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE c.explicitlydeleted = false AND p.language = 'en'; -- || 319

-- p1_p066
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Napoleon' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Napoleon' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 11

-- p1_p067
-- predicates: chp.explicitlydeleted = false AND t.name = 'William_Shakespeare'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND t.name = 'William_Shakespeare'; -- || 26

-- p1_p068
-- predicates: p.gender = 'male' AND c.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE p.gender = 'male' AND c.explicitlydeleted = false; -- || 1714

-- p1_p069
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate >= 1350093427787 AND chp.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate >= 1350093427787 AND chp.explicitlydeleted = false; -- || 1215

-- p1_p070
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'John_F._Kennedy' AND chp.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'John_F._Kennedy' AND chp.explicitlydeleted = false; -- || 13

-- p1_p071
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.length >= 79 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.length >= 79 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 1140

-- p1_p072
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Augustine_of_Hippo'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'Augustine_of_Hippo'; -- || 105

-- p1_p073
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Elizabeth_II'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Elizabeth_II'; -- || 17

-- p1_p074
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.explicitlydeleted = false; -- || 2169

-- p1_p075
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'George_W._Bush'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'George_W._Bush'; -- || 18

-- p1_p076
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 2109

-- p1_p077
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'zh;en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'zh;en'; -- || 454

-- p1_p078
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length >= 79 AND p.gender = 'female'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length >= 79 AND p.gender = 'female'; -- || 796

-- p1_p079
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.creationdate >= 1350093427787; -- || 974

-- p1_p080
-- predicates: chp.explicitlydeleted = false AND c.length >= 79 AND p.language = 'es;en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND c.length >= 79 AND p.language = 'es;en'; -- || 135

-- p1_p081
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p.gender = 'male' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p.gender = 'male' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 1127

-- p1_p082
-- predicates: chp.explicitlydeleted = false AND t.name = 'George_W._Bush'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND t.name = 'George_W._Bush'; -- || 28

-- p1_p083
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Augustine_of_Hippo' AND chp.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Augustine_of_Hippo' AND chp.explicitlydeleted = false; -- || 106

-- p1_p084
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.creationdate >= 1350093427787; -- || 1267

-- p1_p085
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p.language = 'zh;en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p.language = 'zh;en'; -- || 378

-- p1_p086
-- predicates: chp.explicitlydeleted = false AND p.gender = 'female' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND p.gender = 'female' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 1011

-- p1_p087
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'George_W._Bush' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'George_W._Bush' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 19

-- p1_p088
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'George_W._Bush' AND c.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'George_W._Bush' AND c.explicitlydeleted = false; -- || 18

-- p1_p089
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false; -- || 2110

-- p1_p090
-- predicates: chp.explicitlydeleted = false AND t.name = 'Napoleon'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND t.name = 'Napoleon'; -- || 24

-- p1_p091
-- predicates: chp.explicitlydeleted = false AND p.gender = 'female' AND c.length >= 79
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND p.gender = 'female' AND c.length >= 79; -- || 1220

-- p1_p092
-- predicates: chp.explicitlydeleted = false AND t.name = 'Augustine_of_Hippo'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.explicitlydeleted = false AND t.name = 'Augustine_of_Hippo'; -- || 187

-- p1_p093
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240; -- || 1468

-- p1_p094
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.gender = 'male' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND p.gender = 'male' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 1127

-- p1_p095
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Jesus'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'Jesus'; -- || 15

-- p1_p096
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'John_F._Kennedy'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t.name = 'John_F._Kennedy'; -- || 13

-- p1_p097
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.length >= 79 AND t.name = 'Augustine_of_Hippo'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.length >= 79 AND t.name = 'Augustine_of_Hippo'; -- || 101

-- p1_p098
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'William_Shakespeare' AND chp.explicitlydeleted = false
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t.name = 'William_Shakespeare' AND chp.explicitlydeleted = false; -- || 18

-- p1_p099
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'en'
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p.language = 'en'; -- || 247

-- p1_p100
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  c.*,
  p.*,
  t.*,

  -- 边表
  cht.*,
  chp.*,
  phi.*
FROM comment AS c
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t
  ON t.id = cht.tagid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p
  ON p.id = chp.personid
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p.id
 AND phi.tagid = t.id
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 1493

