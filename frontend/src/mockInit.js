// mockInit.js - Mock服务初始化
// 这个文件可以在不需要mock时轻松删除

import { mockFetch, createMockSwitch, mockConfig } from './mockService.js';

// 保存原始fetch函数
const originalFetch = window.fetch;

// 拦截全局fetch
window.fetch = function(url, options) {
  return mockFetch(url, options, originalFetch);
};

// 在页面加载完成后添加开关UI
document.addEventListener('DOMContentLoaded', function() {
  // 添加mock开关到页面
  const mockSwitch = createMockSwitch();
  document.body.appendChild(mockSwitch);
  
  // 在控制台中显示当前状态
  console.log(`Mock服务状态: ${mockConfig.enabled ? '开启' : '关闭'}`);
});

// 导出原始fetch，以便在需要时使用
export const originalFetchFunction = originalFetch; 