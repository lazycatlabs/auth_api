-- Your SQL goes here
CREATE TABLE login_history
(
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    login_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ip_addr VARCHAR NOT NULL,
    device_info VARCHAR NOT NULL,
    os_info VARCHAR NOT NULL
);
