-- create in mysql
DROP TABLE peg_solitaire_values;
CREATE TABLE if not exists `peg_solitaire_values` (
          `hash` varchar(100) NOT NULL,
          `value` int DEFAULT NULL,
          `holes` int NOT NULL,
          `position` varchar(100) NOT NULL,
          PRIMARY KEY (`hash`,`holes`)
          )
        PARTITION BY LIST(holes) (
            PARTITION pHoles_1 VALUES IN (1,2,3,4,5,6,7,8,9,10,11,12,13),
            PARTITION pHoles_2 VALUES IN (14,15),
            PARTITION pHoles_3 VALUES IN(16),
            PARTITION pHoles_4 VALUES IN(17),
            PARTITION pHoles_5 VALUES IN(18),
            PARTITION pHoles_6 VALUES IN(19),
            PARTITION pHoles_7 VALUES IN(20),
            PARTITION pHoles_8 VALUES IN(21),
            PARTITION pHoles_9 VALUES IN(22),
            PARTITION pHoles_10 VALUES IN(23),
            PARTITION pHoles_11 VALUES IN(24),
            PARTITION pHoles_12 VALUES IN(25),
            PARTITION pHoles_13 VALUES IN(26, 27, 28, 29, 30, 31, 32)
        );