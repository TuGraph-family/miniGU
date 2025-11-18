-- Q4 base true_cardinality: 290601
-- Generated with seed=20260115

-- q4_p001
-- predicates: crp.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989 AND liker.gender = 'male'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989 AND liker.gender = 'male'; -- || 63317

-- q4_p002
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND liker.language = 'es;en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND liker.language = 'es;en'; -- || 8085

-- q4_p003
-- predicates: crp.explicitlydeleted = false AND liker.language = 'es;en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND liker.language = 'es;en'; -- || 18904

-- q4_p004
-- predicates: crp.explicitlydeleted = false AND c.length >= 79 AND php.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND c.length >= 79 AND php.explicitlydeleted = false; -- || 76065

-- q4_p005
-- predicates: crp.explicitlydeleted = false AND liker.language = 'pt;en' AND php.creationdate BETWEEN 1308052370341 AND 1352177414840
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND liker.language = 'pt;en' AND php.creationdate BETWEEN 1308052370341 AND 1352177414840; -- || 7393

-- q4_p006
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND liker.language = 'en' AND t.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND liker.language = 'en' AND t.name = 'Wolfgang_Amadeus_Mozart'; -- || 156

-- q4_p007
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Muammar_Gaddafi'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Muammar_Gaddafi'; -- || 768

-- q4_p008
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'; -- || 675

-- q4_p009
-- predicates: plp.explicitlydeleted = false AND liker.language = 'en' AND m.creationdate >= 1328021100728
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.explicitlydeleted = false AND liker.language = 'en' AND m.creationdate >= 1328021100728; -- || 14560

-- q4_p010
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND creator.language = 'es;en' AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND creator.language = 'es;en' AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082; -- || 12477

-- q4_p011
-- predicates: php.explicitlydeleted = false AND m.language = 'en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND m.language = 'en'; -- || 166059

-- q4_p012
-- predicates: plp.creationdate >= 1338452282578 AND t.name = 'Genghis_Khan'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.creationdate >= 1338452282578 AND t.name = 'Genghis_Khan'; -- || 555

-- q4_p013
-- predicates: plp.explicitlydeleted = false AND c.length <= 4
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.explicitlydeleted = false AND c.length <= 4; -- || 105335

-- q4_p014
-- predicates: php.explicitlydeleted = false AND t.name = 'Hamid_Karzai'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND t.name = 'Hamid_Karzai'; -- || 292

-- q4_p015
-- predicates: php.explicitlydeleted = false AND liker.language = 'es;en' AND creator.language = 'en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND liker.language = 'es;en' AND creator.language = 'en'; -- || 1562

-- q4_p016
-- predicates: php.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989 AND crp.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989 AND crp.explicitlydeleted = false; -- || 131725

-- q4_p017
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND c.length <= 4
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND c.length <= 4; -- || 46892

-- q4_p018
-- predicates: php.explicitlydeleted = false AND m.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND m.explicitlydeleted = false; -- || 290601

-- q4_p019
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Wolfgang_Amadeus_Mozart'; -- || 2660

-- q4_p020
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND c.length >= 79 AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND c.length >= 79 AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082; -- || 18286

-- q4_p021
-- predicates: crp.explicitlydeleted = false AND m.language = 'en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND m.language = 'en'; -- || 164726

-- q4_p022
-- predicates: liker.language = 'en' AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE liker.language = 'en' AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'; -- || 397

-- q4_p023
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND liker.language = 'en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND liker.language = 'en'; -- || 16240

-- q4_p024
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Genghis_Khan' AND m.language = 'en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Genghis_Khan' AND m.language = 'en'; -- || 483

-- q4_p025
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Hamid_Karzai' AND liker.gender = 'female'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Hamid_Karzai' AND liker.gender = 'female'; -- || 57

-- q4_p026
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.language = 'en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.language = 'en'; -- || 115817

-- q4_p027
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Hamid_Karzai' AND liker.gender = 'female'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Hamid_Karzai' AND liker.gender = 'female'; -- || 67

-- q4_p028
-- predicates: php.explicitlydeleted = false AND m.creationdate >= 1328021100728 AND t.name = 'Adolf_Hitler'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND m.creationdate >= 1328021100728 AND t.name = 'Adolf_Hitler'; -- || 439

