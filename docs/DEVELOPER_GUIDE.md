# 开发者指南

## 概述

本文档面向开发人员，介绍Logos智道办公软件项目的架构、开发环境设置、代码规范和贡献指南。

## 技术栈

### 前端

- **框架**: Vue 3 (Composition API)
- **构建工具**: Vite
- **编辑器**: Tiptap
- **PDF渲染**: pdf.js
- **状态管理**: 本地状态（计划引入Pinia）
- **测试**: Vitest + Vue Test Utils
- **语言**: TypeScript

### 后端

- **语言**: Rust
- **框架**: Axum
- **数据库**: SQLite (sqlx)
- **认证**: JWT (jsonwebtoken)
- **密码**: bcrypt
- **日志**: tracing
- **测试**: 内置测试框架

## 项目结构

```
logos-zhidao-office/
├── src/                          # 前端源代码
│   ├── components/              # Vue组件
│   ├── composables/              # Vue组合式函数
│   ├── utils/                   # 工具函数
│   ├── services/                # API服务
│   └── App.vue                  # 根组件
├── spreadsheet-service/          # 后端Rust服务
│   ├── src/                     # Rust源代码
│   │   ├── main.rs             # 入口文件
│   │   ├── handlers.rs         # API处理器
│   │   ├── auth.rs             # 认证模块
│   │   ├── db.rs               # 数据库模块
│   │   ├── validation.rs       # 验证模块
│   │   ├── csrf.rs             # CSRF保护
│   │   ├── rate_limit.rs       # 速率限制
│   │   ├── transaction.rs      # 事务管理
│   │   ├── error.rs            # 错误处理
│   │   └── config.rs           # 配置管理
│   ├── tests/                   # 测试文件
│   └── Cargo.toml              # Rust依赖
├── docs/                        # 文档
├── public/                      # 静态资源
└── package.json                 # Node.js依赖
```

## 开发环境设置

### 前端开发环境

1. **安装Node.js** (>=16):
```bash
# 使用nvm安装
nvm install 18
nvm use 18
```

2. **安装依赖**:
```bash
npm install
```

3. **启动开发服务器**:
```bash
npm run dev
```

4. **运行测试**:
```bash
npm run test
```

5. **构建生产版本**:
```bash
npm run build
```

### 后端开发环境

1. **安装Rust**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **进入后端目录**:
```bash
cd spreadsheet-service
```

3. **运行开发服务器**:
```bash
cargo run
```

4. **运行测试**:
```bash
cargo test
```

5. **构建生产版本**:
```bash
cargo build --release
```

## 代码规范

### 前端代码规范

#### TypeScript

- 使用TypeScript strict模式
- 所有函数必须有类型注解
- 避免使用`any`类型
- 使用接口定义数据结构

#### Vue组件

- 使用Composition API
- 组件文件使用PascalCase命名
- Props和Emits必须有类型定义
- 使用`<script setup>`语法

#### 命名约定

- 组件: PascalCase (e.g., `TypstPreviewEditor.vue`)
- 函数: camelCase (e.g., `updateEditorState`)
- 常量: UPPER_SNAKE_CASE (e.g., `MAX_RETRY_COUNT`)
- 接口: PascalCase (e.g., `EditorState`)

### 后端代码规范

#### Rust

- 使用`cargo fmt`格式化代码
- 使用`cargo clippy`检查代码
- 避免使用`unwrap()`，使用`?`或`expect()`
- 错误处理使用`Result`类型

#### 命名约定

- 结构体: PascalCase (e.g., `SpreadsheetError`)
- 函数: snake_case (e.g., `validate_sheet_name`)
- 常量: UPPER_SNAKE_CASE (e.g., `MAX_RETRY_COUNT`)
- 模块: snake_case (e.g., `error_handler`)

## 测试

### 前端测试

#### 单元测试

