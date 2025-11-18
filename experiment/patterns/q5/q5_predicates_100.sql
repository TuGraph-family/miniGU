-- Q5 base true_cardinality: 492452
-- Generated with seed=20260115

-- q5_p001
-- predicates: crp.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989 AND t2.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989 AND t2.name = 'Wolfgang_Amadeus_Mozart'; -- || 449

-- q5_p002
-- predicates: crp.explicitlydeleted = false AND t1.name = 'Augustine_of_Hippo' AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND t1.name = 'Augustine_of_Hippo' AND c.length BETWEEN 5 AND 79; -- || 758

-- q5_p003
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.creationdate >= 1350093427787 AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.creationdate >= 1350093427787 AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082; -- || 73232

-- q5_p004
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t2.name = 'Augustine_of_Hippo' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t2.name = 'Augustine_of_Hippo' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 339

-- q5_p005
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019; -- || 176129

-- q5_p006
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.explicitlydeleted = false AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.explicitlydeleted = false AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019; -- || 217542

-- q5_p007
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Augustine_of_Hippo'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Augustine_of_Hippo'; -- || 1566

-- q5_p008
-- predicates: crp.explicitlydeleted = false AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND c.creationdate >= 1350093427787; -- || 145626

-- q5_p009
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.explicitlydeleted = false
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.explicitlydeleted = false; -- || 266672

-- q5_p010
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.length > 0 AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.length > 0 AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019; -- || 190341

-- q5_p011
-- predicates: crp.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND m.creationdate BETWEEN 1308052370341 AND 1343818164989; -- || 252738

-- q5_p012
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t2.name = 'Muammar_Gaddafi'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t2.name = 'Muammar_Gaddafi'; -- || 983

-- q5_p013
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.language = 'en'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.language = 'en'; -- || 151409

-- q5_p014
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t2.name = 'Genghis_Khan'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t2.name = 'Genghis_Khan'; -- || 532

-- q5_p015
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length >= 79 AND t2.name = 'Genghis_Khan'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length >= 79 AND t2.name = 'Genghis_Khan'; -- || 460

-- q5_p016
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Hamid_Karzai'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Hamid_Karzai'; -- || 643

-- q5_p017
-- predicates: crp.explicitlydeleted = false AND t2.name = 'Khalid_Sheikh_Mohammed'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND t2.name = 'Khalid_Sheikh_Mohammed'; -- || 1201

-- q5_p018
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t2.name = 'Genghis_Khan'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629 AND t2.name = 'Genghis_Khan'; -- || 364

-- q5_p019
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Muammar_Gaddafi' AND t2.name = 'Genghis_Khan'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Muammar_Gaddafi' AND t2.name = 'Genghis_Khan'; -- || 1

-- q5_p020
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate >= 1350093427787; -- || 122236

-- q5_p021
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Genghis_Khan'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Genghis_Khan'; -- || 1426

-- q5_p022
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t2.name = 'Augustine_of_Hippo' AND t1.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t2.name = 'Augustine_of_Hippo' AND t1.name = 'Wolfgang_Amadeus_Mozart'; -- || 3

-- q5_p023
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Muammar_Gaddafi'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Muammar_Gaddafi'; -- || 2306

-- q5_p024
-- predicates: crp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79; -- || 139645

-- q5_p025
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND m.language = 'en' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND m.language = 'en' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 97603

-- q5_p026
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate >= 1350093427787 AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate >= 1350093427787 AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082; -- || 120287

-- q5_p027
-- predicates: crp.explicitlydeleted = false AND t1.name = 'Muammar_Gaddafi' AND t2.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND t1.name = 'Muammar_Gaddafi' AND t2.name = 'Wolfgang_Amadeus_Mozart'; -- || 2

-- q5_p028
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.language = 'en' AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.language = 'en' AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019; -- || 97603

-- q5_p029
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.creationdate >= 1328021100728
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.creationdate >= 1328021100728; -- || 124771

