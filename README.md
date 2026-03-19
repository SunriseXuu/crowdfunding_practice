# Crowdfunding Practice (Rust + Axum)

这是一个使用 Rust 语言和 **Axum** 框架构建的企业级现代化后端平台实战项目。

项目采用了企业级的分层架构（分包）设计，确保代码的高内聚、低耦合，方便后期维护与测试。从纯函数抽象、参数拦截器，再到数据库模型的演进，我们已经打造了一个坚实的架构底座。

## 核心技术栈

- **Web 框架**: Axum 0.8
- **异步运行时**: Tokio
- **数据库**: PostgreSQL
- **数据库驱动 & ORM**: sqlx (原生 SQL + 编译期宏校验)
- **参数校验**: validator
- **序列化/反序列化**: serde / serde_json
- **密码哈希**: argon2 (最安全的哈希算法之一)
- **身份鉴权**: jsonwebtoken (Access Token + Refresh Token 双令牌方案)
- **日志监控**: tracing / tower-http (真正的全局中间件 TraceLayer)
- **配置管理**: dotenvy (环境变量映射)

---

## 🏗 架构概览与目录规范

项目采用了严格的经典三层架构（Data Mapper 模式），并进一步对 Web 框架层进行了垂直拆分：

```text
src/
├── main.rs            # 项目入口：组装配置文件、连接数据库、挂载全局 Middleware、启动 Server
├── config/            # 配置层：读取 .env，暴露强类型的全局单例配置 (AppConfig)
├── model/             # 领域实体层：与数据库表结构 1:1 映射的纯“贫血”数据结构体 (不会污染业务逻辑)
├── repository/        # 数据访问层 (Repo)：隔离所有 sqlx 宏查询操作，通过编译期检查保护数据的进出
├── service/           # 业务逻辑层 (胖服务)：最核心的包，处理所有与业务相关的逻辑
├── dto/               # 数据传输对象 (Data Transfer Object)：隔离层。定义外部 Request 与 Response 格式
├── handler/           # 控制器层 (瘦控制器)：解析 HTTP 协议，调用 Extractor 提炼用户参数后，纯转发调度给 Service
├── router/            # 路由网关层：定义各类模块（如 auth, user）的路由表，彻底解耦 Handler 与具体的 URL
├── extractor/         # 提取器层 (Extractor)：包含统一参数验证 (ValidatedJson) 与鉴权拦截器 (AuthUser)
└── error/             # 全局错误处理：实现 IntoResponse，抹平所有内部异常并统一序列化为 JSON 返回
```

---

## 💡 核心设计特性（亮点）

### 1. 极致的路由与中间件分离理念
在使用 Rust 构建大型项目时，我们对 `Extractor（提取器）` 和 `Middleware（中间件）` 进行了清晰的界定：
- **真·中间件 (Middleware)**：使用 `tower_http::TraceLayer`，像一个洋葱圈包裹整个项目，**不对后续业务主动投喂数据**（如处理跨域、日志打印）。
- **细粒度提取器 (Extractor)**：采用结构图实现 `FromRequestParts`，像 `AuthUser`、`ValidatedJson`。它们在 **Handler 的参数列表中显式声明**，通过拦截请求头进行 Token 获取或 JSON 校验，成功后**强类型**地向业务投喂数据。让业务代码绝不出现任何 `if token.is_none()`。
- **孤立的控制器**：路由控制从 Handler 中抽离，组装进 `router/` 模块隔离分发，保证了控制器代码的绝对纯粹，是经典的企业级 Web 服务架构。

### 2. 完整的 HTTP 请求生命周期 (以 User API 为例)

为了清晰呈现架构中每一层的实际运转机制，以下是经过一层层防御与流转的请求生命周期：

