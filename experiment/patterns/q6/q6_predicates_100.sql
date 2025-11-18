-- Q6 base true_cardinality: 55607896
-- Generated with seed=20260115

-- q6_p001
-- predicates: p3.language = 'pt;en' AND p2.language = 'es;en' AND t.name = 'Plato'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE p3.language = 'pt;en' AND p2.language = 'es;en' AND t.name = 'Plato'; -- || 761

-- q6_p002
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.gender = 'male'; -- || 13706330

-- q6_p003
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p1.language = 'pt;en' AND p2.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p1.language = 'pt;en' AND p2.language = 'zh;en'; -- || 53486

-- q6_p004
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Augustine_of_Hippo' AND p1.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Augustine_of_Hippo' AND p1.language = 'en'; -- || 10915

-- q6_p005
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p1.language = 'es;en' AND t.name = 'Bill_Clinton'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p1.language = 'es;en' AND t.name = 'Bill_Clinton'; -- || 4799

-- q6_p006
-- predicates: phi.creationdate >= 1309845298772 AND t.name = 'Jesus'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND t.name = 'Jesus'; -- || 152102

-- q6_p007
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Napoleon'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Napoleon'; -- || 113882

-- q6_p008
-- predicates: phi.creationdate >= 1309845298772 AND t.name = 'Elizabeth_II' AND p2.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND t.name = 'Elizabeth_II' AND p2.gender = 'male'; -- || 45366

-- q6_p009
-- predicates: p2.language = 'pt;en' AND t.name = 'Augustine_of_Hippo' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE p2.language = 'pt;en' AND t.name = 'Augustine_of_Hippo' AND pk23.dst BETWEEN 252947 AND 253760; -- || 5719

-- q6_p010
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p2.gender = 'female'; -- || 15386006

-- q6_p011
-- predicates: p3.language = 'en' AND p2.gender = 'male' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE p3.language = 'en' AND p2.gender = 'male' AND pk23.dst BETWEEN 252947 AND 253760; -- || 1585053

-- q6_p012
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'en'; -- || 2777679

-- q6_p013
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'pt;en' AND phi.creationdate >= 1309845298772
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'pt;en' AND phi.creationdate >= 1309845298772; -- || 904705

-- q6_p014
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.gender = 'female' AND phi.creationdate >= 1309845298772
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.gender = 'female' AND phi.creationdate >= 1309845298772; -- || 6321758

-- q6_p015
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'zh;en' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'zh;en' AND pk23.dst BETWEEN 252947 AND 253760; -- || 2052886

-- q6_p016
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'zh;en' AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'zh;en' AND p1.language = 'pt;en'; -- || 84105

-- q6_p017
-- predicates: phi.creationdate >= 1309845298772 AND p1.gender = 'female' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p1.gender = 'female' AND pk12.src BETWEEN 252947 AND 253760; -- || 7162901

-- q6_p018
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'pt;en'; -- || 830254

-- q6_p019
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'zh;en' AND t.name = 'John_F._Kennedy'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'zh;en' AND t.name = 'John_F._Kennedy'; -- || 13202

-- q6_p020
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.gender = 'male'; -- || 14213244

-- q6_p021
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Napoleon' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Napoleon' AND pk12.src BETWEEN 252947 AND 253760; -- || 80833

-- q6_p022
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p1.language = 'pt;en' AND p2.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p1.language = 'pt;en' AND p2.language = 'zh;en'; -- || 76244

-- q6_p023
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'zh;en' AND p2.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'zh;en' AND p2.language = 'zh;en'; -- || 1036611

-- q6_p024
-- predicates: phi.creationdate >= 1309845298772 AND p2.language = 'zh;en' AND p1.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p2.language = 'zh;en' AND p1.gender = 'female'; -- || 1386832

-- q6_p025
-- predicates: phi.creationdate >= 1309845298772 AND t.name = 'William_Shakespeare'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND t.name = 'William_Shakespeare'; -- || 148997

-- q6_p026
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Plato'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Plato'; -- || 104133

-- q6_p027
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'es;en' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'es;en' AND pk23.dst BETWEEN 252947 AND 253760; -- || 768973

-- q6_p028
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Bill_Clinton'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Bill_Clinton'; -- || 86280

-- q6_p029
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p3.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p3.gender = 'male'; -- || 13912632

-- q6_p030
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p3.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p3.language = 'es;en'; -- || 1187653

-- q6_p031
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'es;en'; -- || 1524400

-- q6_p032
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'es;en' AND p1.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'es;en' AND p1.language = 'en'; -- || 148071

-- q6_p033
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'zh;en'; -- || 4170734

-- q6_p034
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'en' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'en' AND phi.creationdate BETWEEN 1285156166915 AND 1348051459240; -- || 2097333

-- q6_p035
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p3.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p3.language = 'pt;en'; -- || 1505046

-- q6_p036
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Jesus'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Jesus'; -- || 174719

