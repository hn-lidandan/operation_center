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

// 页面加载完成后执行
document.addEventListener('DOMContentLoaded', function() {
    console.log('页面已加载');
});

// 切换标签页
function switchTab(tabId) {
    // 移除所有菜单项的活动状态
    document.querySelectorAll('.sidebar-menu li').forEach(item => {
        item.classList.remove('active');
    });
    
    // 隐藏所有内容区域
    document.querySelector('.main-content').style.display = 'none';
    document.querySelector('.settings-content').style.display = 'none';
    document.querySelector('.execute-content').style.display = 'none';
    document.querySelector('.upgrade-prepare-content').style.display = 'none';
    document.querySelector('.upgrade-content').style.display = 'none';
    
    // 根据选择的标签页显示对应内容
    if (tabId === 'init') {
        document.querySelector('.main-content').style.display = 'flex';
        document.querySelector('.settings-content').style.display = 'flex';
        document.getElementById('menu-init').classList.add('active');
    } else if (tabId === 'execute') {
        document.querySelector('.execute-content').style.display = 'flex';
        document.getElementById('menu-execute').classList.add('active');
    } else if (tabId === 'upgrade-prepare') {
        document.querySelector('.upgrade-prepare-content').style.display = 'flex';
        document.getElementById('menu-upgrade-prepare').classList.add('active');
        // 加载历史版本信息
        loadHistoryVersions();
    } else if (tabId === 'upgrade') {
        document.querySelector('.upgrade-content').style.display = 'flex';
        document.getElementById('menu-upgrade').classList.add('active');
    }
}

// 处理安装包路径
async function handleProcess() {
    const pathInput = document.getElementById('pathInput').value;
    if (!pathInput) {
        alert('请输入安装包路径');
        return;
    }
    
    document.getElementById('loading').style.display = 'block';
    document.getElementById('result').textContent = '处理中...';
    
    try {
        // 1. 调用解压API
        const unzipResponse = await fetch('/api/unzip', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ zip_path: pathInput })
        });
        
        const unzipText = await unzipResponse.text();
        console.log('解压结果:', unzipText);
        
        // 检查是否解压成功
        if (unzipResponse.ok && unzipText.includes('解压成功')) {
            // 修正：获取解压后的文件路径（去掉.zip扩展名）
            const dirPath = pathInput.endsWith('.zip') 
                ? pathInput.substring(0, pathInput.length - 4)  // 去掉.zip扩展名
                : pathInput;
            
            console.log('使用的目录路径:', dirPath);
            
            // 2. 调用find_info_file API获取版本信息
            try {
                const infoResponse = await fetch(`/api/find_info_file?dir_path=${encodeURIComponent(dirPath)}`);
                if (infoResponse.ok) {
                    const versionInfo = await infoResponse.text();
                    // 确保版本信息正确显示在版本信息框中
                    const resultElement = document.getElementById('result');
                    resultElement.textContent = versionInfo;
                    resultElement.style.whiteSpace = 'pre-wrap'; // 保留换行符
                    console.log('版本信息已加载:', versionInfo);
                } else {
                    const errorText = await infoResponse.text();
                    document.getElementById('result').textContent = `获取版本信息失败: ${errorText}`;
                    console.error('获取版本信息失败:', errorText);
                }
            } catch (error) {
                document.getElementById('result').textContent = `获取版本信息出错: ${error.message}`;
                console.error('获取版本信息异常:', error);
            }
            
            // 3. 调用find_setting_file API获取配置信息
            try {
                // 注意这里的路由，根据后端定义修正
                const settingsResponse = await fetch(`/api/find_setting_file?dir_path=${encodeURIComponent(dirPath)}`);
                if (settingsResponse.ok) {
                    const settingsData = await settingsResponse.json();
                    loadSettings(settingsData);
                } else {
                    const errorText = await settingsResponse.text();
                    console.error(`获取配置信息失败: ${errorText}`);
                }
            } catch (error) {
                console.error(`获取配置信息出错: ${error.message}`);
            }
        } else {
            // 解压失败，显示错误信息
            document.getElementById('result').textContent = unzipText;
        }
    } catch (error) {
        document.getElementById('result').textContent = '请求错误: ' + error.message;
    } finally {
        document.getElementById('loading').style.display = 'none';
    }
}

// 加载设置
function loadSettings(settings) {
    const settingsEditor = document.getElementById('settings-editor');
    settingsEditor.innerHTML = '';
    
    if (!settings || Object.keys(settings).length === 0) {
        settingsEditor.textContent = '无可用设置';
        return;
    }
    
    for (const key in settings) {
        const row = document.createElement('div');
        row.className = 'settings-row';
        
        const keyElement = document.createElement('div');
        keyElement.className = 'settings-key';
        keyElement.textContent = key;
        
        const valueElement = document.createElement('input');
        valueElement.className = 'settings-value';
        valueElement.value = settings[key];
        valueElement.dataset.key = key;
        
        row.appendChild(keyElement);
        row.appendChild(valueElement);
        settingsEditor.appendChild(row);
    }
}