```mermaid
sequenceDiagram
    participant Client as 客户端 (Postman/浏览器)
    participant Middleware as 核心中间件 (TraceLayer)
    participant Router as 路由派发 (Router)
    participant Auth as 提取器 (AuthUser)
    participant Json as 参数校验 (ValidatedJson)
    participant Handler as 控制器 (Handler)
    participant Service as 业务层 (Service)
    participant Repo as 访问层 (Repository)
    participant DB as 数据库 (PostgreSQL)

    Client->>Middleware: 1. 发起请求 (带 Bearer Token & JSON Body)
    activate Middleware
    Middleware->>Router: 2. 记录请求开始日志，转交路由
    activate Router
    
    Router->>Auth: 3. 匹配到需登录的路由，触发参数提取
    activate Auth
    Auth-->>Router: 4A. [失败] 返回 401 拒止信号 (伪造/过期)
    Auth->>Router: 4B. [成功] 校验通过，提炼出 user_id 
    deactivate Auth
    
    Router->>Json: 5. 触发表单/参数提取
    activate Json
    Json-->>Router: 6A. [失败] validator抛错，返回 400 Bad Request
    Json->>Router: 6B. [成功] 反序列化成功，化为结构体(Req DTO)
    deactivate Json

    Router->>Handler: 7. 将准备好的 user_id 和 DTO 交给业务函数处理
    deactivate Router
    activate Handler
    
    Handler->>Service: 8. 结合数据库连接池 (AppState)，将指令下发业务逻辑
    activate Service
    
    Service->>Repo: 9. 进行业务前置计算/冲突检查后，发起数据库指令
    activate Repo
    
    Repo->>DB: 10. 使用 sqlx 宏 (query_as!) 开启防注入原生查询
    activate DB
    DB-->>Repo: 11. 返回一行或多行 Row
    deactivate DB
    
    Repo-->>Service: 12. 将 Row 映射为强类型贫血模型实体 (Model) 返回
    deactivate Repo
    
    Service->>Service: 13. [核心计算] 例如验证 Argon2 密码，抛出各类 AppError
    Service-->>Handler: 14. 抹去密码等敏感字段，转化为输出结构体 (Res DTO)
    deactivate Service
    
    Handler->>Handler: 15. 装载进 ApiResponse 标准信封 {code, msg, data}
    Handler-->>Middleware: 16. 返回 HTTP Response
    deactivate Handler
    
    Middleware->>Client: 17. 计算耗时打出完毕日志，返回给客户端
    deactivate Middleware
```

### 4. 双重 JWT 无状态鉴权与 RBAC 权限系统
不再使用一招鲜的万能 Token。实现了极致端到端独立的权限控制表：
- **Access Token 认证**：仅提供 15 分钟有效期，被 `AuthenticatedUser` 提取器拦截，用于常规业务接口放行。
- **Refresh Token 续签通道**：有效期 7 天。跨端前端项目通过调用 `POST /api/v1/auth/refresh` 可实现完全无感的自动续期。
- **RBAC 超级门卫**：载荷中嵌入了 `Role`（`user` / `admin`）。为管理员专用网络构建了 `AdminUser` 权限提取器，一旦检测到越权操作即刻硬性拒绝（403 Forbidden）。

### 5. ApiResponse 全局统一切面
所有接口强制返回统一规范的 JSON，并且内部集成各种各样的错误抛出。
```json
{
  "success": true,
  "code": 0,
  "msg": "操作成功",
  "data": { ... }
}
```
当系统级挂在遇到 `sqlx::Error` 和 `jsonwebtoken::errors` 时，使用 `From` trait，底层自动切面转换为 500 大报错或者 401 拒止日志，对 C 端脱敏只返回友好提示语。

### 6. OpenAPI (Swagger) 与 HTTP 原生测试生态
为了打造完美的微服务沉浸式开发对接体验，全面告别口头协议和零散的 Postman：
- **互动的 Swagger UI**：在所有响应结构和路由上挂载了 `utoipa` 宏，只需访问启动后的 `http://localhost:8080/swagger-ui` 即可享用官方自动刷新的可视化数据看板。
- **Jetbrains / VS Code 原生脚本**：提取了庞大的项目 `http/` 目录。使用 `.http` 格式对复杂的跨请求传递（Token 全局上下文替换）一通全包，开发调错如丝般顺滑。

