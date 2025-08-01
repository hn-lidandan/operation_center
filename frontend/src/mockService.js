// mockService.js - Mock服务实现
// 这个文件可以在不需要mock时轻松删除

// Mock开关控制 - 存储在localStorage中，便于持久化
export const mockConfig = {
  // 默认关闭mock服务
  get enabled() {
    return localStorage.getItem('useMockService') === 'true';
  },
  set enabled(value) {
    localStorage.setItem('useMockService', value);
  }
};

// Mock数据存储 - 按API路径和方法组织
const mockDataStore = {
  // 解压API
  '/api/unzip': {
    POST: (params) => {
      console.log('Mock: 解压文件', params);
      return {
        text: `解压成功! 输出: 已解压 ${params.zip_path} 到同级目录`
      };
    }
  },
  
  // 查找信息文件API
  '/api/current_version': {
    GET: (params) => {
      console.log('Mock: 查找信息文件', params);
      return {
        text: `版本: 1.0.0-mock
发布日期: ${new Date().toISOString().split('T')[0]}
描述: 这是一个模拟的版本信息
组件:
  - 名称: 核心组件
    版本: 1.0.0
  - 名称: 界面组件
    版本: 1.2.3
  - 名称: 数据组件
    版本: 0.9.5`
      };
    }
  },
  
  // 查找设置文件API - GET方式
  '/api/find_setting': {
    GET: (params) => {
      console.log('Mock: 查找设置文件(GET)', params);
      return {
        json: {
          "host": "localhost",
          "port": "8080",
          "username": "admin",
          "password": "******",
          "db_name": "operation_center",
          "log_level": "info"
        }
      };
    },
    // 新增POST方式的接口
    POST: (params, body) => {
      console.log('Mock: 查找设置文件(POST)', params, body);
      return {
        json: {
          "host": "localhost",
          "port": "8080",
          "username": "admin",
          "password": "******",
          "db_name": "operation_center",
          "log_level": "info",
          "auto_start": "true",
          "backup_enabled": "true",
          "max_log_size": "100MB",
          "update_channel": "stable"
        }
      };
    }
  },

  // 添加查找多个设置文件API - GET方式
  '/api/find_settings': {
    GET: (params) => {
      console.log('Mock: 查找多个设置文件(GET)', params);
      return {
        json: {
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
      };
    }
  },
  
  // 新增历史版本API
  '/api/history_version': {
    POST: (params, body) => {
      console.log('Mock: 获取历史版本', params, body);
      return {
        text: `历史版本记录:

版本: 0.9.0-beta
发布日期: 2023-01-05
描述: 测试版本

版本: 1.0.0
发布日期: 2023-02-15
描述: 首次正式发布

版本: 1.1.0
发布日期: 2023-04-20
描述: 功能增强更新

版本: 1.2.0
发布日期: 2023-06-10
描述: 稳定性提升版本`
      };
    }
  },
  
  // 保存设置API
  '/api/save_setting': {
    POST: (params, body) => {
      console.log('Mock: 保存设置', params, body);
      // 记录保存的设置，以便在下次加载时显示
      const savedSettings = body.settings || {};
      const dirPath = params.dir_path || 'default_path';
      
      // 在控制台中显示保存的设置
      console.log(`Mock: 已保存设置到路径 ${dirPath}:`, savedSettings);
      
      return {
        text: "配置保存成功"
      };
    }
  },
  
  // 安装设置API
  '/api/setup': {
    POST: (params, body) => {
      console.log('Mock: 安装设置', params, body);
      // 获取选择的配置文件
      const configFile = body.config_file || 'value.yml';
      console.log('使用配置文件:', configFile);
      
      // 这个API需要返回流式响应
      return {
        stream: [
          { text: `开始安装...\n使用配置文件: ${configFile}\n`, delay: 300 },
          { text: "组件: 核心组件 1.0.0\n", delay: 800 },
          { text: "- 复制文件...\n", delay: 600 },
          { text: "- 配置组件...\n", delay: 700 },
          { text: "- 安装完成\n\n", delay: 500 },
          
          { text: "组件: 界面组件 1.0.0\n", delay: 800 },
          { text: "- 复制文件...\n", delay: 600 },
          { text: "- 配置组件...\n", delay: 700 },
          { text: "- 安装完成\n\n", delay: 500 },
          
          { text: "组件: 数据组件 0.9.5\n", delay: 800 },
          { text: "- 复制文件...\n", delay: 600 },
          { text: "- 配置组件...\n", delay: 700 },
          { text: "- 安装完成\n\n", delay: 500 },
          
          { text: "安装完成!\n", delay: 1000 }
        ]
      };
    }
  },
  
  // 获取安装组件信息API
  '/api/setup/components': {
    POST: (params, body) => {
      console.log('Mock: 获取安装组件信息', params, body);
      // 获取选择的配置文件
      const configFile = body.config_file || 'value.yml';
      console.log('使用配置文件:', configFile);
      
      return {
        json: {
          success: true,
          totalComponents: 3,
          components: [
            { name: "核心组件", version: "1.0.0" },
            { name: "界面组件", version: "1.0.0" },
            { name: "数据组件", version: "0.9.5" }
          ]
        }
      };
    }
  },
  
  // 更新组件API
  '/api/update': {
    POST: (params, body) => {
      console.log('Mock: 更新组件', params, body);
      // 这个API需要返回流式响应
      return {
        stream: [
          { text: "开始更新组件...\n", delay: 300 },
          
          { text: "组件: 核心组件 1.0.0 -> 1.1.0\n", delay: 800 },
          { text: "- 备份旧版本...\n", delay: 600 },
          { text: "- 更新文件...\n", delay: 700 },
          { text: "- 更新配置...\n", delay: 800 },
          { text: "- 更新完成\n\n", delay: 500 },
          
          { text: "组件: 界面组件 1.0.0 -> 1.2.0\n", delay: 800 },
          { text: "- 备份旧版本...\n", delay: 600 },
          { text: "- 更新文件...\n", delay: 700 },
          { text: "- 更新配置...\n", delay: 800 },
          { text: "- 更新完成\n\n", delay: 500 },
          
          { text: "组件: 数据组件 0.9.5 (新增)\n", delay: 800 },
          { text: "- 安装文件...\n", delay: 600 },
          { text: "- 配置组件...\n", delay: 700 },
          { text: "- 安装完成\n\n", delay: 500 },
          
          { text: "\n更新组件完成! 当前系统版本: 1.2.0\n", delay: 800 }
        ]
      };
    }
  },
  
  // 获取升级组件信息API
  '/api/update/components': {
    POST: (params, body) => {
      console.log('Mock: 获取升级组件信息', params, body);
      return {
        json: {
          success: true,
          totalComponents: 3,
              components: [
                { name: "核心组件", version: "1.1.0" },
                { name: "界面组件", version: "1.2.0" },
                { name: "数据组件", version: "0.9.5" }
          ]
        }
      };
    }
  },
  

  
  // 升级分析API
  '/api/update/analysis': {
    GET: () => {
      return {
        json: {
          success: true,
          currentSystem: "当前系统版本: 1.0.0\n发布日期: 2023-01-15\n组件数量: 2",
          updateSystem: "升级系统版本: 1.2.0\n发布日期: 2023-06-10\n组件数量: 3",
          updateItems: "需要升级的组件:\n- 核心组件: 1.0.0 -> 1.1.0\n- 界面组件: 1.0.0 -> 1.2.0\n- 新增: 数据组件 0.9.5"
        }
      };
    }
  },
  
  // 升级备份API
  '/api/update/backup': {
    POST: () => {
      return {
        json: {
          success: true,
          message: "系统已成功备份至 /backups/system-20230710-120000.bak"
        }
      };
    }
  },
  
  // 升级更新API
  '/api/update/update': {
    POST: () => {
      return {
        json: {
          success: true,
          message: "系统组件已成功更新"
        }
      };
    }
  },
  

};

// 辅助函数：从URL中提取查询参数
function extractQueryParams(url) {
  try {
    const queryString = url.split('?')[1];
    if (!queryString) return {};
    
    const params = {};
    const pairs = queryString.split('&');
    for (const pair of pairs) {
      const [key, value] = pair.split('=');
      params[decodeURIComponent(key)] = decodeURIComponent(value || '');
    }
    return params;
  } catch (e) {
    console.error('解析URL参数出错', e);
    return {};
  }
}

// 辅助函数：检查URL是否匹配模式
function matchUrl(pattern, url) {
  // 移除查询参数部分进行匹配
  const baseUrl = url.split('?')[0];
  return baseUrl.endsWith(pattern);
}

// 模拟响应生成函数
async function createMockResponse(url, options = {}) {
  // 查找匹配的mock处理器
  const method = options.method || 'GET';
  const body = options.body ? JSON.parse(options.body) : {};
  const queryParams = extractQueryParams(url);
  
  // 查找匹配的API处理器
  let handler = null;
  let matchedPattern = null;
  
  for (const pattern in mockDataStore) {
    if (matchUrl(pattern, url) && mockDataStore[pattern][method]) {
      handler = mockDataStore[pattern][method];
      matchedPattern = pattern;
      break;
    }
  }
  
  if (!handler) {
    console.warn(`Mock服务: 未找到匹配的处理器 ${method} ${url}`);
    return new Response(JSON.stringify({ error: 'Not Found' }), { 
      status: 404,
      headers: { 'Content-Type': 'application/json' }
    });
  }
  
  console.log(`Mock服务: 匹配到 ${method} ${matchedPattern}`);
  const mockResult = handler(queryParams, body);
  
  // 处理不同类型的响应
  if (mockResult.stream) {
    // 处理流式响应
    const stream = new ReadableStream({
      start(controller) {
        let index = 0;
        
        function pushNextChunk() {
          if (index < mockResult.stream.length) {
            const chunk = mockResult.stream[index];
            const encoder = new TextEncoder();
            const uint8array = encoder.encode(chunk.text);
            controller.enqueue(uint8array);
            
            index++;
            setTimeout(pushNextChunk, chunk.delay || 500);
          } else {
            controller.close();
          }
        }
        
        // 开始推送数据
        setTimeout(pushNextChunk, 100);
      }
    });
    
    return new Response(stream, {
      headers: { 'Content-Type': 'text/plain' }
    });
  } else if (mockResult.text) {
    // 处理文本响应
    return new Response(mockResult.text, {
      headers: { 'Content-Type': 'text/plain' }
    });
  } else if (mockResult.json) {
    // 处理JSON响应
    return new Response(JSON.stringify(mockResult.json), {
      headers: { 'Content-Type': 'application/json' }
    });
  } else {
    // 默认响应
    return new Response('OK', {
      headers: { 'Content-Type': 'text/plain' }
    });
  }
}

// Mock服务的fetch拦截器
export async function mockFetch(url, options = {}, originalFetchFunction) {
  // 判断是否启用mock
  if (!mockConfig.enabled) {
    // 未启用mock，使用原始fetch函数（而不是window.fetch）
    return originalFetchFunction(url, options);
  }
  
  // 添加随机延迟，模拟网络请求
  const delay = Math.random() * 300 + 100; // 100-400ms的随机延迟
  await new Promise(resolve => setTimeout(resolve, delay));
  
  // 创建mock响应
  return createMockResponse(url, options);
}

// 创建UI开关组件
export function createMockSwitch() {
  // 创建开关容器
  const switchContainer = document.createElement('div');
  switchContainer.style.position = 'fixed';
  switchContainer.style.bottom = '20px';
  switchContainer.style.right = '20px';
  switchContainer.style.backgroundColor = 'rgba(0, 0, 0, 0.7)';
  switchContainer.style.color = 'white';
  switchContainer.style.padding = '10px';
  switchContainer.style.borderRadius = '5px';
  switchContainer.style.zIndex = '1000';
  switchContainer.style.display = 'flex';
  switchContainer.style.alignItems = 'center';
  switchContainer.style.cursor = 'pointer';
  
  // 创建开关标签
  const label = document.createElement('span');
  label.textContent = 'Mock服务: ';
  switchContainer.appendChild(label);
  
  // 创建开关状态显示
  const status = document.createElement('span');
  status.style.marginLeft = '5px';
  status.style.fontWeight = 'bold';
  switchContainer.appendChild(status);
  
  // 添加刷新按钮
  const refreshButton = document.createElement('button');
  refreshButton.textContent = '刷新页面';
  refreshButton.style.marginLeft = '10px';
  refreshButton.style.padding = '3px 8px';
  refreshButton.style.border = 'none';
  refreshButton.style.borderRadius = '3px';
  refreshButton.style.backgroundColor = '#4CAF50';
  refreshButton.style.color = 'white';
  refreshButton.style.cursor = 'pointer';
  refreshButton.style.display = 'none'; // 默认隐藏
  refreshButton.addEventListener('click', () => {
    window.location.reload();
  });
  switchContainer.appendChild(refreshButton);
  
  // 更新开关状态显示
  function updateSwitchStatus() {
    status.textContent = mockConfig.enabled ? '开启' : '关闭';
    status.style.color = mockConfig.enabled ? '#4CAF50' : '#F44336';
    
    // 在页面标题中显示状态
    document.title = document.title.replace(/ \[(MOCK|REAL)\]$/, '');
    document.title += mockConfig.enabled ? ' [MOCK]' : ' [REAL]';
  }
  
  // 初始化状态
  updateSwitchStatus();
  
  // 添加点击事件
  switchContainer.addEventListener('click', (e) => {
    // 如果点击的是刷新按钮，不处理（由按钮自己的事件处理）
    if (e.target === refreshButton) return;
    
    mockConfig.enabled = !mockConfig.enabled;
    updateSwitchStatus();
    console.log(`Mock服务已${mockConfig.enabled ? '开启' : '关闭'}`);
    
    // 显示提示和刷新按钮
    alert(`Mock服务已${mockConfig.enabled ? '开启' : '关闭'}，需要刷新页面以生效！`);
    refreshButton.style.display = 'block';
  });
  
  return switchContainer;
} 