-- q6_p037
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p3.language = 'es;en' AND p1.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p3.language = 'es;en' AND p1.language = 'es;en'; -- || 250498

-- q6_p038
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p1.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p1.gender = 'male'; -- || 13231262

-- q6_p039
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p1.gender = 'male' AND p3.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p1.gender = 'male' AND p3.language = 'en'; -- || 2063421

-- q6_p040
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'es;en'; -- || 1386107

-- q6_p041
-- predicates: phi.creationdate >= 1309845298772 AND p1.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p1.gender = 'female'; -- || 14371319

-- q6_p042
-- predicates: phi.creationdate >= 1309845298772 AND p2.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p2.language = 'zh;en'; -- || 2739640

-- q6_p043
-- predicates: phi.creationdate >= 1309845298772 AND p2.gender = 'male' AND p1.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p2.gender = 'male' AND p1.language = 'zh;en'; -- || 1709309

-- q6_p044
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p2.language = 'en' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p2.language = 'en' AND pk12.src BETWEEN 252947 AND 253760; -- || 2114585

-- q6_p045
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Jesus' AND p3.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Jesus' AND p3.language = 'en'; -- || 5050

-- q6_p046
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'John_F._Kennedy' AND p1.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'John_F._Kennedy' AND p1.language = 'es;en'; -- || 5560

-- q6_p047
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p2.gender = 'female'; -- || 15654261

-- q6_p048
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'zh;en'; -- || 2553069

-- q6_p049
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND t.name = 'William_Shakespeare'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND t.name = 'William_Shakespeare'; -- || 131407

-- q6_p050
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Napoleon' AND p1.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Napoleon' AND p1.language = 'es;en'; -- || 7043

-- q6_p051
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Plato' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Plato' AND pk12.src BETWEEN 252947 AND 253760; -- || 68235

-- q6_p052
-- predicates: p3.gender = 'female' AND p2.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE p3.gender = 'female' AND p2.language = 'zh;en'; -- || 2504042

-- q6_p053
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p1.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p1.gender = 'male'; -- || 18152775

-- q6_p054
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Bill_Clinton'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Bill_Clinton'; -- || 72044

-- q6_p055
-- predicates: phi.creationdate >= 1309845298772 AND t.name = 'George_W._Bush' AND p3.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND t.name = 'George_W._Bush' AND p3.gender = 'female'; -- || 58729

-- q6_p056
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Bill_Clinton' AND phi.creationdate >= 1309845298772
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Bill_Clinton' AND phi.creationdate >= 1309845298772; -- || 47991

-- q6_p057
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Plato' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Plato' AND pk23.dst BETWEEN 252947 AND 253760; -- || 102387

-- q6_p058
-- predicates: phi.creationdate >= 1309845298772 AND p1.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p1.gender = 'male'; -- || 13840873

-- q6_p059
-- predicates: phi.creationdate >= 1309845298772 AND p1.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p1.language = 'en'; -- || 2500905

-- q6_p060
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Napoleon' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Napoleon' AND pk23.dst BETWEEN 252947 AND 253760; -- || 111469

-- q6_p061
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p3.gender = 'female' AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p3.gender = 'female' AND p1.language = 'pt;en'; -- || 673360

-- q6_p062
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p1.language = 'zh;en' AND t.name = 'Jesus'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p1.language = 'zh;en' AND t.name = 'Jesus'; -- || 31938

-- q6_p063
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Plato'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Plato'; -- || 94017

-- q6_p064
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'zh;en'; -- || 2667732

-- q6_p065
-- predicates: phi.creationdate >= 1309845298772 AND t.name = 'Augustine_of_Hippo' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND t.name = 'Augustine_of_Hippo' AND pk23.dst BETWEEN 252947 AND 253760; -- || 54877

-- q6_p066
-- predicates: phi.creationdate >= 1309845298772 AND t.name = 'John_F._Kennedy'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND t.name = 'John_F._Kennedy'; -- || 102760

-- q6_p067
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.gender = 'female' AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.gender = 'female' AND p2.gender = 'female'; -- || 7832665

-- q6_p068
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Aristotle' AND p3.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Aristotle' AND p3.language = 'zh;en'; -- || 33393

-- q6_p069
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.gender = 'female' AND p2.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.gender = 'female' AND p2.gender = 'female'; -- || 7535708

-- q6_p070
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Napoleon' AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND t.name = 'Napoleon' AND p2.language = 'en'; -- || 16532

-- q6_p071
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p3.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p3.language = 'en'; -- || 3320899

-- q6_p072
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'John_F._Kennedy'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'John_F._Kennedy'; -- || 89863

-- q6_p073
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Jesus'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Jesus'; -- || 122765

-- q6_p074
-- predicates: phi.creationdate >= 1309845298772 AND p1.language = 'pt;en' AND t.name = 'Bill_Clinton'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p1.language = 'pt;en' AND t.name = 'Bill_Clinton'; -- || 4203

