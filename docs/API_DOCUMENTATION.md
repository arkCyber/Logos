# API文档

## 概述

本文档描述了spreadsheet-service REST API的所有端点、请求/响应格式和认证方式。

## 基础信息

- **Base URL**: `http://localhost:8080/api`
- **认证方式**: JWT Bearer Token
- **数据格式**: JSON
- **字符编码**: UTF-8

## 认证

### 获取Token

**端点**: `POST /api/auth/register`

**请求体**:
```json
{
  "username": "string",
  "email": "string",
  "password": "string"
}
```

**响应**:
```json
{
  "token": "string",
  "user": {
    "id": "string",
    "username": "string",
    "email": "string"
  }
}
```

### 登录

**端点**: `POST /api/auth/login`

**请求体**:
```json
{
  "username": "string",
  "password": "string"
}
```

**响应**:
```json
{
  "token": "string",
  "user": {
    "id": "string",
    "username": "string",
    "email": "string"
  }
}
```

### 使用Token

在请求头中添加:
```
Authorization: Bearer <token>
```

## 工作表API

### 获取所有工作表

**端点**: `GET /api/sheets`

**查询参数**:
- `page`: 页码（默认: 1）
- `limit`: 每页数量（默认: 10）
- `sort_by`: 排序字段（默认: created_at）
- `sort_order`: 排序方向（asc/desc，默认: desc）

**响应**:
```json
{
  "sheets": [
    {
      "id": "string",
      "name": "string",
      "created_at": "string",
      "updated_at": "string"
    }
  ],
  "total": 100,
  "page": 1,
  "limit": 10
}
```

### 创建工作表

**端点**: `POST /api/sheets`

**请求体**:
```json
{
  "name": "string"
}
```

**响应**:
```json
{
  "id": "string",
  "name": "string",
  "created_at": "string",
  "updated_at": "string"
}
```

### 获取工作表详情

**端点**: `GET /api/sheets/{id}`

**响应**:
```json
{
  "id": "string",
  "name": "string",
  "created_at": "string",
  "updated_at": "string",
  "cells": []
}
```

### 更新工作表

**端点**: `PUT /api/sheets/{id}`

**请求体**:
```json
{
  "name": "string"
}
```

**响应**:
```json
{
  "id": "string",
  "name": "string",
  "created_at": "string",
  "updated_at": "string"
}
```

### 删除工作表

**端点**: `DELETE /api/sheets/{id}`

**响应**: `204 No Content`

## 单元格API

### 获取单元格

**端点**: `GET /api/sheets/{sheet_id}/cells`

**查询参数**:
- `row`: 行号
- `col`: 列号

**响应**:
```json
{
  "cells": [
    {
      "id": "string",
      "sheet_id": "string",
      "row": 0,
      "col": 0,
      "value": "string",
      "formula": "string",
      "style": "object",
      "created_at": "string",
      "updated_at": "string"
    }
  ]
}
```

### 创建单元格

**端点**: `POST /api/sheets/{sheet_id}/cells`

**请求体**:
```json
{
  "row": 0,
  "col": 0,
  "value": "string",
  "formula": "string",
  "style": "object"
}
```

**响应**:
```json
{
  "id": "string",
  "sheet_id": "string",
  "row": 0,
  "col": 0,
  "value": "string",
  "formula": "string",
  "style": "object",
  "created_at": "string",
  "updated_at": "string"
}
```

### 更新单元格

**端点**: `PUT /api/sheets/{sheet_id}/cells/{id}`

**请求体**:
```json
{
  "value": "string",
  "formula": "string",
  "style": "object"
}
```

**响应**:
```json
{
  "id": "string",
  "sheet_id": "string",
  "row": 0,
  "col": 0,
  "value": "string",
  "formula": "string",
  "style": "object",
  "created_at": "string",
  "updated_at": "string"
}
```

### 删除单元格

**端点**: `DELETE /api/sheets/{sheet_id}/cells/{id}`

**响应**: `204 No Content`

### 批量创建单元格

**端点**: `POST /api/sheets/{sheet_id}/cells/batch`

**请求体**:
```json
{
  "cells": [
    {
      "row": 0,
      "col": 0,
      "value": "string"
    }
  ]
}
```

**响应**:
```json
{
  "created": 10,
  "failed": 0,
  "cells": []
}
```

## 公式API

### 计算公式

**端点**: `POST /api/formulas/calculate`

**请求体**:
```json
{
  "formula": "=SUM(A1:A10)",
  "cell_values": {
    "A1": "100",
    "A2": "200"
  }
}
```

**响应**:
```json
{
  "result": 300,
  "error": null
}
```

## 条件格式API

### 创建条件格式规则

**端点**: `POST /api/sheets/{sheet_id}/conditional-formatting`

**请求体**:
```json
{
  "name": "string",
  "condition": "string",
  "style": "object",
  "range": "A1:A10"
}
```

**响应**:
```json
{
  "id": "string",
  "sheet_id": "string",
  "name": "string",
  "condition": "string",
  "style": "object",
  "range": "string",
  "created_at": "string"
}
```

## 图表API

### 创建图表

**端点**: `POST /api/sheets/{sheet_id}/charts`

**请求体**:
```json
{
  "name": "string",
  "type": "line|bar|pie",
  "data_range": "A1:B10",
  "config": "object"
}
```

**响应**:
```json
{
  "id": "string",
  "sheet_id": "string",
  "name": "string",
  "type": "string",
  "data_range": "string",
  "config": "object",
  "created_at": "string"
}
```

## Excel导入/导出API

### 导入Excel

**端点**: `POST /api/excel/import`

**请求**: multipart/form-data
- `file`: Excel文件

**响应**:
```json
{
  "sheet_id": "string",
  "rows_imported": 100
}
```

### 导出Excel

**端点**: `GET /api/excel/export/{sheet_id}`

**查询参数**:
- `format`: xlsx|csv

**响应**: Excel文件二进制数据

## 错误响应

所有错误响应遵循以下格式:

```json
{
  "error": {
    "code": "string",
    "message": "string",
    "category": "string",
    "details": {}
  }
}
```

### 错误代码

| 代码 | HTTP状态 | 描述 |
|------|----------|------|
| VALIDATION_ERROR | 400 | 输入验证失败 |
| NOT_FOUND | 404 | 资源不存在 |
| UNAUTHORIZED | 401 | 未授权 |
| FORBIDDEN | 403 | 权限不足 |
| CONFLICT | 409 | 资源冲突 |
| RATE_LIMIT_EXCEEDED | 429 | 超过速率限制 |
| INTERNAL_ERROR | 500 | 内部服务器错误 |

## 速率限制

- 默认限制: 100请求/秒
- 突发容量: 200请求
- 超过限制返回: `429 Too Many Requests`

## CSRF保护

所有状态变更请求（POST、PUT、DELETE）需要CSRF token。

**请求头**:
```
X-CSRF-Token: <token>
```

**Cookie**:
```
csrf_token=<token>; HttpOnly; SameSite=Strict
```

## 健康检查

**端点**: `GET /health`

**响应**:
```json
{
  "status": "ok",
  "timestamp": "string"
}
```