-- q5_p030
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Genghis_Khan' AND t1.name = 'Adolf_Hitler'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Genghis_Khan' AND t1.name = 'Adolf_Hitler'; -- || 4

-- q5_p031
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t2.name = 'Georg_Wilhelm_Friedrich_Hegel'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t2.name = 'Georg_Wilhelm_Friedrich_Hegel'; -- || 152

-- q5_p032
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t1.name = 'Muammar_Gaddafi' AND t2.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t1.name = 'Muammar_Gaddafi' AND t2.name = 'Wolfgang_Amadeus_Mozart'; -- || 1

-- q5_p033
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.length >= 79
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.length >= 79; -- || 252724

-- q5_p034
-- predicates: c.length BETWEEN 5 AND 79 AND m.explicitlydeleted = false
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE c.length BETWEEN 5 AND 79 AND m.explicitlydeleted = false; -- || 141887

-- q5_p035
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Genghis_Khan'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Genghis_Khan'; -- || 594

-- q5_p036
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.length BETWEEN 5 AND 79; -- || 96324

-- q5_p037
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t2.name = 'Adolf_Hitler' AND t1.name = 'Augustine_of_Hippo'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t2.name = 'Adolf_Hitler' AND t1.name = 'Augustine_of_Hippo'; -- || 2

-- q5_p038
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length BETWEEN 5 AND 79; -- || 78935

-- q5_p039
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t2.name = 'Sonia_Gandhi'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t2.name = 'Sonia_Gandhi'; -- || 580

-- q5_p040
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 179695

-- q5_p041
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Augustine_of_Hippo'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Augustine_of_Hippo'; -- || 1802

-- q5_p042
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.explicitlydeleted = false AND m.language = 'en'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.explicitlydeleted = false AND m.language = 'en'; -- || 164353

-- q5_p043
-- predicates: crp.explicitlydeleted = false AND m.creationdate >= 1328021100728 AND t2.name = 'Georg_Wilhelm_Friedrich_Hegel'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND m.creationdate >= 1328021100728 AND t2.name = 'Georg_Wilhelm_Friedrich_Hegel'; -- || 95

-- q5_p044
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Adolf_Hitler'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Adolf_Hitler'; -- || 1780

-- q5_p045
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND m.language = 'en'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND m.language = 'en'; -- || 167259

-- q5_p046
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 253938

-- q5_p047
-- predicates: crp.explicitlydeleted = false AND m.explicitlydeleted = false
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND m.explicitlydeleted = false; -- || 484583

-- q5_p048
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.explicitlydeleted = false
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.explicitlydeleted = false; -- || 327228

-- q5_p049
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Mariano_Rivera'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Mariano_Rivera'; -- || 1056

-- q5_p050
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.explicitlydeleted = false AND t1.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.explicitlydeleted = false AND t1.name = 'Wolfgang_Amadeus_Mozart'; -- || 1201

-- q5_p051
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.explicitlydeleted = false AND t1.name = 'Muammar_Gaddafi'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.explicitlydeleted = false AND t1.name = 'Muammar_Gaddafi'; -- || 2249

-- q5_p052
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t1.name = 'Adolf_Hitler' AND t2.name = 'Genghis_Khan'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t1.name = 'Adolf_Hitler' AND t2.name = 'Genghis_Khan'; -- || 4

-- q5_p053
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND m.creationdate >= 1328021100728 AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND m.creationdate >= 1328021100728 AND c.length BETWEEN 5 AND 79; -- || 41744

-- q5_p054
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Hamid_Karzai'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Hamid_Karzai'; -- || 730

-- q5_p055
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Augustine_of_Hippo' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Augustine_of_Hippo' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 1023

-- q5_p056
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Wolfgang_Amadeus_Mozart'; -- || 1499

-- q5_p057
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.creationdate >= 1328021100728 AND t2.name = 'Khalid_Sheikh_Mohammed'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.creationdate >= 1328021100728 AND t2.name = 'Khalid_Sheikh_Mohammed'; -- || 1143

