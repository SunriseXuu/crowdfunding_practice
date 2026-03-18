-- Add age columns to users table
ALTER TABLE users
ADD COLUMN age INT;

-- Add gender columns to users table
ALTER TABLE users
ADD COLUMN gender VARCHAR(10);
