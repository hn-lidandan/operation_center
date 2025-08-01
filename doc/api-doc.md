# Operation Center API 文档

## 基础信息

- **基础URL**: `http://localhost:8080`
- **内容类型**: `application/json`

## API 接口列表

### 安装管理

#### 1. 解压安装包

**请求**
- **路径**: `/api/unzip`
- **方法**: `POST`
- **参数**:
  ```json
  {
    "zip_path": "安装包路径，例如: /Users/ldd/Workspaces/zip/dss-gateway-mac-app.zip"
  }
  ```

**响应**
- **成功 (200)**: 文本响应，包含"解压成功"字样
  ```
  解压成功! 输出: 已解压 /path/to/file.zip 到同级目录
  ```
- **失败 (400/500)**: 错误信息

#### 2. 获取安装组件信息

**请求**
- **路径**: `/api/setup/components`
- **方法**: `POST`
- **参数**:
  ```json
  {
    "dir_path": "解压后的目录路径",
    "config_file": "要使用的配置文件名称，例如：value.yml"
  }
  ```

**响应**
- **成功 (200)**: JSON格式的组件信息
  ```json
  {
    "success": true,
    "totalComponents": 3,
    "components": [
      { "name": "核心组件", "version": "1.0.0" },
      { "name": "界面组件", "version": "1.0.0" },
      { "name": "数据组件", "version": "0.9.5" }
    ]
  }
  ```
- **失败 (400/500)**: 错误信息

#### 3. 执行安装

**请求**
- **路径**: `/api/setup`
- **方法**: `POST`
- **参数**:
  ```json
  {
    "dir_path": "解压后的目录路径",
    "config_file": "要使用的配置文件名称，例如：value.yml"
  }
  ```

**响应**
- **成功 (200)**: 流式响应，包含安装进度信息
  ```
  开始安装...
  使用配置文件: value.yml
  组件: 核心组件 1.0.0
  - 复制文件...
  - 配置组件...
  - 安装完成
  
  组件: 界面组件 1.0.0
  - 复制文件...
  - 配置组件...
  - 安装完成
  
  组件: 数据组件 0.9.5
  - 复制文件...
  - 配置组件...
  - 安装完成
  
  安装完成!
  ```
  
  > **注意**: 响应中通过"组件:"关键词来标识每个组件的安装进度，前端应根据组件数量来计算进度条。
  
- **失败 (400/500)**: 错误信息

### 配置管理

#### 3. 获取配置信息

**请求**
- **路径**: `/api/find_setting`
- **方法**: `GET`
- **查询参数**: `dir_path=解压后的目录路径`

**响应**
- **成功 (200)**: JSON格式的配置信息
  ```json
  {
    "host": "localhost",
    "port": "8080",
    "username": "admin",
    "password": "******",
    "db_name": "operation_center",
    "log_level": "info"
  }
  ```
- **失败 (400/404/500)**: 错误信息

#### 4. 获取多个配置文件信息

**请求**
- **路径**: `/api/find_settings`
- **方法**: `GET`
- **查询参数**: `dir_path=解压后的目录路径`

**响应**
- **成功 (200)**: JSON格式的多个配置文件信息
  ```json
  {
    "value.yml": {
      "host": "localhost",
      "port": "8080",
      "username": "admin",
      "password": "******",
      "db_name": "operation_center"
    },
    "advanced.yml": {
      "log_level": "info",
      "auto_start": "true",
      "backup_enabled": "true",
      "max_log_size": "100MB"
    }
  }
  ```
- **失败 (400/404/500)**: 错误信息

#### 5. 保存配置

**请求**
- **路径**: `/api/save_setting`
- **方法**: `POST`
- **查询参数**: `dir_path=解压后的目录路径`
- **参数**: 
  ```json
  {
    "file_name": "要保存的文件名，例如：value.yml",
    "settings": {
      "host": "localhost",
      "port": "8080",
      "username": "admin",
      "password": "******",
      "db_name": "operation_center",
      "log_level": "info"
    }
  }
  ```

**响应**
- **成功 (200)**: "配置保存成功到文件 value.yml"
- **失败 (400/500)**: 错误信息

### 信息查询

#### 6. 获取版本信息