-- q5_p058
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.length > 0
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.length > 0; -- || 271260

-- q5_p059
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND m.length > 0
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND m.length > 0; -- || 332263

-- q5_p060
-- predicates: crp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79 AND t1.name = 'Hamid_Karzai'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND c.length BETWEEN 5 AND 79 AND t1.name = 'Hamid_Karzai'; -- || 236

-- q5_p061
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t2.name = 'Khalid_Sheikh_Mohammed' AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t2.name = 'Khalid_Sheikh_Mohammed' AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019; -- || 1135

-- q5_p062
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Mariano_Rivera'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Mariano_Rivera'; -- || 2290

-- q5_p063
-- predicates: crp.explicitlydeleted = false AND t2.name = 'Georg_Wilhelm_Friedrich_Hegel' AND t1.name = 'Mariano_Rivera'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND t2.name = 'Georg_Wilhelm_Friedrich_Hegel' AND t1.name = 'Mariano_Rivera'; -- || 2

-- q5_p064
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND c.creationdate >= 1350093427787; -- || 90773

-- q5_p065
-- predicates: t2.name = 'Augustine_of_Hippo' AND c.explicitlydeleted = false
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE t2.name = 'Augustine_of_Hippo' AND c.explicitlydeleted = false; -- || 963

-- q5_p066
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.length BETWEEN 0 AND 97
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.length BETWEEN 0 AND 97; -- || 93728

-- q5_p067
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.length > 0
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.length > 0; -- || 303930

-- q5_p068
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t2.name = 'Muammar_Gaddafi' AND t1.name = 'Hamid_Karzai'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t2.name = 'Muammar_Gaddafi' AND t1.name = 'Hamid_Karzai'; -- || 1

-- q5_p069
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length BETWEEN 5 AND 79 AND t2.name = 'Khalid_Sheikh_Mohammed'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length BETWEEN 5 AND 79 AND t2.name = 'Khalid_Sheikh_Mohammed'; -- || 331

-- q5_p070
-- predicates: crp.explicitlydeleted = false AND m.language = 'en'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND m.language = 'en'; -- || 237352

-- q5_p071
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t2.name = 'Khalid_Sheikh_Mohammed'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t2.name = 'Khalid_Sheikh_Mohammed'; -- || 1211

-- q5_p072
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length BETWEEN 5 AND 79 AND t2.name = 'Adolf_Hitler'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length BETWEEN 5 AND 79 AND t2.name = 'Adolf_Hitler'; -- || 208

-- q5_p073
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Wolfgang_Amadeus_Mozart'; -- || 687

-- q5_p074
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Adolf_Hitler' AND t2.name = 'Genghis_Khan'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t1.name = 'Adolf_Hitler' AND t2.name = 'Genghis_Khan'; -- || 4

-- q5_p075
-- predicates: crp.explicitlydeleted = false AND t2.name = 'Augustine_of_Hippo' AND t1.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND t2.name = 'Augustine_of_Hippo' AND t1.name = 'Wolfgang_Amadeus_Mozart'; -- || 4

-- q5_p076
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.length BETWEEN 0 AND 97 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.length BETWEEN 0 AND 97 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 79455

-- q5_p077
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t1.name = 'Hamid_Karzai'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t1.name = 'Hamid_Karzai'; -- || 692

-- q5_p078
-- predicates: pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Adolf_Hitler' AND t2.name = 'Muammar_Gaddafi'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE pht.creationdate BETWEEN 1301148754469 AND 1349895389019 AND t1.name = 'Adolf_Hitler' AND t2.name = 'Muammar_Gaddafi'; -- || 1

-- q5_p079
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.explicitlydeleted = false
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.explicitlydeleted = false; -- || 298932

-- q5_p080
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.length BETWEEN 0 AND 97
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.length BETWEEN 0 AND 97; -- || 84174

-- q5_p081
-- predicates: crp.explicitlydeleted = false AND m.length > 0
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND m.length > 0; -- || 484620

