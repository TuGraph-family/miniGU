-- p2 base true_cardinality: 2183
-- Generated with seed=20260115

-- p2_p001
-- predicates: plc2.explicitlydeleted = false AND p1.language = 'zh;en' AND chp.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.explicitlydeleted = false AND p1.language = 'zh;en' AND chp.explicitlydeleted = false; -- || 268

-- p2_p002
-- predicates: plc_like.creationdate >= 1351344255062 AND ci.name = 'Rạch_Giá' AND plc1.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND ci.name = 'Rạch_Giá' AND plc1.explicitlydeleted = false; -- || 2

-- p2_p003
-- predicates: plc2.explicitlydeleted = false AND c.creationdate >= 1350093427787 AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.explicitlydeleted = false AND c.creationdate >= 1350093427787 AND p2.language = 'en'; -- || 101

-- p2_p004
-- predicates: plc_like.explicitlydeleted = false AND p2.language = 'pt;en' AND plc2.creationdate BETWEEN 1285358988937 AND 1347940587243
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND p2.language = 'pt;en' AND plc2.creationdate BETWEEN 1285358988937 AND 1347940587243; -- || 41

-- p2_p005
-- predicates: plc_like.creationdate >= 1351344255062 AND p2.language = 'en' AND plc1.creationdate BETWEEN 1285358988937 AND 1347940587243
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND p2.language = 'en' AND plc1.creationdate BETWEEN 1285358988937 AND 1347940587243; -- || 62

-- p2_p006
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.gender = 'female' AND p1.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.gender = 'female' AND p1.language = 'zh;en'; -- || 95

-- p2_p007
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND plc_like.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND plc_like.explicitlydeleted = false; -- || 1388

-- p2_p008
-- predicates: plc_like.explicitlydeleted = false AND c.length BETWEEN 5 AND 79 AND plc1.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND c.length BETWEEN 5 AND 79 AND plc1.explicitlydeleted = false; -- || 588

-- p2_p009
-- predicates: plc1.explicitlydeleted = false AND c.length BETWEEN 5 AND 79 AND plc_like.creationdate >= 1351344255062
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND c.length BETWEEN 5 AND 79 AND plc_like.creationdate >= 1351344255062; -- || 244

-- p2_p010
-- predicates: plc1.explicitlydeleted = false AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND p2.language = 'en'; -- || 188

-- p2_p011
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.language = 'en'; -- || 131

-- p2_p012
-- predicates: chp.explicitlydeleted = false AND c.length >= 79 AND plc_like.creationdate >= 1351344255062
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.explicitlydeleted = false AND c.length >= 79 AND plc_like.creationdate >= 1351344255062; -- || 767

-- p2_p013
-- predicates: plc_like.explicitlydeleted = false AND c.length >= 79 AND p2.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND c.length >= 79 AND p2.language = 'pt;en'; -- || 76

-- p2_p014
-- predicates: plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.length >= 79 AND p1.gender = 'male'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.length >= 79 AND p1.gender = 'male'; -- || 542

-- p2_p015
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.creationdate >= 1350093427787; -- || 871

-- p2_p016
-- predicates: plc1.explicitlydeleted = false AND p1.gender = 'female' AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND p1.gender = 'female' AND p2.gender = 'female'; -- || 1112

-- p2_p017
-- predicates: plc_like.explicitlydeleted = false AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND p2.language = 'en'; -- || 184

-- p2_p018
-- predicates: chp.explicitlydeleted = false AND ci.name = 'Rạch_Giá' AND plc_like.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.explicitlydeleted = false AND ci.name = 'Rạch_Giá' AND plc_like.explicitlydeleted = false; -- || 2

-- p2_p019
-- predicates: plc_like.creationdate >= 1351344255062 AND p2.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND p2.language = 'pt;en'; -- || 49

-- p2_p020
-- predicates: chp.explicitlydeleted = false AND c.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.explicitlydeleted = false AND c.explicitlydeleted = false; -- || 2158

-- p2_p021
-- predicates: plc_like.creationdate >= 1351344255062 AND p2.language = 'zh;en' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND p2.language = 'zh;en' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 98

-- p2_p022
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND ci.name = 'Intramuros' AND plc1.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND ci.name = 'Intramuros' AND plc1.explicitlydeleted = false; -- || 4

