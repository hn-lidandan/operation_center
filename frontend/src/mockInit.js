// mockInit.js - Mock服务初始化
// 这个文件可以在不需要mock时轻松删除

import { mockFetch, createMockSwitch, mockConfig } from './mockService.js';

// 拦截全局fetch
const originalFetch = window.fetch;
window.fetch = function(url, options) {
  return mockFetch(url, options);
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