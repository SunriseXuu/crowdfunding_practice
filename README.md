# Crowdfunding Practice (众筹系统后端实战)

> 💡 **项目愿景**：从零构建一个企业级标准、极致严谨的 Web3/众筹业务 Rust 后端。本仓库旨在记录一个完备的高性能 Rust 后端的开发历程，并作为一份严谨的后端架构实践指南。

## 🛠 技术栈与核心选型

为了保证项目的现代化与健壮性，本项目全系采用了各库的最新稳定版本组合：

- **核心 Web 框架**：`axum 0.8` (极速且符合人体工程学的异步路由层)
- **异步运行时**：`tokio 1.0` (业界标杆的基础运行时引擎)
- **硬核数据库驱动**：`sqlx 0.8` + PostgreSQL 17 (自带编译期强类型 SQL 检查，告别拼写错误)
- **数据流转与序列化**：`serde 1.0` (强大的数据结构序列化引擎)
- **API 数据校验**：`validator 0.20` (基于宏机制的数据守卫)
- **统一错误捕获**：`thiserror 2.0` (宏驱动的业务级统一异常枚举)
- **全链路日志追踪**：`tower-http 0.6` + `tracing 0.1` (结构化与非阻塞日志记录)
- **JWT 鉴权机制**：`jsonwebtoken 10.3` (基于双 Token 无状态机制)

## 🏗 企业级严谨架构规范

本项目**极其严苛**地遵守以下三点后端黄金准则，坚决抵制意大利面条式的代码堆砌：

1. **三层隔离架构 (Router -> Handler -> Service -> Repository)**
   - `Handler`：**绝不**包含任何一行查库或计算逻辑。只做两件事：参数验收提取、调用 Service 后格式化返回 JSON。
   - `Service`：业务的心脏。资金计算、状态推演、并发逻辑统统包在这里。
   - `Repository`：持久层。**绝不**包含业务逻辑判断，只负责拼接纯净的 SQL 并交由 `sqlx` 执行查询或写入。

2. **DTO 与 Model 的绝对隔离 (防火墙法则)**
   - 暴露给前端接收的数据（如 `UserLoginReq`）与数据库底层映射的实体（如 `User`）老死不相往来。所有数据进出必须经过中间结构（DTO）的清洗与过滤，以此保证 DB 表结构的高级安全与前端接口解耦。

3. **全局错误池收敛机制**
   - 所有的深层错误（例如：账号不存在、众筹进度已满）都在触发点抛出一个自定义的强类型 `AppError`。在 Axum 的顶层 `IntoResponse` 切面统一捕获，并转化为前台熟悉的 `{ "code": 40xxx, "msg": "xx", "data": null }`。

## 📂 核心目录索引 (当前阶段已搭建完毕)

```text
src/
├── config/             # 环境变量绑定与数据源配置 [AppConfig]
├── dto/                # 与前端通讯的数据契约层
│   ├── request/        # 进站数据校验与提取层
│   └── response/       # 出站脱敏包装层
├── error/              # 统筹全局异常处理与错误码映射
├── handler/            # 路由解析与 Controller
├── middleware/         # 鉴权 Token 拦截器与日志挂载点
├── model/              # Postgres DB 物理层表结构映射实体
├── repository/         # DB CRUD 原生 SQL 封装
├── router/             # Axum 路由分发大厅
├── service/            # 极高纯度的业务逻辑校验与计算引擎
└── util/               # 分页、时间、哈希加密等脚手架工具
```

## 🚀 本地开发起步指南

如果你 clone 了这个项目，可以通过以下流程启动纯净的开发环境：

### 1. 准备全局开发工具 (推荐在 Mac/Linux 终端操作)

```bash
# 安装 rust-watch，这是 Rust 领域的 nodemon，保存自动热重载
cargo install cargo-watch

# 安装 sqlx-cli 用于执行 DB 结构迁移 (注意不含任何默认 feature，为了清爽只保留 postgres)
cargo install sqlx-cli --no-default-features --features postgres
```

### 2. 准备 Docker 环境与配置

1. 根目录下复制一份环境变量模板：
```bash
cp .env.example .env
# 自行修改里面的账密 (已在 gitignore 屏蔽)
```
2. 拉起高稳定度的 Debian 版 Postgres 17 数据库：
```bash
docker compose up -d
```

### 3. 一键挂载启动

启动本地后端服务，附带修改代码自动热刷新：
```bash
cargo watch -x run
```