-- p2_p023
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'male'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'male'; -- || 711

-- p2_p024
-- predicates: plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.gender = 'female' AND plc_like.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.gender = 'female' AND plc_like.explicitlydeleted = false; -- || 753

-- p2_p025
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.gender = 'female'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.gender = 'female'; -- || 773

-- p2_p026
-- predicates: chp.explicitlydeleted = false AND p1.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.explicitlydeleted = false AND p1.language = 'es;en'; -- || 168

-- p2_p027
-- predicates: plc_like.creationdate >= 1351344255062 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.explicitlydeleted = false; -- || 774

-- p2_p028
-- predicates: plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.creationdate >= 1350093427787; -- || 715

-- p2_p029
-- predicates: plc_like.explicitlydeleted = false AND ci.name = 'Pontianak'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND ci.name = 'Pontianak'; -- || 1

-- p2_p030
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'zh;en' AND plc2.creationdate BETWEEN 1285358988937 AND 1347940587243
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'zh;en' AND plc2.creationdate BETWEEN 1285358988937 AND 1347940587243; -- || 148

-- p2_p031
-- predicates: plc2.explicitlydeleted = false AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.explicitlydeleted = false AND p2.language = 'en'; -- || 188

-- p2_p032
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.gender = 'female' AND p2.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.gender = 'female' AND p2.language = 'pt;en'; -- || 21

-- p2_p033
-- predicates: plc_like.creationdate >= 1351344255062 AND c.explicitlydeleted = false AND p2.gender = 'male'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND c.explicitlydeleted = false AND p2.gender = 'male'; -- || 467

-- p2_p034
-- predicates: plc_like.explicitlydeleted = false AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND c.length BETWEEN 5 AND 79; -- || 591

-- p2_p035
-- predicates: plc_like.explicitlydeleted = false AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND p2.gender = 'female'; -- || 1130

-- p2_p036
-- predicates: plc1.explicitlydeleted = false AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND p1.language = 'pt;en'; -- || 104

-- p2_p037
-- predicates: plc1.explicitlydeleted = false AND ci.name = 'Pontianak'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND ci.name = 'Pontianak'; -- || 1

-- p2_p038
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.language = 'es;en' AND plc_like.creationdate >= 1351344255062
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.language = 'es;en' AND plc_like.creationdate >= 1351344255062; -- || 61

-- p2_p039
-- predicates: p2.gender = 'female' AND ci.name = 'Intramuros' AND plc2.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE p2.gender = 'female' AND ci.name = 'Intramuros' AND plc2.explicitlydeleted = false; -- || 4

-- p2_p040
-- predicates: plc2.explicitlydeleted = false AND p1.gender = 'male'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.explicitlydeleted = false AND p1.gender = 'male'; -- || 1016

-- p2_p041
-- predicates: plc2.explicitlydeleted = false AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.explicitlydeleted = false AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 1426

-- p2_p042
-- predicates: plc2.explicitlydeleted = false AND p2.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.explicitlydeleted = false AND p2.language = 'es;en'; -- || 167

-- p2_p043
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND ci.name = 'Rạch_Giá' AND plc_like.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND ci.name = 'Rạch_Giá' AND plc_like.explicitlydeleted = false; -- || 1

-- p2_p044
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.explicitlydeleted = false; -- || 1419

-- p2_p045
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'pt;en'; -- || 62

-- p2_p046
-- predicates: chp.explicitlydeleted = false AND p2.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.explicitlydeleted = false AND p2.language = 'pt;en'; -- || 105

-- p2_p047
-- predicates: chp.explicitlydeleted = false AND p1.language = 'en' AND plc2.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.explicitlydeleted = false AND p1.language = 'en' AND plc2.explicitlydeleted = false; -- || 183

-- p2_p048
-- predicates: plc1.explicitlydeleted = false AND ci.name = 'Intramuros'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND ci.name = 'Intramuros'; -- || 4

-- p2_p049
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.gender = 'male' AND plc2.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.gender = 'male' AND plc2.explicitlydeleted = false; -- || 669

-- p2_p050
-- predicates: p2.gender = 'male' AND p1.gender = 'male' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE p2.gender = 'male' AND p1.gender = 'male' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 642

-- p2_p051
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.explicitlydeleted = false; -- || 1462