-- q5_p082
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.creationdate >= 1328021100728 AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.creationdate >= 1328021100728 AND pht.creationdate BETWEEN 1301148754469 AND 1349895389019; -- || 103657

-- q5_p083
-- predicates: crp.explicitlydeleted = false AND m.creationdate >= 1328021100728 AND c.length >= 79
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND m.creationdate >= 1328021100728 AND c.length >= 79; -- || 127259

-- q5_p084
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.creationdate >= 1350093427787
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.creationdate >= 1350093427787; -- || 120287

-- q5_p085
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.language = 'en' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND m.language = 'en' AND cht.creationdate BETWEEN 1338765911761 AND 1356307644022; -- || 136028

-- q5_p086
-- predicates: crp.explicitlydeleted = false AND t2.name = 'Adolf_Hitler'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.explicitlydeleted = false AND t2.name = 'Adolf_Hitler'; -- || 1207

-- q5_p087
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Sonia_Gandhi'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Sonia_Gandhi'; -- || 529

-- q5_p088
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.length BETWEEN 5 AND 79
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.length BETWEEN 5 AND 79; -- || 88429

-- q5_p089
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Khalid_Sheikh_Mohammed'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Khalid_Sheikh_Mohammed'; -- || 1143

-- q5_p090
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.explicitlydeleted = false AND t2.name = 'Wolfgang_Amadeus_Mozart'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.explicitlydeleted = false AND t2.name = 'Wolfgang_Amadeus_Mozart'; -- || 687

-- q5_p091
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND c.creationdate BETWEEN 1340327339000 AND 1356461058629; -- || 251989

-- q5_p092
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Augustine_of_Hippo'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Augustine_of_Hippo'; -- || 573

-- q5_p093
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.language = 'en' AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.language = 'en' AND crp.creationdate BETWEEN 1335749795059 AND 1356234923082; -- || 136028

-- q5_p094
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t1.name = 'Adolf_Hitler'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t1.name = 'Adolf_Hitler'; -- || 1990

-- q5_p095
-- predicates: c.explicitlydeleted = false AND m.length BETWEEN 0 AND 97 AND t1.name = 'Hamid_Karzai'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE c.explicitlydeleted = false AND m.length BETWEEN 0 AND 97 AND t1.name = 'Hamid_Karzai'; -- || 360

-- q5_p096
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length >= 79
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND c.length >= 79; -- || 206294

-- q5_p097
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.creationdate BETWEEN 1308052370341 AND 1343818164989
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND m.creationdate BETWEEN 1308052370341 AND 1343818164989; -- || 131761

-- q5_p098
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t2.name = 'Genghis_Khan'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t2.name = 'Genghis_Khan'; -- || 625

-- q5_p099
-- predicates: cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Wolfgang_Amadeus_Mozart' AND t1.name = 'Muammar_Gaddafi'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE cht.creationdate BETWEEN 1338765911761 AND 1356307644022 AND t2.name = 'Wolfgang_Amadeus_Mozart' AND t1.name = 'Muammar_Gaddafi'; -- || 1

-- q5_p100
-- predicates: crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t1.name = 'Genghis_Khan'
SELECT
  -- 点表
  m.*,
  c.*,
  t1.*,
  t2.*,

  -- 边表
  pht.*,
  crp.*,
  cht.*
FROM post AS m
JOIN post_hastag_tag AS pht
  ON pht.postid = m.id
JOIN tag AS t1
  ON t1.id = pht.tagid
JOIN comment_replyof_post AS crp
  ON crp.postid = m.id
JOIN comment AS c
  ON c.id = crp.commentid
JOIN comment_hastag_tag AS cht
  ON cht.commentid = c.id
JOIN tag AS t2
  ON t2.id = cht.tagid
 AND t2.id <> t1.id
WHERE crp.creationdate BETWEEN 1335749795059 AND 1356234923082 AND t1.name = 'Genghis_Khan'; -- || 1520

