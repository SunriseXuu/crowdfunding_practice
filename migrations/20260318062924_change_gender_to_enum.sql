-- 1. 创建 Postgres 本地的枚举类型 (GenderEnum)
CREATE TYPE gender_enum AS ENUM ('M', 'F', 'O');

-- 2. 将原有的 gender (VARCHAR) 转换为刚才创建的 gender_enum 类型
ALTER TABLE users 
    ALTER COLUMN gender TYPE gender_enum
    USING CASE 
        WHEN gender = 'M' OR gender = 'MALE' OR gender = '男' OR gender = '1'
            THEN 'M'::gender_enum
        WHEN gender = 'F' OR gender = 'FEMALE' OR gender = '女' OR gender = '0'
            THEN 'F'::gender_enum
        ELSE 'O'::gender_enum 
    END;
