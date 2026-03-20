-- 1. 创建众筹状态枚举类型
CREATE TYPE campaign_status AS ENUM ('Pending', 'Active', 'Funded', 'Failed', 'Cancelled');

-- 2. 创建众筹项目表
CREATE TABLE campaigns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    creator_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    goal_amount BIGINT NOT NULL,           -- 单位：分
    current_amount BIGINT NOT NULL DEFAULT 0, -- 单位：分
    status campaign_status NOT NULL DEFAULT 'Pending',
    start_at TIMESTAMPTZ NOT NULL,
    end_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. 为状态和创建者添加索引，优化查询速度
CREATE INDEX idx_campaigns_status ON campaigns(status);
CREATE INDEX idx_campaigns_creator_id ON campaigns(creator_id);

-- 4. 触发器：自动更新 updated_at
CREATE TRIGGER update_campaigns_updated_at
    BEFORE UPDATE ON campaigns
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