-- p2_p052
-- predicates: plc_like.creationdate >= 1351344255062 AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND p2.language = 'en'; -- || 95

-- p2_p053
-- predicates: plc1.explicitlydeleted = false AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND c.creationdate >= 1350093427787; -- || 1067

-- p2_p054
-- predicates: plc_like.creationdate >= 1351344255062 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 781

-- p2_p055
-- predicates: plc_like.explicitlydeleted = false AND c.creationdate >= 1350093427787 AND plc1.creationdate BETWEEN 1285358988937 AND 1347940587243
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND c.creationdate >= 1350093427787 AND plc1.creationdate BETWEEN 1285358988937 AND 1347940587243; -- || 692

-- p2_p056
-- predicates: plc_like.creationdate >= 1351344255062 AND p2.language = 'pt;en' AND chp.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND p2.language = 'pt;en' AND chp.explicitlydeleted = false; -- || 49

-- p2_p057
-- predicates: plc1.explicitlydeleted = false AND c.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND c.explicitlydeleted = false; -- || 2147

-- p2_p058
-- predicates: chp.explicitlydeleted = false AND p1.gender = 'female' AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.explicitlydeleted = false AND p1.gender = 'female' AND c.length BETWEEN 5 AND 79; -- || 311

-- p2_p059
-- predicates: plc2.explicitlydeleted = false AND p1.gender = 'female' AND p2.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.explicitlydeleted = false AND p1.gender = 'female' AND p2.language = 'zh;en'; -- || 139

-- p2_p060
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.length >= 79
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND c.length >= 79; -- || 1103

-- p2_p061
-- predicates: plc1.explicitlydeleted = false AND p1.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND p1.language = 'zh;en'; -- || 270

-- p2_p062
-- predicates: plc_like.creationdate >= 1351344255062 AND p2.gender = 'female' AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND p2.gender = 'female' AND p1.language = 'pt;en'; -- || 21

-- p2_p063
-- predicates: plc2.explicitlydeleted = false AND ci.name = 'Rạch_Giá' AND plc_like.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.explicitlydeleted = false AND ci.name = 'Rạch_Giá' AND plc_like.explicitlydeleted = false; -- || 2

-- p2_p064
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p1.language = 'es;en'; -- || 117

-- p2_p065
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.gender = 'male' AND plc2.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND p2.gender = 'male' AND plc2.explicitlydeleted = false; -- || 671

-- p2_p066
-- predicates: plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.language = 'zh;en'; -- || 212

-- p2_p067
-- predicates: plc_like.explicitlydeleted = false AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND c.creationdate >= 1350093427787; -- || 1038

-- p2_p068
-- predicates: plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.length BETWEEN 5 AND 79 AND ci.name = 'Izmir'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.length BETWEEN 5 AND 79 AND ci.name = 'Izmir'; -- || 2

-- p2_p069
-- predicates: plc_like.creationdate >= 1351344255062 AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND p1.language = 'pt;en'; -- || 49

-- p2_p070
-- predicates: chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND ci.name = 'Intramuros'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.creationdate BETWEEN 1340327339000 AND 1356461058629 AND ci.name = 'Intramuros'; -- || 2

-- p2_p071
-- predicates: plc2.explicitlydeleted = false AND p2.gender = 'male'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.explicitlydeleted = false AND p2.gender = 'male'; -- || 1016

-- p2_p072
-- predicates: plc2.explicitlydeleted = false AND p1.language = 'en' AND ci.name = 'Intramuros'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.explicitlydeleted = false AND p1.language = 'en' AND ci.name = 'Intramuros'; -- || 4

-- p2_p073
-- predicates: plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 1022

-- p2_p074
-- predicates: chp.explicitlydeleted = false AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.explicitlydeleted = false AND p2.gender = 'female'; -- || 1148

-- p2_p075
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.length >= 79 AND ci.name = 'Izmir'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.length >= 79 AND ci.name = 'Izmir'; -- || 2

-- p2_p076
-- predicates: plc1.explicitlydeleted = false AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND c.length BETWEEN 5 AND 79; -- || 600

-- p2_p077
-- predicates: plc_like.creationdate >= 1351344255062 AND p2.gender = 'male' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND p2.gender = 'male' AND chp.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 376

