-- 1. 创建角色枚举类型
CREATE TYPE user_role AS ENUM ('user', 'admin');

-- 2. 添加角色列
ALTER TABLE users ADD COLUMN role user_role NOT NULL DEFAULT 'user';

-- 3. 添加 is_deactivated 和 is_banned 列
ALTER TABLE users ADD COLUMN is_deactivated BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE users ADD COLUMN is_banned BOOLEAN NOT NULL DEFAULT FALSE;

-- 4. 迁移 existing is_active 逻辑 (假设 false 表示已自行注销)
UPDATE users SET is_deactivated = TRUE WHERE is_active = FALSE;

-- 5. 删除 is_active 列
ALTER TABLE users DROP COLUMN is_active;
