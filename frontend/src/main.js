import axios from 'axios';

// 引入mock服务（可以在不需要时轻松删除这一行）
import './mockInit.js';

// 配置axios基础URL - 使用相对路径，通过代理访问后端
const api = axios.create({
    baseURL: '',  // 移除 '/path' 前缀，使用相对路径
    headers: {
        'Content-Type': 'application/json'
    }
});

// 全局变量存储当前活动的标签页
window.currentTab = 'init';
// 全局变量存储当前顶部选项卡
window.currentTopTab = 'setup';
// 全局变量标记安装包是否已成功载入
window.packageLoaded = false;
// 全局变量标记配置是否已成功保存
window.configSaved = false;

// 页面加载完成后执行
document.addEventListener('DOMContentLoaded', function() {
    console.log('页面已加载');
    // 初始化顶部选项卡状态
    updateSidebarMenuByTopTab('setup');
});

// 切换顶部选项卡
function switchTopTab(tabId) {
    console.log('切换顶部选项卡到:', tabId);
    
    // 移除所有顶部选项卡的活动状态
    document.querySelectorAll('.top-tab').forEach(tab => {
        tab.classList.remove('active');
    });
    
    // 设置当前选中的顶部选项卡为活动状态
    document.getElementById('tab-' + tabId).classList.add('active');
    
    // 保存当前顶部选项卡
    window.currentTopTab = tabId;
    
    // 根据顶部选项卡更新侧边栏菜单
    updateSidebarMenuByTopTab(tabId);
    
    // 如果当前侧边栏菜单项在新的顶部选项卡下不可见，则选择第一个可见的菜单项
    const activeMenuItem = document.querySelector('.sidebar-menu li.active');
    if (!activeMenuItem || activeMenuItem.style.display === 'none') {
        const firstVisibleMenuItem = document.querySelector('.sidebar-menu li:not([style*="display: none"])');
        if (firstVisibleMenuItem) {
            switchTab(firstVisibleMenuItem.id.replace('menu-', ''));
        }
    }
}

// 根据顶部选项卡更新侧边栏菜单显示
function updateSidebarMenuByTopTab(topTabId) {
    const menuItems = document.querySelectorAll('.sidebar-menu li');
    
    if (topTabId === 'setup') {
        // 显示"安装包载入"、"配置本地化"和"执行安装"，隐藏"升级准备"和"升级"
        menuItems.forEach(item => {
            if (item.id === 'menu-init' || item.id === 'menu-config' || item.id === 'menu-execute') {
                item.style.display = 'block';
            } else {
                item.style.display = 'none';
            }
        });
        
        // 如果当前标签页是升级相关的，则切换到配置初始化
        if (window.currentTab === 'upgrade-prepare' || window.currentTab === 'upgrade') {
            switchTab('init');
        }
    } else if (topTabId === 'update') {
        // 显示"升级准备"和"升级"，隐藏"配置初始化"和"执行安装"
        menuItems.forEach(item => {
            if (item.id === 'menu-upgrade-prepare' || item.id === 'menu-upgrade') {
                item.style.display = 'block';
            } else {
                item.style.display = 'none';
            }
        });
        
        // 如果当前标签页是配置相关的，则切换到升级准备
        if (window.currentTab === 'init' || window.currentTab === 'execute') {
            switchTab('upgrade-prepare');
        }
    }
}