-- p2_p078
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.explicitlydeleted = false AND plc1.creationdate BETWEEN 1285358988937 AND 1347940587243
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND c.explicitlydeleted = false AND plc1.creationdate BETWEEN 1285358988937 AND 1347940587243; -- || 1432

-- p2_p079
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'male' AND c.length >= 79
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'male' AND c.length >= 79; -- || 536

-- p2_p080
-- predicates: plc_like.creationdate >= 1351344255062 AND c.length >= 79
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND c.length >= 79; -- || 777

-- p2_p081
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.language = 'es;en'; -- || 91

-- p2_p082
-- predicates: plc_like.creationdate >= 1351344255062 AND p1.gender = 'female'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND p1.gender = 'female'; -- || 506

-- p2_p083
-- predicates: plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'male'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'male'; -- || 706

-- p2_p084
-- predicates: plc1.explicitlydeleted = false AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 1426

-- p2_p085
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND ci.name = 'Pontianak' AND c.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND ci.name = 'Pontianak' AND c.explicitlydeleted = false; -- || 1

-- p2_p086
-- predicates: chp.explicitlydeleted = false AND p1.gender = 'male'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.explicitlydeleted = false AND p1.gender = 'male'; -- || 1011

-- p2_p087
-- predicates: plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'female'; -- || 780

-- p2_p088
-- predicates: plc_like.creationdate >= 1351344255062 AND c.length >= 79 AND plc2.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND c.length >= 79 AND plc2.explicitlydeleted = false; -- || 775

-- p2_p089
-- predicates: plc1.explicitlydeleted = false AND p1.gender = 'female' AND plc2.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND p1.gender = 'female' AND plc2.explicitlydeleted = false; -- || 1156

-- p2_p090
-- predicates: plc_like.explicitlydeleted = false AND p1.gender = 'female' AND plc1.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND p1.gender = 'female' AND plc1.explicitlydeleted = false; -- || 1125

-- p2_p091
-- predicates: plc_like.creationdate >= 1351344255062 AND p1.language = 'pt;en' AND plc1.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND p1.language = 'pt;en' AND plc1.explicitlydeleted = false; -- || 49

-- p2_p092
-- predicates: plc_like.explicitlydeleted = false AND c.length >= 79
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND c.length >= 79; -- || 1624

-- p2_p093
-- predicates: plc1.explicitlydeleted = false AND p1.language = 'en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc1.explicitlydeleted = false AND p1.language = 'en'; -- || 185

-- p2_p094
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p2.gender = 'female'; -- || 768

-- p2_p095
-- predicates: chp.explicitlydeleted = false AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.explicitlydeleted = false AND p2.language = 'en'; -- || 187

-- p2_p096
-- predicates: chp.explicitlydeleted = false AND p1.language = 'en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE chp.explicitlydeleted = false AND p1.language = 'en'; -- || 184

-- p2_p097
-- predicates: plc_like.creationdate >= 1351344255062 AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.creationdate >= 1351344255062 AND c.length BETWEEN 5 AND 79; -- || 244

-- p2_p098
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.language = 'en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.language = 'en'; -- || 131

-- p2_p099
-- predicates: plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.gender = 'female' AND plc1.explicitlydeleted = false
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc2.creationdate BETWEEN 1285358988937 AND 1347940587243 AND p1.gender = 'female' AND plc1.explicitlydeleted = false; -- || 772

-- p2_p100
-- predicates: plc_like.explicitlydeleted = false AND p2.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  c.*,
  p2.*,
  ci.*,

  -- 边表
  plc_like.*,
  chp.*,
  plc1.*,
  plc2.*
FROM person AS p1
JOIN person_likes_comment AS plc_like
  ON plc_like.personid = p1.id
JOIN comment AS c
  ON c.id = plc_like.commentid
JOIN comment_hascreator_person AS chp
  ON chp.commentid = c.id
JOIN person AS p2
  ON p2.id = chp.personid
JOIN person_islocatedin_city AS plc1
  ON plc1.personid = p1.id
JOIN person_islocatedin_city AS plc2
  ON plc2.personid = p2.id
 AND plc2.cityid = plc1.cityid
JOIN city AS ci
  ON ci.id = plc1.cityid
WHERE plc_like.explicitlydeleted = false AND p2.language = 'pt;en'; -- || 101