-- q6_p075
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'en' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.language = 'en' AND pk23.dst BETWEEN 252947 AND 253760; -- || 1304228

-- q6_p076
-- predicates: p1.language = 'en' AND t.name = 'Bill_Clinton' AND p2.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE p1.language = 'en' AND t.name = 'Bill_Clinton' AND p2.language = 'es;en'; -- || 994

-- q6_p077
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p1.language = 'zh;en' AND p2.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p1.language = 'zh;en' AND p2.language = 'pt;en'; -- || 134533

-- q6_p078
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p1.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p1.language = 'pt;en'; -- || 1084666

-- q6_p079
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'pt;en' AND p1.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'pt;en' AND p1.language = 'es;en'; -- || 165702

-- q6_p080
-- predicates: phi.creationdate >= 1309845298772 AND p2.language = 'es;en' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p2.language = 'es;en' AND pk23.dst BETWEEN 252947 AND 253760; -- || 749589

-- q6_p081
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p1.gender = 'female' AND p3.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p1.gender = 'female' AND p3.language = 'pt;en'; -- || 620636

-- q6_p082
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p1.gender = 'female'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p1.gender = 'female'; -- || 13741136

-- q6_p083
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Elizabeth_II'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Elizabeth_II'; -- || 118851

-- q6_p084
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p2.language = 'en'; -- || 3205111

-- q6_p085
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Aristotle' AND phi.creationdate >= 1309845298772
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Aristotle' AND phi.creationdate >= 1309845298772; -- || 40648

-- q6_p086
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'pt;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p2.language = 'pt;en'; -- || 2437766

-- q6_p087
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p1.gender = 'male' AND t.name = 'Plato'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p1.gender = 'male' AND t.name = 'Plato'; -- || 51165

-- q6_p088
-- predicates: phi.creationdate >= 1309845298772 AND p2.language = 'en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p2.language = 'en'; -- || 3212809

-- q6_p089
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND p3.language = 'es;en' AND t.name = 'William_Shakespeare'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND p3.language = 'es;en' AND t.name = 'William_Shakespeare'; -- || 6719

-- q6_p090
-- predicates: phi.creationdate >= 1309845298772 AND p1.language = 'es;en' AND pk23.dst BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p1.language = 'es;en' AND pk23.dst BETWEEN 252947 AND 253760; -- || 835283

-- q6_p091
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p1.gender = 'male' AND p3.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p1.gender = 'male' AND p3.language = 'es;en'; -- || 778804

-- q6_p092
-- predicates: t.name = 'John_F._Kennedy' AND p3.language = 'pt;en' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE t.name = 'John_F._Kennedy' AND p3.language = 'pt;en' AND pk12.src BETWEEN 252947 AND 253760; -- || 8722

-- q6_p093
-- predicates: phi.creationdate >= 1309845298772 AND p2.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND p2.gender = 'male'; -- || 11948317

-- q6_p094
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'zh;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND p3.language = 'zh;en'; -- || 3808806

-- q6_p095
-- predicates: phi.creationdate >= 1309845298772 AND t.name = 'Plato'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND t.name = 'Plato'; -- || 85704

-- q6_p096
-- predicates: pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Napoleon' AND pk12.src BETWEEN 252947 AND 253760
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk23.dst BETWEEN 252947 AND 253760 AND t.name = 'Napoleon' AND pk12.src BETWEEN 252947 AND 253760; -- || 56822

-- q6_p097
-- predicates: phi.creationdate >= 1309845298772 AND t.name = 'George_W._Bush'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate >= 1309845298772 AND t.name = 'George_W._Bush'; -- || 124604

-- q6_p098
-- predicates: p1.language = 'es;en' AND p3.gender = 'male'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE p1.language = 'es;en' AND p3.gender = 'male'; -- || 1791500

-- q6_p099
-- predicates: phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p1.language = 'es;en'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE phi.creationdate BETWEEN 1285156166915 AND 1348051459240 AND p1.language = 'es;en'; -- || 2244764

-- q6_p100
-- predicates: pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Napoleon'
SELECT
  -- 点表
  p1.*,
  p2.*,
  p3.*,
  t.*,

  -- 边表
  pk12.*,
  pk23.*,
  phi.*
FROM person AS p1
JOIN person_knows_person AS pk12
  ON pk12.src = p1.id
JOIN person AS p2
  ON p2.id = pk12.dst
JOIN person_knows_person AS pk23
  ON pk23.src = p2.id
JOIN person AS p3
  ON p3.id = pk23.dst
 AND p3.id <> p1.id
JOIN person_hasinterest_tag AS phi
  ON phi.personid = p3.id
JOIN tag AS t
  ON t.id = phi.tagid
WHERE pk12.src BETWEEN 252947 AND 253760 AND t.name = 'Napoleon'; -- || 116624