**请求**
- **路径**: `/api/current_version`
- **方法**: `GET`
- **查询参数**: `dir_path=解压后的目录路径`

**响应**
- **成功 (200)**: 版本信息文本
  ```
  版本: 1.0.0-mock
  发布日期: 2025-07-12
  描述: 这是一个模拟的版本信息
  组件:
    - 名称: 核心组件
      版本: 1.0.0
    - 名称: 界面组件
      版本: 1.2.3
    - 名称: 数据组件
      版本: 0.9.5
  ```
- **失败 (400/404/500)**: 错误信息

#### 7. 获取历史版本信息

**请求**
- **路径**: `/api/history_version`
- **方法**: `POST`
- **参数**:
  ```json
  {
    "dir_path": "解压后的目录路径"
  }
  ```

**响应**
- **成功 (200)**: 历史版本信息文本
  ```
  历史版本记录:

  版本: 0.9.0-beta
  发布日期: 2023-01-05
  描述: 测试版本

  版本: 1.0.0
  发布日期: 2023-02-15
  描述: 首次正式发布
  ```
- **失败 (400/404/500)**: 错误信息



### 升级管理

#### 9. 升级分析

**请求**
- **路径**: `/api/update/analysis`
- **方法**: `GET`

**响应**
- **成功 (200)**:
  ```json
  {
    "success": true,
    "currentSystem": "当前系统版本: 1.0.0\n发布日期: 2023-01-15\n组件数量: 2",
    "updateSystem": "升级系统版本: 1.2.0\n发布日期: 2023-06-10\n组件数量: 3",
    "updateItems": "需要升级的组件:\n- 核心组件: 1.0.0 -> 1.1.0\n- 界面组件: 1.0.0 -> 1.2.0\n- 新增: 数据组件 0.9.5"
  }
  ```
- **失败 (500)**: 错误信息

#### 10. 获取升级组件信息

**请求**
- **路径**: `/api/update/components`
- **方法**: `POST`
- **参数**:
  ```json
  {
    "dir_path": "升级包解压后的目录路径"
  }
  ```

**响应**
- **成功 (200)**: JSON格式的组件信息
  ```json
  {
    "success": true,
    "totalComponents": 3,
    "components": [
      { "name": "核心组件", "version": "1.1.0" },
      { "name": "界面组件", "version": "1.2.0" },
      { "name": "数据组件", "version": "0.9.5" }
    ]
  }
  ```
- **失败 (400/500)**: 错误信息

#### 11. 执行更新

**请求**
- **路径**: `/api/update`
- **方法**: `POST`
- **参数**:
  ```json
  {
    "dir_path": "升级包解压后的目录路径"
  }
  ```

**响应**
- **成功 (200)**: 流式响应，包含更新进度信息
  ```
  开始更新组件...
  组件: 核心组件 1.0.0 -> 1.1.0
  - 备份旧版本...
  - 更新文件...
  - 更新配置...
  - 更新完成
  
  组件: 界面组件 1.0.0 -> 1.2.0
  - 备份旧版本...
  - 更新文件...
  - 更新配置...
  - 更新完成
  
  组件: 数据组件 0.9.5 (新增)
  - 安装文件...
  - 配置组件...
  - 安装完成
  
  更新组件完成!
  ```
  
  > **注意**: 响应中通过"组件:"关键词来标识每个组件的更新进度，前端应根据组件数量来计算进度条。
  
- **失败 (400/500)**: 错误信息



## 数据结构

### 配置项结构
配置项为键值对形式，常见的配置项包括：
- `host`: 主机名
- `port`: 端口号
- `username`: 用户名
- `password`: 密码
- `db_name`: 数据库名称
- `log_level`: 日志级别
- `auto_start`: 自动启动
- `backup_enabled`: 是否启用备份
- `max_log_size`: 最大日志大小
- `update_channel`: 更新渠道

### 版本信息结构
版本信息包含：
- 版本号
- 发布日期
- 描述信息
- 组件列表，每个组件包含名称和版本号

## 错误处理

所有接口在发生错误时会返回以下格式的错误信息：

```json
{
  "error": "错误描述信息",
  "code": 400 // 错误代码
}
```

常见错误代码：
- `400`: 请求参数错误
- `404`: 资源未找到
- `500`: 服务器内部错误 