-- This file should undo anything in `up.sql`
DROP TRIGGER `SessionNeedsUpdate_User_AfterUpdate_trig`;
DROP TABLE `Session`;
DROP TABLE `Authentication`;
DROP TABLE `User`;