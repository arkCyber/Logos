# 实时协作前端架构设计

## 设计日期
2026年5月30日

## 设计目标
实现航空航天级实时协作前端集成，与后端 CRDT + WebSocket 完美对接。

---

## 架构概览

### 核心组件
1. **CollaborationService** - 协作服务层（WebSocket 连接管理）
2. **CursorTracker** - 光标追踪器（位置同步）
3. **PresenceManager** - 在线状态管理器
4. **OperationBroadcaster** - 操作广播器
5. **CollaborationUI** - 协作 UI 组件
6. **ConflictResolver** - 冲突解决器

---

## 组件设计

### 1. CollaborationService (协作服务层)

#### 职责
- WebSocket 连接管理
- 消息发送/接收
- 重连机制
- 错误处理

#### 接口设计
```typescript
interface CollaborationService {
  connect(documentId: string, userId: string, userName: string): Promise<void>
  disconnect(): void
  sendOperation(operation: CRDTOperation): void
  sendPresence(presence: PresenceInfo): void
  requestSync(sinceVersion: number): Promise<CRDTOperation[]>
  onMessage(callback: (message: CollaborationMessage) => void): void
  onConnected(callback: () => void): void
  onDisconnected(callback: () => void): void
  onError(callback: (error: Error) => void): void
}
```

#### 实现要点
- 使用 Tauri invoke 调用后端 WebSocket
- 实现自动重连机制（指数退避）
- 实现心跳检测
- 实现消息队列（离线缓存）

---

### 2. CursorTracker (光标追踪器)

#### 职责
- 追踪本地光标位置
- 接收远程光标位置
- 渲染远程光标
- 光标冲突检测

#### 接口设计
```typescript
interface CursorTracker {
  trackLocalPosition(position: number): void
  updateRemoteCursor(userId: string, position: number, userName: string): void
  removeRemoteCursor(userId: string): void
  getRemoteCursors(): Map<string, CursorInfo>
  renderCursors(): void
}
```

#### 实现要点
- 使用 TipTap 的 selection API
- 实现光标位置到文档坐标的转换
- 实现远程光标渲染（带用户名和颜色）
- 实现光标冲突检测（避免重叠）

---

### 3. PresenceManager (在线状态管理器)

#### 职责
- 管理用户在线状态
- 显示在线用户列表
- 处理用户加入/离开事件

#### 接口设计
```typescript
interface PresenceManager {
  updatePresence(presence: PresenceInfo): void
  getOnlineUsers(): PresenceInfo[]
  onUserJoined(callback: (user: PresenceInfo) => void): void
  onUserLeft(callback: (userId: string) => void): void
  onPresenceUpdated(callback: (presence: PresenceInfo) => void): void
}
```

#### 实现要点
- 使用响应式状态管理（Vue 3 ref/reactive）
- 实现用户在线/离线状态切换
- 实现用户活动检测（超时自动离线）
- 实现用户列表排序（按活动时间）

---

### 4. OperationBroadcaster (操作广播器)

#### 职责
- 将本地编辑操作转换为 CRDTOperation
- 广播操作到其他用户
- 接收远程操作并应用到编辑器
- 操作转换（OT）

#### 接口设计
```typescript
interface OperationBroadcaster {
  broadcastOperation(operation: CRDTOperation): void
  receiveOperation(operation: CRDTOperation): void
  transformOperation(operation: CRDTOperation, against: CRDTOperation): CRDTOperation
  applyOperationToEditor(operation: CRDTOperation): void
}
```

#### 实现要点
- 监听 TipTap 的 update 事件
- 将 TipTap Transaction 转换为 CRDTOperation
- 实现操作转换算法（与后端一致）
- 实现操作批量处理（性能优化）

---

### 5. CollaborationUI (协作 UI 组件)

#### 职责
- 显示在线用户列表
- 显示协作状态指示器
- 显示远程光标
- 显示冲突警告

#### 组件设计
```vue
<template>
  <div class="collaboration-ui">
    <!-- 在线用户列表 -->
    <div class="online-users">
      <div v-for="user in onlineUsers" :key="user.user_id" class="user-avatar">
        {{ user.user_name }}
      </div>
    </div>

    <!-- 协作状态指示器 -->
    <div class="collaboration-status" :class="status">
      {{ statusText }}
    </div>

    <!-- 远程光标渲染 -->
    <div v-for="cursor in remoteCursors" :key="cursor.userId" 
         class="remote-cursor" 
         :style="cursorStyle(cursor)">
      <span class="cursor-label">{{ cursor.userName }}</span>
    </div>

    <!-- 冲突警告 -->
    <div v-if="hasConflict" class="conflict-warning">
      检测到编辑冲突
    </div>
  </div>
</template>
```