---

## 🚀 本地开发指南

### 1. 环境准备与依赖
确保本机安装了 Rust (cargo)、Docker 以及专为项目数据库做编译期验证的 `sqlx-cli` 工具：
```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

### 2. 环境变量配置
在项目根目录创建 `.env` 文件，完善你的配置：
```env
# 启动端口
PORT=8080

# 数据库连接，例如：
DATABASE_URL=postgres://postgres:password@localhost:5432/crowdfunding

# 双令牌独立密钥（严禁线上使用弱密钥）
JWT_ACCESS_SECRET=your_super_secret_access_key
JWT_REFRESH_SECRET=your_super_secret_refresh_key_for_7_days
```

### 3. 启动数据库与数据迁移
确保你的 Postgres 已经跑起来了。本项目使用严格的 Migration (迁移) 机制来管理表结构：

**创建与执行迁移：**
```bash
# 1. 生成一个新的时间戳迁移文件 (不要修改历史文件！)
sqlx migrate add <description_name>

# 2. 将 migrations/ 下未执行的 SQL 脚本按顺序推送到真实数据库中
sqlx migrate run
```

**更新离线宏检查字典：**
为了让 CI/CD 能够在没有真实数据库连接的情况下通过编译，并在 IDE 中消除假报错，每次修改完相关的 `query!` 查询后，一定要执行：
```bash
# 生成或更新 .sqlx/ 离线元数据目录
cargo sqlx prepare
```

> **💡 高级技巧：处理 Postgres 自定义类型的类型覆盖 (Type Override)**  
> 当我们在 Postgres 中创建了自定义的类型（如 `CREATE TYPE gender_enum AS ENUM (...)`），Rust 强类型的 `query_as!` 宏在编译阶段无法自动推算它的内存布局（因其不在宏的内置通用字典里）。
> 因此我们需要使用 `sqlx` 提供的**运行时免检通行证 (Type Override)** 语法：
> 1. 在 Rust 参数传递时：写 `gender as _`
> 2. 在 SQL RETURNING/SELECT 时：写 `gender as "gender: _"`
> 然后在接收的 Model 侧为其标记 `#[derive(sqlx::Type)]`，Rust 就会彻底接管类型的强约束！

### 4. 运行与编译说明

**1. 语法检查 (cargo check)**
如果你只是想检查代码有没有拼写错误、类型匹配错误，而不想花费时间去生成最终的二进制可执行文件，请使用：
```bash
cargo check
```
它只会运行编译器前端，速度极快，是 Rust 开发者最常用的日常命令。

**2. 最快启动项目 (cargo run)**
编译并直接运行当前项目：
```bash
cargo run
```

**3. 查看完整宏报错日志**
如果你在编写带 `sqlx` 宏的自动化脚本，或想彻底看到所有因为宏替换而导致的 `stderr` 管道内容，可使用：
```bash
cargo build 2>&1 || true
```

---

## 📜 当前可用 API (v1 API Reference)

### 📌 Auth 模块 (公开与续期)
- `POST /api/v1/auth/register`: 新账号注册
- `POST /api/v1/auth/login`: 账号密码校验并安全下发 Access/Refresh 双端 Token
- `POST /api/v1/auth/refresh`: 核心长效保活信道。消耗有效 Refresh Token 单独换取全系新牌照

### 🔐 User 模块 (需携头部: `Authorization: Bearer <Access-Token>`)
- `GET /api/v1/users/me`: 获取当前拥有者的极密隐私库数据
- `PUT /api/v1/users`: 局部资料更新（如用户名/年龄/性别）
- `PATCH /api/v1/users/password`: 严格风控环节（依赖高强度的 Argon2 就地校对后覆盖改密）
- `DELETE /api/v1/users/me`: 主动退网功能（标记 `is_deactivated = true`），Token 入黑洞

### 🛡 Admin 模块 (必需超管身份: `Role::Admin`)
- `DELETE /api/v1/admin/users/{id}`: 天降封禁之锤（强行设置 `is_banned = true`）
