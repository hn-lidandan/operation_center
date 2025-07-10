import axios from 'axios';

// 配置axios基础URL - 使用相对路径，通过代理访问后端
const api = axios.create({
    baseURL: '/path',
    headers: {
        'Content-Type': 'application/json'
    }
});

// 全局变量存储当前活动的标签页
window.currentTab = 'init';

// 初始化时打开安装子菜单
document.addEventListener('DOMContentLoaded', function() {
    document.getElementById('submenu-install').classList.add('open');
    document.getElementById('menu-init').classList.add('active');
    
    // 初始化配置表单中的设置
    initializeSettings();
});

// 全局变量存储当前的设置和路径
let currentSettings = {};
let currentDirPath = '';

// 初始化设置表单
function initializeSettings() {
    // 这里我们可以设置一些默认值，但实际值会由API返回
    currentSettings = {
        // 默认设置，实际会被API返回的数据覆盖
    };
}

// 切换子菜单显示/隐藏
window.toggleSubmenu = function(id) {
    const submenu = document.getElementById(`submenu-${id}`);
    if (submenu) {
        submenu.classList.toggle('open');
    }
};

// 切换不同的标签页内容
window.switchTab = function(tabId, event) {
    // 如果是子菜单项被点击，阻止事件冒泡
    if (event) {
        event.stopPropagation();
    }
    
    // 移除所有菜单的活动状态
    document.querySelectorAll('.sidebar-menu li').forEach(item => {
        item.classList.remove('active');
    });
    
    document.querySelectorAll('.submenu li').forEach(item => {
        item.classList.remove('active');
    });
    
    // 设置当前菜单的活动状态
    if (tabId === 'init' || tabId === 'execute') {
        document.getElementById('menu-install').classList.add('active');
        document.getElementById(`menu-${tabId}`).classList.add('active');
    } else {
        document.getElementById(`menu-${tabId}`).classList.add('active');
    }
    
    // 隐藏所有内容区域
    document.querySelector('.main-content').style.display = 'none';
    document.querySelector('.settings-content').style.display = 'none';
    document.querySelector('.execute-content').style.display = 'none';
    
    // 显示当前标签页对应的内容区域
    if (tabId === 'init') {
        document.querySelector('.main-content').style.display = 'flex';
        document.querySelector('.settings-content').style.display = 'flex';
    } else if (tabId === 'execute') {
        document.querySelector('.execute-content').style.display = 'flex';
    }
    
    // 更新当前标签页
    window.currentTab = tabId;
};

// 显示加载状态
function showLoading() {
    document.getElementById('loading').style.display = 'block';
}

// 隐藏加载状态
function hideLoading() {
    document.getElementById('loading').style.display = 'none';
}

// 显示结果
function showResult(text) {
    document.getElementById('result').textContent = text;
}

// 格式化 JSON 对象为易读的字符串
function formatSettings(settings) {
    let result = '';
    for (const [key, value] of Object.entries(settings)) {
        result += `${key}: ${value}\n`;
    }
    return result;
}

// 在设置编辑器中显示设置项
function displaySettings(settings) {
    currentSettings = settings;
    
    const settingsEditor = document.getElementById('settings-editor');
    settingsEditor.innerHTML = '';
    
    for (const [key, value] of Object.entries(settings)) {
        const row = document.createElement('div');
        row.className = 'settings-row';
        
        const keyElem = document.createElement('div');
        keyElem.className = 'settings-key';
        keyElem.textContent = key + ':';
        
        const valueInput = document.createElement('input');
        valueInput.className = 'settings-value';
        valueInput.value = value;
        valueInput.setAttribute('data-key', key);
        valueInput.addEventListener('change', (e) => {
            currentSettings[key] = e.target.value;
        });
        
        row.appendChild(keyElem);
        row.appendChild(valueInput);
        settingsEditor.appendChild(row);
    }
    
    // 显示设置容器
    document.querySelector('.settings-container').style.display = 'flex';
}

// 保存设置
window.saveSettings = async function() {
    if (!currentDirPath) {
        alert('请先加载安装包，获取配置信息后再保存');
        return;
    }
    
    try {
        // 调用后端保存设置的接口
        const response = await api.post('/save_settings', currentSettings, {
            params: {
                dir_path: currentDirPath
            }
        });
        alert('配置保存成功!');
    } catch (error) {
        alert('保存配置失败: ' + (error.response?.data || error.message));
        console.error('保存配置错误:', error);
    }
};

// 开始安装
window.startInstall = async function() {
    if (!currentDirPath) {
        alert('请先加载安装包，获取配置信息后再开始安装');
        return;
    }
    
    try {
        // 切换到执行安装标签页
        switchTab('execute');
        
        // 获取执行安装的容器
        const executeContainer = document.querySelector('.execute-container');
        executeContainer.innerHTML = '<div class="install-output"></div>';
        const outputDiv = document.querySelector('.install-output');
        
        // 显示开始安装信息
        outputDiv.textContent = '开始安装...\n';
        
        // 调用后端安装接口，使用fetch以支持流式响应
        const response = await fetch('/path/setup', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dir_path: currentDirPath })
        });
        
        // 创建一个读取器来读取流式响应
        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        
        // 循环读取流式响应
        while (true) {
            const { value, done } = await reader.read();
            if (done) break;
            
            // 解码并添加到输出区域
            const text = decoder.decode(value, { stream: true });
            outputDiv.textContent += text;
            
            // 自动滚动到底部
            outputDiv.scrollTop = outputDiv.scrollHeight;
        }
        
        // 安装完成
        outputDiv.textContent += '\n安装完成!';
    } catch (error) {
        const executeContainer = document.querySelector('.execute-container');
        executeContainer.innerHTML = `<div class="install-output">安装失败: ${error.message}</div>`;
        console.error('安装错误:', error);
    }
};

// 处理错误
function showError(error) {
    showResult(`错误: ${error.message || error}`);
    console.error('详细错误信息:', error);
}

// 处理文件处理请求
window.handleProcess = async function() {
    const pathInput = document.getElementById('pathInput');
    const zipPath = pathInput.value.trim();
    
    if (!zipPath) {
        alert('请输入文件路径');
        return;
    }

    showLoading();
    try {
        // 1. 调用解压接口
        const unzipResponse = await api.post('/unzip', {
            zip_path: zipPath  // 发送用户输入的文件路径
        });

        if (unzipResponse.status === 200) {
            // 2. 获取版本信息
            const dirPath = zipPath.replace(/\.zip$/, '');
            currentDirPath = dirPath; // 保存当前目录路径，用于后续保存配置
            
            const findFileResponse = await api.get('/find_info_file', {
                params: { dir_path: dirPath }
            });
            
            // 3. 获取配置信息
            const getVersionResponse = await api.get('/get_version_info', {
                params: { dir_path: dirPath }
            });
            
            showResult(getVersionResponse.data.info || '未找到版本信息');
            
            // 4. 获取默认配置信息
            const getSettingsResponse = await api.get('/get_settings', {
                params: { dir_path: dirPath }
            });
            
            displaySettings(getSettingsResponse.data);
        }
    } catch (error) {
        showError(error);
    } finally {
        hideLoading();
    }
}; 