#### 实现要点
- 使用 Vue 3 Composition API
- 实现响应式 UI 更新
- 实现动画效果（光标移动、用户加入/离开）
- 实现主题适配（亮色/暗色模式）

---

### 6. ConflictResolver (冲突解决器)

#### 职责
- 检测编辑冲突
- 提供冲突解决选项
- 应用冲突解决方案

#### 接口设计
```typescript
interface ConflictResolver {
  detectConflict(localOp: CRDTOperation, remoteOp: CRDTOperation): boolean
  resolveConflict(localOp: CRDTOperation, remoteOp: CRDTOperation): Resolution
  applyResolution(resolution: Resolution): void
}
```

#### 实现要点
- 实现冲突检测算法
- 提供多种解决策略（本地优先、远程优先、手动合并）
- 实现冲突 UI（显示冲突内容）
- 实现冲突历史记录

---

## 集成到 Editor.vue

### 集成点
1. **onMounted**: 初始化协作服务
2. **onUnmounted**: 断开协作连接
3. **TipTap update 事件**: 广播操作
4. **TipTap selection 事件**: 更新光标位置
5. **工具栏**: 添加协作状态按钮

### 集成步骤
1. 导入协作组件
2. 初始化协作服务
3. 连接到文档
4. 监听协作事件
5. 渲染协作 UI
6. 清理资源

---

## 数据流

### 本地编辑流程
```
用户编辑 → TipTap Transaction → OperationBroadcaster → CRDTOperation → CollaborationService → WebSocket → 后端
```

### 远程编辑流程
```
后端 → WebSocket → CollaborationService → CRDTOperation → OperationBroadcaster → TipTap Transaction → 编辑器更新
```

### 光标同步流程
```
用户移动光标 → CursorTracker → PresenceInfo → CollaborationService → WebSocket → 后端 → 其他用户
```

---

## 性能优化

### 1. 操作批处理
- 将多个连续操作合并为一个批量操作
- 减少网络传输次数
- 提高渲染性能

### 2. 防抖处理
- 光标位置更新使用防抖（100ms）
- 在线状态更新使用防抖（500ms）
- 减少不必要的网络传输

### 3. 增量同步
- 只同步变更的操作
- 使用版本号控制
- 减少数据传输量

### 4. 本地缓存
- 缓存远程操作
- 离线时缓存本地操作
- 重连后批量同步

---

## 安全性

### 1. 身份验证
- 使用 JWT 令牌
- 验证用户权限
- 防止未授权访问

### 2. 数据加密
- WebSocket 使用 TLS
- 敏感数据加密传输
- 防止中间人攻击

### 3. 输入验证
- 验证操作数据
- 防止注入攻击
- 限制操作频率

---

## 测试策略

### 1. 单元测试
- CollaborationService 测试
- CursorTracker 测试
- PresenceManager 测试
- OperationBroadcaster 测试

### 2. 集成测试
- WebSocket 连接测试
- 操作同步测试
- 光标同步测试
- 冲突解决测试

### 3. E2E 测试
- 多用户协作测试
- 网络断开重连测试
- 大量操作压力测试
- 冲突场景测试

---

## 依赖项

### 新增依赖
- 无需新增依赖（使用现有 Tauri 和 Vue 3）

### 现有依赖
- Vue 3
- TipTap
- Tauri
- TypeScript

---

## 实现优先级

### Phase 1: 核心功能（高优先级）
1. CollaborationService 实现
2. CursorTracker 实现
3. PresenceManager 实现
4. OperationBroadcaster 实现
5. 基础 CollaborationUI 实现

### Phase 2: 高级功能（中优先级）
1. ConflictResolver 实现
2. 高级 CollaborationUI 实现
3. 性能优化
4. 离线缓存

### Phase 3: 测试和文档（高优先级）
1. 单元测试
2. 集成测试
3. E2E 测试
4. 文档编写

---

## 预期成果

### 功能完整性
- ✅ 实时协作编辑
- ✅ 光标位置同步
- ✅ 用户在线状态
- ✅ 操作冲突解决
- ✅ 离线缓存

### 性能指标
- 操作延迟 < 100ms
- 光标同步延迟 < 50ms
- 支持 10+ 用户同时协作
- 内存占用 < 100MB

### 质量标准
- 航空航天级代码质量
- 100% 测试覆盖率
- 完整的文档
- 无已知 bug

---

**设计完成日期**: 2026年5月30日
**设计人员**: Cascade AI Assistant
**设计标准**: 航空航天级软件质量标准