-- q4_p029
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND creator.gender = 'male'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND creator.gender = 'male'; -- || 89622

-- q4_p030
-- predicates: liker.gender = 'female' AND creator.language = 'zh;en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE liker.gender = 'female' AND creator.language = 'zh;en'; -- || 13211

-- q4_p031
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Mariano_Rivera'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Mariano_Rivera'; -- || 1170

-- q4_p032
-- predicates: php.explicitlydeleted = false AND m.language = 'en' AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND m.language = 'en' AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082; -- || 115817

-- q4_p033
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND m.length BETWEEN 0 AND 97
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND m.length BETWEEN 0 AND 97; -- || 36385

-- q4_p034
-- predicates: plp.creationdate >= 1338452282578 AND creator.language = 'en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.creationdate >= 1338452282578 AND creator.language = 'en'; -- || 6782

-- q4_p035
-- predicates: plp.creationdate >= 1338452282578 AND m.explicitlydeleted = false AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.creationdate >= 1338452282578 AND m.explicitlydeleted = false AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 123356

-- q4_p036
-- predicates: crp.explicitlydeleted = false AND m.creationdate >= 1328021100728
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND m.creationdate >= 1328021100728; -- || 176835

-- q4_p037
-- predicates: c.length >= 79 AND t.name = 'Mariano_Rivera' AND m.creationdate >= 1328021100728
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE c.length >= 79 AND t.name = 'Mariano_Rivera' AND m.creationdate >= 1328021100728; -- || 4

-- q4_p038
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND m.explicitlydeleted = false AND liker.gender = 'male'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND m.explicitlydeleted = false AND liker.gender = 'male'; -- || 106402

-- q4_p039
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND creator.language = 'pt;en' AND php.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND creator.language = 'pt;en' AND php.explicitlydeleted = false; -- || 3730

-- q4_p040
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND creator.gender = 'female'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND creator.gender = 'female'; -- || 107229

-- q4_p041
-- predicates: crp.explicitlydeleted = false AND creator.language = 'zh;en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND creator.language = 'zh;en'; -- || 26084

-- q4_p042
-- predicates: plp.explicitlydeleted = false AND liker.language = 'pt;en' AND creator.gender = 'male'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.explicitlydeleted = false AND liker.language = 'pt;en' AND creator.gender = 'male'; -- || 5475

-- q4_p043
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND c.length >= 79
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND c.length >= 79; -- || 34744

-- q4_p044
-- predicates: plp.creationdate >= 1338452282578 AND m.creationdate BETWEEN 1308052370341 AND 1343818164989 AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.creationdate >= 1338452282578 AND m.creationdate BETWEEN 1308052370341 AND 1343818164989 AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781; -- || 4907

-- q4_p045
-- predicates: plp.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989; -- || 129067

-- q4_p046
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'; -- || 3312

-- q4_p047
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND liker.language = 'en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND liker.language = 'en'; -- || 10368

-- q4_p048
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND liker.language = 'zh;en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND liker.language = 'zh;en'; -- || 27399

-- q4_p049
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND m.creationdate >= 1328021100728
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND m.creationdate >= 1328021100728; -- || 37512

-- q4_p050
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND creator.language = 'es;en' AND php.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND creator.language = 'es;en' AND php.explicitlydeleted = false; -- || 15068

-- q4_p051
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Augustine_of_Hippo'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Augustine_of_Hippo'; -- || 579

-- q4_p052
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND creator.gender = 'female' AND liker.gender = 'male'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND creator.gender = 'female' AND liker.gender = 'male'; -- || 53703

-- q4_p053
-- predicates: plp.creationdate >= 1338452282578 AND m.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.creationdate >= 1338452282578 AND m.explicitlydeleted = false; -- || 146203

-- q4_p054
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 52140

-- q4_p055
-- predicates: crp.explicitlydeleted = false AND t.name = 'Muammar_Gaddafi' AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND t.name = 'Muammar_Gaddafi' AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781; -- || 757