```typescript
import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import MyComponent from './MyComponent.vue';

describe('MyComponent', () => {
  it('renders correctly', () => {
    const wrapper = mount(MyComponent);
    expect(wrapper.exists()).toBe(true);
  });
});
```

#### 运行测试

```bash
npm run test
npm run test:coverage
```

### 后端测试

#### 单元测试

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_validation() {
        let result = validate_sheet_name("Test");
        assert!(result.is_ok());
    }
}
```

#### 集成测试

```rust
#[tokio::test]
async fn test_api_endpoint() {
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

#### 运行测试

```bash
cargo test
cargo test --release
cargo tarpaulin --out Html
```

## 航空航天级开发标准

### 错误处理

所有错误必须被捕获和处理，不能让应用崩溃。

```typescript
try {
  await riskyOperation();
} catch (error) {
  logger.error('Operation failed', error, LogCategory.SYSTEM);
  throw createError(ErrorCode.OPERATION_FAILED, error);
}
```

### 日志记录

所有关键操作必须记录日志。

```typescript
logger.info('User logged in', { userId }, LogCategory.SECURITY);
logger.error('Database connection failed', error, LogCategory.SYSTEM);
```

### 输入验证

所有用户输入必须验证和清理。

```typescript
const validated = validateInput(userInput);
if (!validated.valid) {
  throw createError(ErrorCode.INVALID_INPUT, validated.errors);
}
```

### 安全性

- 使用参数化查询防止SQL注入
- 使用CSRF token防止CSRF攻击
- 使用速率限制防止DDoS攻击
- 使用JWT进行认证

## 贡献指南

### 提交代码

1. Fork仓库
2. 创建功能分支: `git checkout -b feature/amazing-feature`
3. 提交更改: `git commit -m 'Add amazing feature'`
4. 推送分支: `git push origin feature/amazing-feature`
5. 创建Pull Request

### 提交信息格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

类型:
- `feat`: 新功能
- `fix`: 修复bug
- `docs`: 文档更新
- `style`: 代码格式
- `refactor`: 重构
- `test`: 测试
- `chore`: 构建/工具

### 代码审查

- 所有代码必须通过CI/CD检查
- 必须有对应的测试
- 必须更新相关文档
- 必须通过代码审查

## 部署

### 前端部署

```bash
npm run build
# 将dist目录部署到CDN或静态服务器
```

### 后端部署

```bash
cargo build --release
# 将target/release部署到服务器
```

### Docker部署

```dockerfile
# Dockerfile
FROM node:18 as frontend
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

FROM rust:1.70 as backend
WORKDIR /app
COPY spreadsheet-service .
RUN cargo build --release
```

## 性能优化

### 前端优化

- 使用虚拟滚动处理大列表
- 使用懒加载组件
- 使用防抖和节流
- 优化图片加载

### 后端优化

- 使用数据库连接池
- 添加查询缓存
- 使用异步处理
- 优化数据库索引

## 监控和日志

### 日志级别

- TRACE: 最详细的追踪信息
- DEBUG: 调试信息
- INFO: 一般信息
- WARNING: 警告信息
- ERROR: 错误信息
- CRITICAL: 严重错误
- FATAL: 致命错误

### 监控指标

- 请求响应时间
- 错误率
- 数据库查询时间
- 内存使用
- CPU使用

## 故障排除

### 常见问题

1. **前端构建失败**
   - 检查Node.js版本
   - 清除node_modules重新安装
   - 检查依赖版本冲突

2. **后端编译失败**
   - 检查Rust版本
   - 更新依赖: `cargo update`
   - 检查Rust工具链

3. **测试失败**
   - 检查测试环境配置
   - 清理测试数据库
   - 检查mock数据

## 资源

- [Vue 3文档](https://vuejs.org/)
- [Rust文档](https://doc.rust-lang.org/)
- [Axum文档](https://docs.rs/axum/)
- [Tiptap文档](https://tiptap.dev/)
- [Typst文档](https://typst.app/docs/)
