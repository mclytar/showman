-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `User`
(
    user_id INT UNSIGNED NOT NULL UNIQUE AUTO_INCREMENT,
    name    VARCHAR(64)  NOT NULL,
    surname VARCHAR(64)  NOT NULL,
    role    VARCHAR(64)  NOT NULL DEFAULT 'pending',
    PRIMARY KEY (user_id)
);

CREATE TABLE IF NOT EXISTS `Authentication`
(
    auth_id   INT UNSIGNED NOT NULL UNIQUE AUTO_INCREMENT,
    user_id   INT UNSIGNED NOT NULL,
    method    VARCHAR(64)  NOT NULL,
    user_data VARCHAR(255) NOT NULL,
    token     TEXT(1024)   NOT NULL,
    PRIMARY KEY (auth_id),
    UNIQUE KEY `AuthenticationUniqueness_key` (`method`, `user_data`),
    FOREIGN KEY (user_id) REFERENCES `User` (user_id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `Session`
(
    token_id     INT UNSIGNED NOT NULL UNIQUE AUTO_INCREMENT,
    user_id      INT UNSIGNED NOT NULL,
    needs_update BOOLEAN      NOT NULL DEFAULT FALSE,
    expiration   DATETIME     NOT NULL,
    PRIMARY KEY (token_id),
    FOREIGN KEY (user_id) REFERENCES `User` (user_id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TRIGGER `SessionNeedsUpdate_User_AfterUpdate_trig`
    AFTER UPDATE ON `User` FOR EACH ROW
        UPDATE `Session` SET `needs_update` = TRUE WHERE `user_id` = NEW.user_id;

/*
CREATE TABLE IF NOT EXISTS `Show` (
    show_id             INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    title               VARCHAR(255),
    description         TEXT(4096),
    due_date            DATETIME,
    PRIMARY KEY (show_id)
);

CREATE TABLE IF NOT EXISTS StaffMember (
    user_id            INT UNSIGNED NOT NULL,
    show_id            INT UNSIGNED NOT NULL,
    role               VARCHAR(255) NOT NULL DEFAULT 'observer',
    PRIMARY KEY (user_id, show_id),
    FOREIGN KEY (user_id) REFERENCES User(user_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (show_id) REFERENCES `Show`(show_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS Scene (
    scene_id            INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    title               VARCHAR(255),
    description         TEXT(4096),
    show_id             INT UNSIGNED NOT NULL,
    PRIMARY KEY (scene_id),
    FOREIGN KEY (show_id) REFERENCES `Show`(show_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS Version (
    version_id          INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    number              INT UNSIGNED NOT NULL,
    scene_id            INT UNSIGNED NOT NULL,
    PRIMARY KEY (version_id),
    FOREIGN KEY (scene_id) REFERENCES Scene(scene_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- ----------------------------------------------------------------
-- Scene Entities
-- ----------------------------------------------------------------

CREATE TABLE IF NOT EXISTS SE_Character (
    character_id        INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    name                VARCHAR(255) NOT NULL,
    display             VARCHAR(255) NOT NULL,
    description         TEXT(4096),
    user_id             INT UNSIGNED DEFAULT NULL,
    scene_id            INT UNSIGNED NOT NULL,
    PRIMARY KEY (character_id),
    FOREIGN KEY (user_id) REFERENCES User(user_id)
        ON DELETE SET NULL
        ON UPDATE CASCADE,
    FOREIGN KEY (scene_id) REFERENCES Scene(scene_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS SE_Object (
    object_id           INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    name                VARCHAR(255) NOT NULL,
    display             VARCHAR(255) NOT NULL,
    description         TEXT(4096),
    scene_id            INT UNSIGNED NOT NULL,
    PRIMARY KEY (object_id)
);

CREATE TABLE IF NOT EXISTS SE_Audio (
    audio_id            INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    name                VARCHAR(255),
    filename            VARCHAR(255),
    scene_id            INT UNSIGNED NOT NULL,
    PRIMARY KEY (audio_id),
    FOREIGN KEY (scene_id) REFERENCES Scene(scene_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS SE_Music (
    music_id            INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    name                VARCHAR(255),
    scene_id            INT UNSIGNED NOT NULL,
    PRIMARY KEY (music_id),
    FOREIGN KEY (scene_id) REFERENCES Scene(scene_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS SE_MusicPlayer (
    player_id           INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    music_id            INT UNSIGNED NOT NULL,
    user_id             INT UNSIGNED DEFAULT NULL,
    role                VARCHAR(255),
    PRIMARY KEY (player_id),
    FOREIGN KEY (music_id) REFERENCES SE_Music(music_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (user_id) REFERENCES User(user_id)
        ON DELETE SET NULL
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS SE_MusicLyrics (
    lyrics_id           INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    music_id            INT UNSIGNED NOT NULL,
    PRIMARY KEY (lyrics_id),
    FOREIGN KEY (music_id) REFERENCES SE_Music(music_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS SE_MusicLyricsLine (
    lyrics_line_id      INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    lyrics_id           INT UNSIGNED NOT NULL,
    contents            VARCHAR(255),
    PRIMARY KEY (lyrics_line_id),
    FOREIGN KEY (lyrics_id) REFERENCES SE_MusicLyrics(lyrics_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- ----------------------------------------------------------------
-- Scene Flow Rows
-- ----------------------------------------------------------------

CREATE TABLE IF NOT EXISTS SFI_Row (
    row_id              INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    version_id          INT UNSIGNED NOT NULL,
    PRIMARY KEY (row_id),
    FOREIGN KEY (version_id) REFERENCES Version(version_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS SFI_Speech (
    row_id              INT UNSIGNED UNIQUE NOT NULL,
    character_id        INT UNSIGNED NOT NULL,
    text                TEXT(4096),
    PRIMARY KEY (row_id),
    FOREIGN KEY (row_id) REFERENCES SFI_Row(row_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (character_id) REFERENCES SE_Character(character_id)
        ON DELETE RESTRICT
        ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS SFI_Audio (
    row_id              INT UNSIGNED UNIQUE NOT NULL,
    audio_id            INT UNSIGNED NOT NULL,
    PRIMARY KEY (row_id),
    FOREIGN KEY (row_id) REFERENCES SFI_Row(row_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (audio_id) REFERENCES SE_Audio(audio_id)
        ON DELETE RESTRICT
        ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS SFI_Music (
    row_id              INT UNSIGNED UNIQUE NOT NULL,
    music_id            INT UNSIGNED NOT NULL,
    PRIMARY KEY (row_id),
    FOREIGN KEY (row_id) REFERENCES SFI_Row(row_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (music_id) REFERENCES SE_Music(music_id)
        ON DELETE RESTRICT
        ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS SFI_Static (
    row_id              INT UNSIGNED UNIQUE NOT NULL,
    contents            TEXT(4096),
    PRIMARY KEY (row_id),
    FOREIGN KEY (row_id) REFERENCES SFI_Row(row_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS SFI_ObjectMove (
    row_id              INT UNSIGNED UNIQUE NOT NULL,
    object_id           INT UNSIGNED NOT NULL,
    kind                VARCHAR(64),
    PRIMARY KEY (row_id),
    FOREIGN KEY (row_id) REFERENCES SFI_Row(row_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (object_id) REFERENCES SE_Object(object_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS SFI_CharacterMove (
    row_id              INT UNSIGNED UNIQUE NOT NULL,
    character_id        INT UNSIGNED NOT NULL,
    kind                VARCHAR(64),
    PRIMARY KEY (row_id),
    FOREIGN KEY (row_id) REFERENCES SFI_Row(row_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (character_id) REFERENCES SE_Character(character_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS SFI_Comment (
    comment_id          INT UNSIGNED UNIQUE NOT NULL AUTO_INCREMENT,
    row_id              INT UNSIGNED NOT NULL,
    user_id             INT UNSIGNED DEFAULT NULL,
    timestamp           DATETIME DEFAULT NOW(),
    contents            TEXT(4096),
    PRIMARY KEY (comment_id),
    FOREIGN KEY (row_id) REFERENCES SFI_Row(row_id)
        ON DELETE RESTRICT
        ON UPDATE RESTRICT ,
    FOREIGN KEY (user_id) REFERENCES User(user_id)
        ON DELETE SET NULL
        ON UPDATE CASCADE
);

*/