-- q4_p056
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.length BETWEEN 5 AND 79 AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.length BETWEEN 5 AND 79 AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781; -- || 21669

-- q4_p057
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND liker.language = 'zh;en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND liker.language = 'zh;en'; -- || 32848

-- q4_p058
-- predicates: crp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79 AND t.name = 'Hamid_Karzai'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79 AND t.name = 'Hamid_Karzai'; -- || 110

-- q4_p059
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND c.length BETWEEN 5 AND 79 AND plp.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND c.length BETWEEN 5 AND 79 AND plp.explicitlydeleted = false; -- || 45605

-- q4_p060
-- predicates: creator.gender = 'male' AND m.creationdate >= 1328021100728
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE creator.gender = 'male' AND m.creationdate >= 1328021100728; -- || 82280

-- q4_p061
-- predicates: crp.explicitlydeleted = false AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'; -- || 5201

-- q4_p062
-- predicates: php.explicitlydeleted = false AND t.name = 'Hamid_Karzai' AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND t.name = 'Hamid_Karzai' AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781; -- || 75

-- q4_p063
-- predicates: crp.explicitlydeleted = false AND t.name = 'Hamid_Karzai' AND plp.creationdate >= 1338452282578
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND t.name = 'Hamid_Karzai' AND plp.creationdate >= 1338452282578; -- || 122

-- q4_p064
-- predicates: php.explicitlydeleted = false AND c.length <= 4 AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND c.length <= 4 AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082; -- || 76520

-- q4_p065
-- predicates: crp.explicitlydeleted = false AND liker.gender = 'male'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND liker.gender = 'male'; -- || 139831

-- q4_p066
-- predicates: crp.explicitlydeleted = false AND t.name = 'Muammar_Gaddafi' AND liker.language = 'pt;en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND t.name = 'Muammar_Gaddafi' AND liker.language = 'pt;en'; -- || 35

-- q4_p067
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Mariano_Rivera'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Mariano_Rivera'; -- || 1170

-- q4_p068
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Muammar_Gaddafi'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND t.name = 'Muammar_Gaddafi'; -- || 757

-- q4_p069
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Augustine_of_Hippo'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Augustine_of_Hippo'; -- || 893

-- q4_p070
-- predicates: php.explicitlydeleted = false AND m.explicitlydeleted = false AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND m.explicitlydeleted = false AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781; -- || 126302

-- q4_p071
-- predicates: crp.explicitlydeleted = false AND c.length >= 79
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND c.length >= 79; -- || 76065

-- q4_p072
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND m.length BETWEEN 0 AND 97 AND liker.gender = 'female'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND m.length BETWEEN 0 AND 97 AND liker.gender = 'female'; -- || 27214

-- q4_p073
-- predicates: plp.explicitlydeleted = false AND creator.gender = 'male' AND t.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.explicitlydeleted = false AND creator.gender = 'male' AND t.name = 'Wolfgang_Amadeus_Mozart'; -- || 94

-- q4_p074
-- predicates: php.explicitlydeleted = false AND liker.language = 'es;en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND liker.language = 'es;en'; -- || 19038

-- q4_p075
-- predicates: php.explicitlydeleted = false AND m.length > 0 AND t.name = 'Hamid_Karzai'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND m.length > 0 AND t.name = 'Hamid_Karzai'; -- || 292

-- q4_p076
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Muammar_Gaddafi' AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Muammar_Gaddafi' AND c.creationdate >= 1350093427787; -- || 106

-- q4_p077
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND m.language = 'en' AND crp.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND m.language = 'en' AND crp.explicitlydeleted = false; -- || 125279

-- q4_p078
-- predicates: plp.explicitlydeleted = false AND liker.language = 'en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.explicitlydeleted = false AND liker.language = 'en'; -- || 22938

-- q4_p079
-- predicates: plp.creationdate >= 1338452282578 AND t.name = 'Adolf_Hitler' AND php.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.creationdate >= 1338452282578 AND t.name = 'Adolf_Hitler' AND php.explicitlydeleted = false; -- || 435

-- q4_p080
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND m.creationdate >= 1328021100728 AND liker.gender = 'female'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND m.creationdate >= 1328021100728 AND liker.gender = 'female'; -- || 18587