// 保存设置
async function saveSettings() {
    const settingsInputs = document.querySelectorAll('.settings-value');
    const settings = {};
    
    settingsInputs.forEach(input => {
        settings[input.dataset.key] = input.value;
    });
    
    // 获取当前解压路径
    const pathInput = document.getElementById('pathInput').value;
    if (!pathInput) {
        alert('请先载入安装包');
        return;
    }
    
    // 获取解压后的文件路径（去掉.zip扩展名）
    const dirPath = pathInput.endsWith('.zip') 
        ? pathInput.substring(0, pathInput.length - 4)  // 去掉.zip扩展名
        : pathInput;
    
    try {
        const response = await fetch(`/api/save_settings?dir_path=${encodeURIComponent(dirPath)}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(settings)
        });
        
        const text = await response.text();
        
        if (response.ok) {
            alert('设置已保存');
            console.log('保存配置成功:', text);
        } else {
            alert('保存失败: ' + text);
            console.error('保存配置失败:', text);
        }
    } catch (error) {
        alert('请求错误: ' + error.message);
        console.error('保存配置异常:', error);
    }
}

// 开始安装
async function startInstall() {
    const executeContainer = document.querySelector('.execute-container');
    executeContainer.innerHTML = '<div class="install-output">开始安装...</div>';
    
    // 切换到执行安装标签页
    switchTab('execute');
    
    try {
        const response = await fetch('/api/install', {
            method: 'POST'
        });
        
        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        const output = document.querySelector('.install-output');
        
        while (true) {
            const { value, done } = await reader.read();
            if (done) break;
            
            const text = decoder.decode(value, { stream: true });
            output.textContent += text;
            output.scrollTop = output.scrollHeight; // 自动滚动到底部
        }
    } catch (error) {
        const output = document.querySelector('.install-output');
        output.textContent += '\n安装过程出错: ' + error.message;
    }
}

// 加载升级包
async function loadUpgradePackage() {
    const pathInput = document.getElementById('upgradePathInput').value;
    if (!pathInput) {
        alert('请输入升级包路径');
        return;
    }
    
    try {
        const response = await fetch('/api/upgrade/load', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ path: pathInput })
        });
        
        const data = await response.json();
        
        if (data.success) {
            alert('升级包已成功载入');
        } else {
            alert('载入失败: ' + (data.error || '未知错误'));
        }
    } catch (error) {
        alert('请求错误: ' + error.message);
    }
}

// 启动升级流程
function startUpgrade() {
    // 切换到升级标签页
    switchTab('upgrade');
    
    // 初始化升级流程
    initUpgradeProcess();
}

// 初始化升级流程
async function initUpgradeProcess() {
    // 重置所有步骤状态
    document.querySelectorAll('.upgrade-step').forEach(step => {
        step.classList.remove('active', 'completed');
    });
    
    // 设置第一步为活动状态
    document.getElementById('step-analysis').classList.add('active');
    
    // 隐藏所有步骤内容
    document.querySelectorAll('.upgrade-step-content').forEach(content => {
        content.classList.remove('active');
    });
    
    // 显示第一步内容
    document.getElementById('content-analysis').classList.add('active');
    
    // 加载当前系统信息
    try {
        const response = await fetch('/api/upgrade/analysis');
        const data = await response.json();
        
        if (data.success) {
            // 显示当前系统信息
            document.getElementById('current-system-info').textContent = data.currentSystem || '无法获取当前系统信息';
            
            // 显示升级系统信息
            document.getElementById('upgrade-system-info').textContent = data.upgradeSystem || '无法获取升级系统信息';
            
            // 显示升级项
            document.getElementById('upgrade-items').textContent = data.upgradeItems || '无升级项';
        } else {
            document.getElementById('current-system-info').textContent = '获取信息失败: ' + (data.error || '未知错误');
        }
    } catch (error) {
        document.getElementById('current-system-info').textContent = '请求错误: ' + error.message;
    }
}

// 切换到下一个升级步骤
async function nextUpgradeStep() {
    // 获取当前活动步骤
    const activeStep = document.querySelector('.upgrade-step.active');
    const activeStepId = activeStep.id;
    
    // 将当前步骤标记为已完成
    activeStep.classList.remove('active');
    activeStep.classList.add('completed');
    
    // 隐藏当前步骤内容
    document.querySelector('.upgrade-step-content.active').classList.remove('active');
    
    // 确定下一步骤
    let nextStepId, nextContentId;
    
    if (activeStepId === 'step-analysis') {
        nextStepId = 'step-backup';
        nextContentId = 'content-backup';
        await performBackup();
    } else if (activeStepId === 'step-backup') {
        nextStepId = 'step-update';
        nextContentId = 'content-update';
        await performUpdate();
    } else if (activeStepId === 'step-update') {
        nextStepId = 'step-finish';
        nextContentId = 'content-finish';
        await completeUpgrade();
    }
    
    // 激活下一步骤
    document.getElementById(nextStepId).classList.add('active');
    
    // 显示下一步骤内容
    document.getElementById(nextContentId).classList.add('active');
}

// 执行备份
async function performBackup() {
    const backupProgress = document.getElementById('backup-progress');
    backupProgress.textContent = '备份中...';
    
    try {
        const response = await fetch('/api/upgrade/backup', {
            method: 'POST'
        });
        
        const data = await response.json();
        
        if (data.success) {
            backupProgress.textContent = '备份完成: ' + (data.message || '');
        } else {
            backupProgress.textContent = '备份失败: ' + (data.error || '未知错误');
        }
    } catch (error) {
        backupProgress.textContent = '备份过程出错: ' + error.message;
    }
}

// 执行更新
async function performUpdate() {
    const updateProgress = document.getElementById('update-progress');
    updateProgress.textContent = '更新中...';
    
    try {
        const response = await fetch('/api/upgrade/update', {
            method: 'POST'
        });
        
        const data = await response.json();
        
        if (data.success) {
            updateProgress.textContent = '更新完成: ' + (data.message || '');
        } else {
            updateProgress.textContent = '更新失败: ' + (data.error || '未知错误');
        }
    } catch (error) {
        updateProgress.textContent = '更新过程出错: ' + error.message;
    }
}

// 完成升级
async function completeUpgrade() {
    const upgradeResult = document.getElementById('upgrade-result');
    upgradeResult.textContent = '正在完成升级...';
    
    try {
        const response = await fetch('/api/upgrade/complete', {
            method: 'POST'
        });
        
        const data = await response.json();
        
        if (data.success) {
            upgradeResult.textContent = '升级成功: ' + (data.message || '');
        } else {
            upgradeResult.textContent = '升级失败: ' + (data.error || '未知错误');
        }
    } catch (error) {
        upgradeResult.textContent = '升级过程出错: ' + error.message;
    }
}

// 结束升级流程
function finishUpgrade() {
    // 切换回配置初始化标签页
    switchTab('init');
    alert('升级已完成');
}

// 加载历史版本信息
async function loadHistoryVersions() {
    const historyVersionsElement = document.getElementById('history-versions');
    historyVersionsElement.innerHTML = '加载历史版本中...';
    
    try {
        const response = await fetch('/versions/history');
        const data = await response.json();
        
        if (data.success && data.versions && data.versions.length > 0) {
            // 清空容器
            historyVersionsElement.innerHTML = '';
            
            // 创建版本列表
            const versionList = document.createElement('ul');
            versionList.style.listStyleType = 'none';
            versionList.style.padding = '0';
            versionList.style.margin = '0';
            
            // 添加每个版本
            data.versions.forEach(version => {
                const versionItem = document.createElement('li');
                versionItem.style.padding = '8px 0';
                versionItem.style.borderBottom = '1px solid #eee';
                versionItem.style.cursor = 'pointer';
                versionItem.textContent = version.name || '未命名版本';
                
                // 添加点击事件以显示版本详情
                versionItem.addEventListener('click', () => {
                    showVersionDetails(version);
                });
                
                versionList.appendChild(versionItem);
            });
            
            historyVersionsElement.appendChild(versionList);
        } else {
            historyVersionsElement.textContent = '没有可用的历史版本';
        }
    } catch (error) {
        historyVersionsElement.textContent = '加载历史版本失败: ' + error.message;
    }
}

// 显示版本详情
function showVersionDetails(version) {
    // 在当前系统版本区域显示选中的历史版本详情
    const currentVersionElement = document.getElementById('current-version');
    
    // 创建版本详情内容
    let detailsContent = '';
    
    if (version.name) {
        detailsContent += `<div><strong>版本名称:</strong> ${version.name}</div>`;
    }
    
    if (version.date) {
        detailsContent += `<div><strong>发布日期:</strong> ${version.date}</div>`;
    }
    
    if (version.description) {
        detailsContent += `<div><strong>描述:</strong> ${version.description}</div>`;
    }
    
    if (version.components && version.components.length > 0) {
        detailsContent += `<div><strong>组件:</strong></div><ul>`;
        version.components.forEach(component => {
            detailsContent += `<li>${component.name || '未命名组件'}: ${component.version || '无版本信息'}</li>`;
        });
        detailsContent += `</ul>`;
    }
    
    currentVersionElement.innerHTML = detailsContent || '无详细信息';
}

// 导出函数到全局作用域，以便HTML中调用
window.switchTab = switchTab;
window.handleProcess = handleProcess;
window.saveSettings = saveSettings;
window.startInstall = startInstall;
window.loadUpgradePackage = loadUpgradePackage;
window.startUpgrade = startUpgrade;
window.nextUpgradeStep = nextUpgradeStep;
window.finishUpgrade = finishUpgrade;
window.loadHistoryVersions = loadHistoryVersions; // 导出历史版本加载函数