// 切换标签页
function switchTab(tabId) {
    console.log('切换标签页到:', tabId);
    
    // 移除所有菜单项的活动状态
    document.querySelectorAll('.sidebar-menu li').forEach(item => {
        item.classList.remove('active');
    });
    
    // 隐藏所有内容区域
    document.querySelector('.main-content').style.display = 'none';
    document.querySelector('.settings-content').style.display = 'none';
    document.querySelector('.config-content').style.display = 'none';
    document.querySelector('.execute-content').style.display = 'none';
    document.querySelector('.upgrade-prepare-content').style.display = 'none';
    document.querySelector('.upgrade-content').style.display = 'none';
    
    // 根据选择的标签页显示对应内容
    if (tabId === 'init') {
        document.querySelector('.main-content').style.display = 'flex';
        document.getElementById('menu-init').classList.add('active');
        // 确保顶部选项卡为"Setup"
        if (window.currentTopTab !== 'setup') {
            switchTopTab('setup');
        }
    } else if (tabId === 'config') {
        document.querySelector('.config-content').style.display = 'flex';
        document.getElementById('menu-config').classList.add('active');
        // 确保顶部选项卡为"Setup"
        if (window.currentTopTab !== 'setup') {
            switchTopTab('setup');
        }
        
        // 重置配置保存状态
        window.configSaved = false;
        
        // 如果配置编辑器为空，尝试加载配置
        const configEditor = document.getElementById('settings-editor');
        if (configEditor && configEditor.children.length === 0) {
            // 获取当前解压路径
            const pathInput = document.getElementById('pathInput').value;
            if (pathInput) {
                const dirPath = pathInput.endsWith('.zip') 
                    ? pathInput.substring(0, pathInput.length - 4)
                    : pathInput;
                
                // 尝试获取配置信息
                fetch(`/api/find_setting_file?dir_path=${encodeURIComponent(dirPath)}`)
                    .then(response => response.json())
                    .then(data => {
                        loadSettings(data);
                    })
                    .catch(error => {
                        console.error('获取配置信息失败:', error);
                    });
            }
        }
    } else if (tabId === 'execute') {
        document.querySelector('.execute-content').style.display = 'flex';
        document.getElementById('menu-execute').classList.add('active');
        // 确保顶部选项卡为"Setup"
        if (window.currentTopTab !== 'setup') {
            switchTopTab('setup');
        }
        
        // 确保执行安装界面的元素正确初始化
        const executeContainer = document.querySelector('.execute-container');
        if (!executeContainer.querySelector('.install-output')) {
            const installOutput = document.createElement('div');
            installOutput.className = 'install-output';
            installOutput.textContent = '请先在"配置初始化"页面载入安装包，然后点击"安装"按钮开始安装。';
            executeContainer.innerHTML = '';
            executeContainer.appendChild(installOutput);
        }
        
        // 重置进度条
        const progressBar = document.querySelector('.install-progress-bar');
        const progressText = document.querySelector('.install-progress-text');
        if (progressBar && progressText) {
            progressBar.style.width = '0%';
            progressBar.textContent = '0%';
            progressText.textContent = '';
        }
    } else if (tabId === 'upgrade-prepare') {
        document.querySelector('.upgrade-prepare-content').style.display = 'flex';
        document.getElementById('menu-upgrade-prepare').classList.add('active');
        // 确保顶部选项卡为"Update"
        if (window.currentTopTab !== 'update') {
            switchTopTab('update');
        }
    } else if (tabId === 'upgrade') {
        document.querySelector('.upgrade-content').style.display = 'flex';
        document.getElementById('menu-upgrade').classList.add('active');
        // 确保顶部选项卡为"Update"
        if (window.currentTopTab !== 'update') {
            switchTopTab('update');
        }
    }
    
    // 保存当前标签页
    window.currentTab = tabId;
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
    
    // 确保下一步按钮处于禁用状态
    document.getElementById('nextButton').disabled = true;
    
    // 重置安装包载入状态
    window.packageLoaded = false;
    
    // 重置配置保存状态
    window.configSaved = false;
    
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
            
            // 2. 调用current_version API获取版本信息
            try {
                const infoResponse = await fetch(`/api/current_version?dir_path=${encodeURIComponent(dirPath)}`);
                if (infoResponse.ok) {
                    const versionInfo = await infoResponse.text();
                    // 确保版本信息正确显示在版本信息框中
                    const resultElement = document.getElementById('result');
                    resultElement.textContent = versionInfo;
                    resultElement.style.whiteSpace = 'pre-wrap'; // 保留换行符
                    console.log('版本信息已加载:', versionInfo);
                    
                    // 启用下一步按钮
                    document.getElementById('nextButton').disabled = false;
                    
                    // 标记安装包已成功载入
                    window.packageLoaded = true;
                    console.log('安装包已成功载入，packageLoaded =', window.packageLoaded);
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
    
    const settingsTable = document.createElement('div');
    settingsTable.style.width = '100%';
    
    for (const key in settings) {
        const row = document.createElement('div');
        row.className = 'settings-row';
        row.style.display = 'flex';
        row.style.marginBottom = '15px';
        row.style.alignItems = 'center';
        
        const keyElement = document.createElement('div');
        keyElement.style.width = '250px';
        keyElement.style.fontWeight = 'bold';
        keyElement.style.paddingRight = '15px';
        keyElement.textContent = key;
        
        const valueElement = document.createElement('input');
        valueElement.style.flexGrow = '1';
        valueElement.style.padding = '8px';
        valueElement.style.border = '1px solid #ccc';
        valueElement.style.borderRadius = '4px';
        valueElement.style.fontSize = '16px';
        valueElement.value = settings[key];
        valueElement.dataset.key = key;
        
        row.appendChild(keyElement);
        row.appendChild(valueElement);
        settingsTable.appendChild(row);
    }
    
    settingsEditor.appendChild(settingsTable);
}

// 保存设置
async function saveSettings() {
    const settingsInputs = document.querySelectorAll('#settings-editor input');
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
        // 使用正确的API端点并包装settings对象
        const response = await fetch(`/api/save_settings?dir_path=${encodeURIComponent(dirPath)}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ settings: settings })  // 将settings包装在settings字段中，符合后端SaveSettingsRequest结构
        });
        
        const text = await response.text();
        
        if (response.ok) {
            alert('设置已保存');
            console.log('保存配置成功:', text);
            // 标记配置已成功保存
            window.configSaved = true;
        } else {
            alert('保存失败: ' + text);
            console.error('保存配置失败:', text);
            // 保存失败，标记配置未保存
            window.configSaved = false;
        }
    } catch (error) {
        alert('请求错误: ' + error.message);
        console.error('保存配置异常:', error);
        // 发生异常，标记配置未保存
        window.configSaved = false;
    }
}

// 开始安装
async function startInstall() {
    // 检查配置是否已保存
    if (!window.configSaved) {
        alert('请对当前配置保存');
        return;
    }
    
    // 获取DOM元素
    const executeContainer = document.querySelector('.execute-container');
    const progressBar = document.querySelector('.install-progress-bar');
    const progressText = document.querySelector('.install-progress-text');
    
    // 创建安装输出元素
    const installOutput = document.createElement('div');
    installOutput.className = 'install-output';
    installOutput.textContent = '开始安装...\n';
    
    // 重置进度条
    progressBar.style.width = '0%';
    progressBar.textContent = '0%';
    progressText.textContent = '';
    
    // 切换到执行安装标签页
    switchTab('execute');
    
    // 获取当前解压路径
    const pathInput = document.getElementById('pathInput').value;
    if (!pathInput) {
        installOutput.textContent = '错误: 请先载入安装包';
        executeContainer.innerHTML = '';
        executeContainer.appendChild(installOutput);
        progressText.textContent = '安装失败';
        return;
    }
    
    // 获取解压后的文件路径（去掉.zip扩展名）
    const dirPath = pathInput.endsWith('.zip') 
        ? pathInput.substring(0, pathInput.length - 4)  // 去掉.zip扩展名
        : pathInput;
    
    try {
        // 1. 先获取组件信息
        const componentsResponse = await fetch('/api/setup/components', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dir_path: dirPath })
        });
        
        if (!componentsResponse.ok) {
            throw new Error('获取组件信息失败');
        }
        
        const componentsData = await componentsResponse.json();
        const totalComponents = componentsData.totalComponents || 0;
        console.log(`获取到组件数量: ${totalComponents}`);
        
        // 清空容器并添加输出元素
        executeContainer.innerHTML = '';
        executeContainer.appendChild(installOutput);
        
        // 2. 调用setup API执行安装
        const response = await fetch('/api/setup', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dir_path: dirPath })
        });
        
        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        
        // 用于跟踪安装进度的变量
        let currentComponent = 0;
        
        while (true) {
            const { value, done } = await reader.read();
            if (done) break;
            
            const text = decoder.decode(value, { stream: true });
            installOutput.textContent += text;
            installOutput.scrollTop = installOutput.scrollHeight; // 自动滚动到底部
            
            // 检查是否包含组件标记
            if (text.includes('组件:')) {
                currentComponent++;
                const progress = Math.min(Math.round((currentComponent / totalComponents) * 100), 100);
                progressBar.style.width = progress + '%';
                progressBar.textContent = progress + '%';
                
                // 提取组件名称用于显示
                const componentMatch = text.match(/组件:\s*([^\n]+)/);
                if (componentMatch) {
                    progressText.textContent = `正在安装组件 (${currentComponent}/${totalComponents}): ${componentMatch[1].trim()}`;
                } else {
                    progressText.textContent = `正在安装组件 (${currentComponent}/${totalComponents})`;
                }
            }
            
            // 检查是否安装完成
            if (text.includes('安装完成!')) {
                progressBar.style.width = '100%';
                progressBar.textContent = '100%';
                progressText.textContent = '安装完成!';
            }
        }
        
        // 安装完成，确保进度条为100%
        progressBar.style.width = '100%';
        progressBar.textContent = '100%';
        progressText.textContent = '安装完成!';
        
    } catch (error) {
        installOutput.textContent += '\n安装过程出错: ' + error.message;
        
        // 错误时设置进度条为红色
        progressBar.style.backgroundColor = '#f44336';
        progressText.textContent = '安装失败: ' + error.message;
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
        // 调用正确的unzip API
        const response = await fetch('/api/unzip', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ zip_path: pathInput })
        });
        
        const text = await response.text();
        
        if (response.ok) {
            alert('升级包已成功载入');
            console.log('解压结果:', text);
            
            // 可以在这里添加额外的逻辑，例如加载解压后的版本信息
            const dirPath = pathInput.endsWith('.zip') 
                ? pathInput.substring(0, pathInput.length - 4)  // 去掉.zip扩展名
                : pathInput;
            
            // 存储当前升级包路径，供后续使用
            window.currentUpgradePath = dirPath;
            console.log('设置当前升级包路径:', window.currentUpgradePath);
            
            // 更新当前版本信息区域
            document.getElementById('current-version').innerHTML = '<div>正在加载版本信息...</div>';
            
            // 1. 加载当前系统版本信息
            try {
                const infoResponse = await fetch(`/api/current_version?dir_path=${encodeURIComponent(dirPath)}`);
                if (infoResponse.ok) {
                    const versionInfo = await infoResponse.text();
                    document.getElementById('current-version').innerHTML = 
                        `<div style="white-space: pre-wrap;">${versionInfo}</div>`;
                } else {
                    const errorText = await infoResponse.text();
                    document.getElementById('current-version').innerHTML = 
                        `<div>获取版本信息失败: ${errorText}</div>`;
                }
            } catch (error) {
                document.getElementById('current-version').innerHTML = 
                    `<div>获取版本信息出错: ${error.message}</div>`;
            }
            
            // 2. 加载历史版本信息
            document.getElementById('history-versions').innerHTML = '<div>正在加载历史版本...</div>';
            try {
                const historyResponse = await fetch('/api/history_version', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ dir_path: dirPath })
                });
                
                if (historyResponse.ok) {
                    const historyInfo = await historyResponse.text();
                    
                    // 显示历史版本信息
                    const historyVersionsElement = document.getElementById('history-versions');
                    historyVersionsElement.innerHTML = `<div style="white-space: pre-wrap;">${historyInfo}</div>`;
                } else {
                    const errorText = await historyResponse.text();
                    document.getElementById('history-versions').innerHTML = 
                        `<div>获取历史版本失败: ${errorText}</div>`;
                }
            } catch (error) {
                document.getElementById('history-versions').innerHTML = 
                    `<div>获取历史版本出错: ${error.message}</div>`;
            }
            
            // 3. 加载本地设置
            const upgradeSettingsEditor = document.getElementById('upgrade-settings-editor');
            upgradeSettingsEditor.innerHTML = '<div>正在加载本地设置...</div>';
            try {
                // 将POST请求改为GET请求，使用查询参数
                const settingsResponse = await fetch(`/api/find_setting_file?dir_path=${encodeURIComponent(dirPath)}`);
                
                if (settingsResponse.ok) {
                    const settingsData = await settingsResponse.json();
                    
                    // 使用与配置初始化页面相同的方式显示可编辑的设置
                    loadUpgradeSettings(settingsData);
                    
                    // 存储当前路径，用于保存配置时使用
                    window.currentUpgradePath = dirPath;
                } else {
                    const errorText = await settingsResponse.text();
                    upgradeSettingsEditor.innerHTML = `<div>获取本地设置失败: ${errorText}</div>`;
                }
            } catch (error) {
                upgradeSettingsEditor.innerHTML = `<div>获取本地设置出错: ${error.message}</div>`;
            }
        } else {
            alert('载入失败: ' + text);
        }
    } catch (error) {
        alert('请求错误: ' + error.message);
    }
}

// 启动升级流程
function startUpgrade() {
    // 获取当前升级包路径
    const pathInput = document.getElementById('upgradePathInput').value;
    if (!pathInput) {
        alert('请先载入升级包');
        return;
    }
    
    // 确保window.currentUpgradePath已设置
    if (!window.currentUpgradePath) {
        const dirPath = pathInput.endsWith('.zip') 
            ? pathInput.substring(0, pathInput.length - 4)  // 去掉.zip扩展名
            : pathInput;
        window.currentUpgradePath = dirPath;
        console.log('启动升级时设置路径:', window.currentUpgradePath);
    }
    
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
        const response = await fetch('/api/update/analysis');
        const data = await response.json();
        
        if (data.success) {
            // 显示当前系统信息
            document.getElementById('current-system-info').textContent = data.currentSystem || '无法获取当前系统信息';
            
            // 显示升级系统信息
            document.getElementById('upgrade-system-info').textContent = data.updateSystem || '无法获取升级系统信息';
            
            // 显示升级项
            document.getElementById('upgrade-items').textContent = data.updateItems || '无升级项';
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
        
        // 激活下一步骤
        document.getElementById(nextStepId).classList.add('active');
        
        // 显示下一步骤内容
        document.getElementById(nextContentId).classList.add('active');
        
        // 执行备份操作
        await performBackup();
    } else if (activeStepId === 'step-backup') {
        nextStepId = 'step-update';
        nextContentId = 'content-update';
        
        // 激活下一步骤
        document.getElementById(nextStepId).classList.add('active');
        
        // 显示下一步骤内容
        document.getElementById(nextContentId).classList.add('active');
        
        // 初始化更新进度显示
        const updateProgress = document.getElementById('update-progress');
        updateProgress.textContent = '更新中...';
        
        // 执行更新操作
        await performUpdate();
    }
    // 移除对step-update后续步骤的处理
}

// 执行备份
async function performBackup() {
    const backupProgress = document.getElementById('backup-progress');
    backupProgress.textContent = '备份中...';
    
    try {
        const response = await fetch('/api/update/backup', {
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
    const progressBar = document.getElementById('update-progress-bar');
    const progressText = document.getElementById('update-progress-text');
    
    // 重置进度条
    progressBar.style.width = '0%';
    progressBar.textContent = '0%';
    progressText.textContent = '';
    
    // 获取当前升级包路径
    const dirPath = window.currentUpgradePath;
    console.log('执行更新使用路径:', dirPath);
    
    if (!dirPath) {
        updateProgress.textContent = '错误: 无法获取升级包路径';
        progressText.textContent = '更新失败: 无法获取升级包路径';
        progressBar.style.backgroundColor = '#f44336';
        return;
    }
    
    try {
        // 1. 先获取组件信息
        const componentsResponse = await fetch('/api/update/components', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dir_path: dirPath })
        });
        
        if (!componentsResponse.ok) {
            throw new Error('获取组件信息失败');
        }
        
        const componentsData = await componentsResponse.json();
        const totalComponents = componentsData.totalComponents || 0;
        console.log(`获取到升级组件数量: ${totalComponents}`);
        
        // 清空之前的内容
        updateProgress.textContent = '';
        
        // 2. 调用update API执行更新
        console.log('开始调用更新API, 参数:', { dir_path: dirPath });
        const response = await fetch('/api/update', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dir_path: dirPath })
        });
        
        console.log('更新API响应状态:', response.status);
        
        // 使用流式处理响应
        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        
        // 用于跟踪更新进度的变量
        let currentComponent = 0;
        
        while (true) {
            const { value, done } = await reader.read();
            if (done) {
                console.log('更新流式响应结束');
                break;
            }
            
            const text = decoder.decode(value, { stream: true });
            console.log('收到更新流数据:', text);
            updateProgress.textContent += text;
            updateProgress.scrollTop = updateProgress.scrollHeight; // 自动滚动到底部
            
            // 检查是否包含组件标记
            if (text.includes('组件:')) {
                currentComponent++;
                const progress = Math.min(Math.round((currentComponent / totalComponents) * 100), 100);
                progressBar.style.width = progress + '%';
                progressBar.textContent = progress + '%';
                
                // 提取组件名称用于显示
                const componentMatch = text.match(/组件:\s*([^\n]+)/);
                if (componentMatch) {
                    progressText.textContent = `正在更新组件 (${currentComponent}/${totalComponents}): ${componentMatch[1].trim()}`;
                } else {
                    progressText.textContent = `正在更新组件 (${currentComponent}/${totalComponents})`;
                }
            }
            
            // 检查是否更新完成
                if (text.includes('更新组件完成') || text.includes('验证通过')) {
                progressBar.style.width = '100%';
                progressBar.textContent = '100%';
                    progressText.textContent = '更新完成!';
            }
        }
        
        // 更新完成后，检查是否成功
        if (updateProgress.textContent.includes('更新组件完成') || 
            updateProgress.textContent.includes('更新成功') || 
            updateProgress.textContent.includes('验证通过')) {
            
            console.log('更新成功，将所有椭圆变绿');
            
            // 将所有步骤椭圆标记为已完成（绿色）
            document.getElementById('step-analysis').classList.add('completed');
            document.getElementById('step-backup').classList.add('completed');
            document.getElementById('step-update').classList.add('completed');
            document.getElementById('step-finish').classList.add('completed');
            
            // 隐藏更新页面的下一步按钮
            const updateNextButton = document.querySelector('#content-update .next-button');
            if (updateNextButton) {
                updateNextButton.style.display = 'none';
            }
            
            // 确保进度条显示100%
            progressBar.style.width = '100%';
            progressBar.textContent = '100%';
            progressText.textContent = '更新完成!';
        }
    } catch (error) {
        console.error('更新过程出错:', error);
        updateProgress.textContent += '\n更新过程出错: ' + error.message;
        
        // 错误时设置进度条为红色
        progressBar.style.backgroundColor = '#f44336';
        progressText.textContent = '更新失败: ' + error.message;
    }
}



// 加载升级设置
function loadUpgradeSettings(settings) {
    const settingsEditor = document.getElementById('upgrade-settings-editor');
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

// 保存升级设置
async function saveUpgradeSettings() {
    const settingsInputs = document.querySelectorAll('#upgrade-settings-editor .settings-value');
    const settings = {};
    
    settingsInputs.forEach(input => {
        settings[input.dataset.key] = input.value;
    });
    
    // 获取当前升级包路径
    const dirPath = window.currentUpgradePath;
    if (!dirPath) {
        alert('请先载入升级包');
        return;
    }
    
    try {
        const response = await fetch(`/api/save_settings?dir_path=${encodeURIComponent(dirPath)}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ settings: settings })  // 将settings包装在settings字段中，符合后端SaveSettingsRequest结构
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

// 添加新函数：跳转到配置本地化界面
function goToConfig() {
    console.log('检查安装包载入状态:', window.packageLoaded);
    
    // 检查安装包是否已成功载入
    if (!window.packageLoaded) {
        alert('请先在"安装包载入"页面载入安装包');
        return;
    }
    
    // 切换到配置本地化标签页
    switchTab('config');
}

// 添加新函数：跳转到执行安装界面
function goToExecute() {
    console.log('检查安装包载入状态:', window.packageLoaded);
    
    // 检查安装包是否已成功载入
    if (!window.packageLoaded) {
        alert('请先在"安装包载入"页面载入安装包');
        switchTab('init'); // 切换到安装包载入页面
        return;
    }
    
    // 切换到执行安装标签页
    switchTab('execute');
}

// 导出函数到全局作用域，以便HTML中调用
window.switchTab = switchTab;
window.handleProcess = handleProcess;
window.saveSettings = saveSettings;
window.startInstall = startInstall;
window.loadUpgradePackage = loadUpgradePackage;
window.startUpgrade = startUpgrade;
window.nextUpgradeStep = nextUpgradeStep;
window.saveUpgradeSettings = saveUpgradeSettings; // 导出保存升级设置函数
window.switchTopTab = switchTopTab; // 导出顶部选项卡切换函数
window.goToConfig = goToConfig; // 导出跳转到配置本地化界面的函数
window.goToExecute = goToExecute; // 导出跳转到执行安装界面的函数