-- Your SQL goes here

CREATE TABLE IF NOT EXISTS `Show`
(
    show_id     INT UNSIGNED NOT NULL UNIQUE AUTO_INCREMENT,
    title       VARCHAR(256) NOT NULL,
    description TEXT(5000),
    creation    DATETIME     NOT NULL DEFAULT NOW(),
    PRIMARY KEY (show_id)
);

CREATE TABLE IF NOT EXISTS `Scene`
(
    scene_id    INT UNSIGNED NOT NULL UNIQUE AUTO_INCREMENT,
    show_id     INT UNSIGNED NOT NULL,
    number      INT          NOT NULL,
    title       VARCHAR(256) NOT NULL,
    description TEXT(1000),
    PRIMARY KEY (scene_id),
    UNIQUE (show_id, number),
    FOREIGN KEY (show_id) REFERENCES `Show` (show_id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `Character`
(
    character_id INT UNSIGNED NOT NULL UNIQUE AUTO_INCREMENT,
    show_id      INT UNSIGNED NOT NULL,
    name         VARCHAR(256) NOT NULL,
    description  TEXT(1000),
    PRIMARY KEY (character_id),
    FOREIGN KEY (show_id) REFERENCES `Show` (show_id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `Prop`
(
    prop_id     INT UNSIGNED NOT NULL UNIQUE AUTO_INCREMENT,
    show_id     INT UNSIGNED NOT NULL,
    name        VARCHAR(256) NOT NULL,
    description TEXT(500),
    PRIMARY KEY (prop_id),
    FOREIGN KEY (show_id) REFERENCES `Show` (show_id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `Sound`
(
    sound_id        INT UNSIGNED NOT NULL UNIQUE AUTO_INCREMENT,
    show_id         INT UNSIGNED NOT NULL,
    name            VARCHAR(256) NOT NULL,
    filename        VARCHAR(256),
    PRIMARY KEY (sound_id),
    FOREIGN KEY (show_id) REFERENCES `Show` (show_id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `Track`
(
    track_id    INT UNSIGNED NOT NULL UNIQUE AUTO_INCREMENT,
    show_id     INT UNSIGNED NOT NULL,
    title       VARCHAR(256) NOT NULL,
    live        BOOL         NOT NULL,
    filename    VARCHAR(256),
    PRIMARY KEY (track_id),
    FOREIGN KEY (show_id) REFERENCES `Show` (show_id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);