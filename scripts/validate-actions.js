#!/usr/bin/env node

// GitHub Actions 配置验证脚本
// 检查工作流文件的语法和配置

const fs = require('fs');
const path = require('path');
const yaml = require('js-yaml');

const workflowsDir = '.github/workflows';
const requiredFiles = [
  'ci.yml',
  'build.yml', 
  'manual-build.yml',
  'release.yml'
];

console.log('🔍 验证GitHub Actions配置...\n');

// 检查工作流目录是否存在
if (!fs.existsSync(workflowsDir)) {
  console.error('❌ 工作流目录不存在:', workflowsDir);
  process.exit(1);
}

let allValid = true;

// 验证每个工作流文件
for (const file of requiredFiles) {
  const filePath = path.join(workflowsDir, file);
  
  console.log(`📄 检查 ${file}...`);
  
  if (!fs.existsSync(filePath)) {
    console.error(`  ❌ 文件不存在: ${filePath}`);
    allValid = false;
    continue;
  }
  
  try {
    const content = fs.readFileSync(filePath, 'utf8');
    const workflow = yaml.load(content);
    
    // 基本结构验证
    if (!workflow.name) {
      console.error(`  ❌ 缺少工作流名称`);
      allValid = false;
    }
    
    if (!workflow.on) {
      console.error(`  ❌ 缺少触发条件`);
      allValid = false;
    }
    
    if (!workflow.jobs) {
      console.error(`  ❌ 缺少作业定义`);
      allValid = false;
    }
    
    // 检查作业配置
    for (const [jobName, job] of Object.entries(workflow.jobs)) {
      if (!job['runs-on']) {
        console.error(`  ❌ 作业 ${jobName} 缺少 runs-on`);
        allValid = false;
      }
      
      if (!job.steps || !Array.isArray(job.steps)) {
        console.error(`  ❌ 作业 ${jobName} 缺少步骤`);
        allValid = false;
      }
    }
    
    console.log(`  ✅ ${file} 配置有效`);
    
  } catch (error) {
    console.error(`  ❌ YAML解析错误: ${error.message}`);
    allValid = false;
  }
}

// 检查package.json脚本
console.log('\n📦 检查package.json脚本...');
try {
  const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
  const requiredScripts = ['build', 'type-check', 'lint'];
  
  for (const script of requiredScripts) {
    if (!packageJson.scripts || !packageJson.scripts[script]) {
      console.error(`  ❌ 缺少脚本: ${script}`);
      allValid = false;
    } else {
      console.log(`  ✅ 脚本 ${script} 存在`);
    }
  }
} catch (error) {
  console.error(`  ❌ package.json读取错误: ${error.message}`);
  allValid = false;
}

// 检查Tauri配置
console.log('\n⚙️ 检查Tauri配置...');
try {
  const tauriConfig = JSON.parse(fs.readFileSync('src-tauri/tauri.conf.json', 'utf8'));
  
  if (!tauriConfig.productName) {
    console.error('  ❌ 缺少产品名称');
    allValid = false;
  }
  
  if (!tauriConfig.version) {
    console.error('  ❌ 缺少版本号');
    allValid = false;
  }
  
  if (!tauriConfig.bundle || !tauriConfig.bundle.active) {
    console.error('  ❌ 未启用打包');
    allValid = false;
  }
  
  console.log('  ✅ Tauri配置有效');
  
} catch (error) {
  console.error(`  ❌ Tauri配置读取错误: ${error.message}`);
  allValid = false;
}

// 总结
console.log('\n' + '='.repeat(50));
if (allValid) {
  console.log('🎉 所有配置验证通过！');
  console.log('\n📋 下一步操作:');
  console.log('1. 提交代码到GitHub仓库');
  console.log('2. 推送标签触发发布: git tag v1.0.0 && git push origin --tags');
  console.log('3. 或手动触发构建: GitHub Actions → Manual Build');
  process.exit(0);
} else {
  console.log('❌ 配置验证失败，请修复上述问题');
  process.exit(1);
}
