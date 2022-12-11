-- create in mysql
DROP TABLE peg_solitaire_puzzles;
CREATE TABLE `peg_solitaire_puzzles` (
  `hash` varchar(100) NOT NULL,
  `value` int DEFAULT NULL,
  `holes` int NOT NULL,
  `position` varchar(100) NOT NULL,
  PRIMARY KEY (`hash`,`holes`)
  );


-- initial load
insert into peg_solitaire_puzzles(hash, value, holes, position)
select hash, value, holes, position from peg_solitaire_values_deep_tree_traversal where holes > 23 and holes < 33 and value = 41 limit 100;