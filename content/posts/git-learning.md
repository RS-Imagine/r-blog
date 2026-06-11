+++
title = "Git 学习手册"
date = "2026-06-11"
updated = "2026-06-11"
slug = "git-learning"
description = "Git 是先进的分布式版本控制系统，掌握 Git 是必备技能。"
draft = false
+++

## 📖 第一章：Git 核心概念

在学习命令之前，必须理解 Git 的**四个区域**和**三种状态**：

### 1. 四个工作区域
* **工作区 (Workspace)**：在电脑里能看到的目录，平时写代码的地方。
* **暂存区 (Index / Stage)**：临时存放改动的地方，事实上它只是一个文件，保存即将提交的文件列表信息。
* **本地仓库 (Repository)**：安全存放数据的位置，这里面有提交的所有版本的数据。
* **远程仓库 (Remote)**：托管在网络上的项目仓库（如 GitHub, GitLab, Gitee）。

**数据流向**：工作区 ➔ `git add` ➔ 暂存区 ➔ `git commit` ➔ 本地仓库 ➔ `git push` ➔ 远程仓库

### 2. 三种文件状态
* **已修改 (Modified)**：修改了文件，但还没保存到暂存区。
* **已暂存 (Staged)**：把已修改的文件放在了下次提交时要保存的清单中。
* **已提交 (Committed)**：数据已经安全的保存在本地数据库中。

---

## ⚙️ 第二章：安装与基础配置

### 1. 安装
* **Windows**: 下载并安装 Git for Windows。
* **macOS**: 终端输入 `git --version`，如果没有安装会自动提示安装（或使用 Homebrew: `brew install git`）。
* **Linux**: `sudo apt install git` (Ubuntu/Debian) 或 `sudo yum install git` (CentOS)。

### 2. 初始化全局配置
安装完成后，必须配置用户名和邮箱（这是每次提交的身份标识）：

```bash
# 设置用户名和邮箱
git config --global user.name "名字"
git config --global user.email "邮箱@example.com"

# 【推荐】将默认初始化分支名称从 master 修改为 main（符合现代开源社区规范）
git config --global init.defaultBranch main

# 查看所有配置
git config --list
```

---

## 🌱 第三章：日常基础操作

### 1. 创建仓库
```bash
# 把当前目录变成 Git 可以管理的仓库
git init

# 或者克隆一个现有的远程仓库
git clone <远程仓库URL>
```

### 2. 查看状态
```bash
# 查看工作区和暂存区的状态（最常用的命令！）
git status
```

### 3. 添加与提交
```bash
# 将指定文件添加到暂存区
git add <文件名>

# 将所有修改和新建的文件添加到暂存区（最常用）
git add .

# 将暂存区的内容提交到本地仓库，并附带说明信息
git commit -m "feat: 新增了登录功能"

# 跳过暂存区，直接将已跟踪的修改过的文件提交（不包含新建的文件）
git commit -a -m "update message"
```

### 4. 查看提交历史
```bash
# 查看完整的提交历史
git log

# 查看精简版的提交历史（一行显示）
git log --oneline

# 图形化查看分支合并历史
git log --graph --oneline --all
```

---

## 🔀 第四章：分支管理

Git 的杀手级功能就是极其轻量级的分支。
*(注：Git 2.23 版本后，引入了专门处理分支的 `switch` 命令，推荐使用它来代替容易混淆的 `checkout`)*

### 1. 查看与创建分支
```bash
# 查看本地所有分支（当前分支前面会有 * 号）
git branch

# 查看所有本地和远程分支
git branch -a

# 创建新分支
git branch <分支名>
```

### 2. 切换分支 (新版推荐 `switch`)
```bash
# 切换到已存在的分支
git switch <分支名>

# 创建并立刻切换到新分支（代替旧版的 git checkout -b）
git switch -c <分支名>
```

### 3. 合并分支
```bash
# 切换回主分支
git switch main

# 将指定分支的代码合并到当前分支
git merge <分支名>
```

### 4. 删除分支
```bash
# 删除已经合并过的分支
git branch -d <分支名>

# 强制删除未合并的分支
git branch -D <分支名>
```

---

## ⏪ 第五章：撤销与回退

*(注：Git 2.23 引入了 `restore` 命令，专门用于撤销工作区和暂存区的修改)*

### 1. 撤销未提交的修改
```bash
# 撤销工作区的修改（让文件回到最近一次 commit 或 add 的状态）
git restore <文件名>

# 把已经 add 到暂存区的文件退回到工作区（不丢失修改内容）
git restore --staged <文件名>
```

### 2. 版本回退 (Reset)
```bash
# 退回到上一个版本（保留工作区改动，撤销 commit 和 add）
git reset HEAD^

# 彻底回退到指定版本（丢弃该版本之后的所有改动，⚠️ 危险操作！）
git reset --hard <Commit_ID>
```

### 3. 撤销某次提交 (Revert)
```bash
# 生成一个新的提交，用来抵消某次历史提交的修改（适用于推送到远程后的撤销）
git revert <Commit_ID>
```

---

## 🌐 第六章：远程协作

### 1. 关联与查看远程仓库
```bash
# 查看已配置的远程仓库
git remote -v

# 添加远程仓库（一般将其命名为 origin）
git remote add origin <远程仓库URL>
```

### 2. 拉取代码
```bash
# 抓取远程最新代码并自动合并到本地当前分支
git pull origin <远程分支名>

# 仅抓取远程最新代码，不自动合并（更安全）
git fetch origin
```

### 3. 推送代码
```bash
# 将本地分支推送到远程仓库
git push origin <本地分支名>

# 第一次推送时加上 -u 参数，将本地和远程分支关联，以后只需输入 git push
git push -u origin main
```

---

## 🛠️ 第七章：进阶高级技巧

### 1. 代码暂存 (Stash)
当正在开发一个功能，突然需要切换分支修 Bug，但不想提交尚未完成的代码时使用。
```bash
# 将当前未提交的修改暂存起来，工作区变得干净
git stash

# 查看所有暂存记录
git stash list

# 恢复最近一次的暂存，并将其从暂存列表中删除
git stash pop
```

### 2. 挑选提交 (Cherry-Pick)
只需要某个分支的某一个特定 commit，而不是整个分支合并过来。
```bash
# 将指定的 commit 应用到当前分支
git cherry-pick <Commit_ID>
```

### 3. 变基合并 (Rebase)
保持提交历史的线性整洁，替代繁杂的 Merge 节点。
```bash
# 在当前分支上，把主分支的最新更新变基过来
git rebase main
```

### 4. 忽略文件 (.gitignore)
在项目根目录下创建一个名为 `.gitignore` 的文件，Git 会自动忽略匹配的文件/文件夹，不把它们纳入版本控制（如日志、编译产生的文件、数据库密码配置等）。

**`.gitignore` 示例：**
```text
# 忽略所有的 .log 文件
*.log

# 忽略 node_modules 文件夹
node_modules/

# 但不忽略 debug.log
!debug.log
```

---

## 📝 第八章：Git 提交规范

为了让团队成员看得懂提交历史，建议采用 **Angular 规范** 的 Commit 格式：

* `feat:` 新增功能 (Feature)
* `fix:` 修复 Bug
* `docs:` 仅仅修改了文档 (Documentation)
* `style:` 代码格式调整（不影响逻辑）
* `refactor:` 代码重构（既不是新增功能，也不是修改 bug）
* `perf:` 性能优化 (Performance)
* `test:` 增加或修改测试用例
* `chore:` 构建过程或辅助工具的变动

**示例**：
> `git commit -m "feat: 新增了用户微信扫码登录功能"`