-- q4_p081
-- predicates: crp.explicitlydeleted = false AND creator.gender = 'female'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND creator.gender = 'female'; -- || 156095

-- q4_p082
-- predicates: plp.creationdate >= 1338452282578 AND c.length <= 4 AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.creationdate >= 1338452282578 AND c.length <= 4 AND pht.creationdate BETWEEN 1301148754469 AND 1339192828781; -- || 2051

-- q4_p083
-- predicates: plp.explicitlydeleted = false AND t.name = 'Hamid_Karzai'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.explicitlydeleted = false AND t.name = 'Hamid_Karzai'; -- || 281

-- q4_p084
-- predicates: crp.explicitlydeleted = false AND t.name = 'Adolf_Hitler'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND t.name = 'Adolf_Hitler'; -- || 1571

-- q4_p085
-- predicates: crp.explicitlydeleted = false AND c.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND c.explicitlydeleted = false; -- || 287945

-- q4_p086
-- predicates: plp.explicitlydeleted = false AND m.creationdate >= 1328021100728
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.explicitlydeleted = false AND m.creationdate >= 1328021100728; -- || 174199

-- q4_p087
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND m.creationdate BETWEEN 1308052370341 AND 1343818164989
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND m.creationdate BETWEEN 1308052370341 AND 1343818164989; -- || 113122

-- q4_p088
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND m.length > 0 AND t.name = 'Genghis_Khan'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND m.length > 0 AND t.name = 'Genghis_Khan'; -- || 2231

-- q4_p089
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t.name = 'Muammar_Gaddafi' AND plp.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t.name = 'Muammar_Gaddafi' AND plp.explicitlydeleted = false; -- || 414

-- q4_p090
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND creator.gender = 'male'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND creator.gender = 'male'; -- || 91683

-- q4_p091
-- predicates: plp.explicitlydeleted = false AND m.creationdate >= 1328021100728 AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE plp.explicitlydeleted = false AND m.creationdate >= 1328021100728 AND t.name = 'Maximilian_I,_Holy_Roman_Emperor'; -- || 4579

-- q4_p092
-- predicates: php.explicitlydeleted = false AND liker.language = 'en' AND t.name = 'Mariano_Rivera'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND liker.language = 'en' AND t.name = 'Mariano_Rivera'; -- || 78

-- q4_p093
-- predicates: crp.explicitlydeleted = false AND m.explicitlydeleted = false
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND m.explicitlydeleted = false; -- || 287945

-- q4_p094
-- predicates: php.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989; -- || 132328

-- q4_p095
-- predicates: php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Genghis_Khan'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.creationdate BETWEEN 1308052370341 AND 1352177414840 AND t.name = 'Genghis_Khan'; -- || 2231

-- q4_p096
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND creator.gender = 'male'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND creator.gender = 'male'; -- || 60906

-- q4_p097
-- predicates: crp.explicitlydeleted = false AND liker.language = 'zh;en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE crp.explicitlydeleted = false AND liker.language = 'zh;en'; -- || 41903

-- q4_p098
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND m.creationdate BETWEEN 1308052370341 AND 1343818164989 AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND m.creationdate BETWEEN 1308052370341 AND 1343818164989 AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082; -- || 59242

-- q4_p099
-- predicates: php.explicitlydeleted = false AND creator.language = 'pt;en'
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE php.explicitlydeleted = false AND creator.language = 'pt;en'; -- || 17277

-- q4_p100
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND liker.gender = 'male' AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  t.*,
  m.*,
  creator.*,
  liker.*,
  c.*,

  -- 边表
  pht.*,
  php.*,
  plp.*,
  crp.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t
  ON t.id = pht.tagid
JOIN post_hascreator_person AS php
  ON php.postid = m.id
JOIN person AS creator
  ON creator.id = php.personid
JOIN person_likes_post AS plp
  ON plp.postid = m.id
JOIN person AS liker
  ON liker.id = plp.personid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
WHERE pht.creationdate BETWEEN 1301148754469 AND 1339192828781 AND liker.gender = 'male' AND c.creationdate >= 1350093427787; -- || 15126

