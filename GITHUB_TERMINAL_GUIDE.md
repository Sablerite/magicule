# Using GitHub Commands in the Terminal

This guide will teach you how to interact with GitHub using terminal commands instead of a graphical user interface (GUI). Understanding these commands gives you more control, works consistently across different systems, and is essential for professional development workflows.

## Git vs GitHub: Understanding the Difference

Before diving into commands, it's important to understand:
- **Git**: A distributed version control system that tracks changes to files locally on your computer
- **GitHub**: A cloud-based hosting service for Git repositories that adds collaboration features like pull requests, issue tracking, and CI/CD

Think of Git as the engine and GitHub as the garage with extra tools and features.

## Prerequisites

1. **Install Git**: Download from https://git-scm.com/downloads (Windows includes Git Bash)
2. **Create a GitHub account**: https://github.com/join
3. **Open your terminal**:
   - Windows: Git Bash (installed with Git) or PowerShell
   - macOS/Linux: Terminal application

## Step 1: Configure Git (First-Time Setup)

Before using Git, tell it who you are:

```bash
# Set your username (will appear in commits)
git config --global user.name "Your Name"

# Set your email (should match your GitHub account email)
git config --global user.email "your.email@example.com"

# Optional: Set default branch name to main (modern standard)
git config --global init.defaultBranch main

# Verify your configuration
git config --list
```

> ���� �� �� 💡 **Tip**: Use `--global` to set these for all your repositories. Omit it for repository-specific settings.

## Step 2: Authenticate with GitHub

For security, GitHub no longer accepts account passwords for Git operations. Instead, use one of these methods:

### Option A: Personal Access Token (Recommended for most users)

1. Go to GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click "Generate new token" → "Generate new token (classic)"
3. Give it a descriptive name (e.g., "Terminal Access")
4. Select scopes: At minimum, select `repo` for full repository access
5. Click "Generate token" and **copy the token immediately** (you won't see it again!)
6. Use this token as your password when prompted

### Option B: SSH Keys (More secure for frequent use)

1. Check if you already have SSH keys:
   ```bash
   ls -al ~/.ssh
   ```
   Look for files like `id_rsa.pub` or `id_ed25519.pub`

2. If none exist, generate a new SSH key:
   ```bash
   ssh-keygen -t ed25519 -C "your.email@example.com"
   ```
   Press Enter to accept default file location and optionally set a passphrase

3. Add your SSH public key to GitHub:
   ```bash
   # Copy the public key to clipboard
   clip < ~/.ssh/id_ed25519.pub  # Windows Git Bash
   # or
   pbcopy < ~/.ssh/id_ed25519.pub  # macOS
   # or
   xclip -selection clipboard < ~/.ssh/id_ed25519.pub  # Linux (may need xclip installed)
   ```
4. Go to GitHub → Settings → SSH and GPG keys → New SSH key
5. Paste the key and save

## Core Git Workflow: The Three Trees

Git maintains three trees that you work with:
1. **Working Directory**: Where you edit files (visible in your file explorer)
2. **Staging Index**: Where you prepare changes for the next commit
3. **HEAD**: Points to your last commit

The basic workflow moves changes from Working Directory → Staging Index → HEAD (via commit) → Remote Repository (via push).

## Essential Commands Explained

### Getting a Repository

#### Cloning an Existing Repository
```bash
# Creates a local copy of a remote repository
git clone https://github.com/username/repository-name.git

# Example cloning your own project
git clone https://github.com/Sablerite/Magicule.git

# Clones into a directory called 'Magicule'
# After cloning, you're automatically on the default branch (usually main)
```

#### Initializing a New Repository
```bash
# Use when starting a new project locally
git init

# Creates a .git subdirectory to track changes
# Then connect to GitHub:
git remote add origin https://github.com/username/new-repo.git
git branch -M main  # Rename current branch to main
git push -u origin main
```

### Checking Status and Viewing Changes

```bash
# See which files have changed, what's staged, and branch info
git status

# View detailed changes in working directory (not staged)
git diff

# View staged changes (what will be committed)
git diff --staged

# View commit history
git log
git log --oneline  # Compact view
git log --graph --oneline --all  # Visualize branches
```

### Staging and Committing Changes

```bash
# Stage specific file
git commit messages/GITHUB_TERMINAL_GUIDE.md

# Stage all modified and deleted files (but not new files)
git add -u

# Stage all changes including new files
git add .
# or
git add --all

# Commit staged changes with a message
git commit -m "Add GitHub terminal guide documentation"

# Commit all tracked changes in one step (skips staging)
git commit -am "Fix typo in README"
# Note: -a only works on already-tracked files, not new files
```

> ���� �� �� 📝 **Commit Message Best Practices**:
> - First line: 50 characters or less, imperative mood ("Add feature" not "Added feature")
> - Blank line
> - Optional detailed explanation wrapped at 72 characters
> - Example:
>   ```
>   Add GitHub terminal guide documentation
>   
>   Created comprehensive guide for using Git commands in terminal
>   covering authentication, basic workflow, branching, and troubleshooting.
>   ```

### Sharing Changes with GitHub

```bash
# Send your commits to GitHub
git push

# First time pushing a new branch, set upstream:
git push -u origin feature-branch-name

# If you get rejected because remote has changes you don't have:
git pull  # Fetch and merge remote changes
# Then try push again

# Alternative to pull when you want to see changes first:
git fetch    # Download remote changes without merging
git log origin/main  # See what's on remote
git merge origin/main  # Then merge if you want to
```

### Getting Updates from GitHub

```bash
# Fetch downloads changes but doesn't merge them
git fetch

# Pull fetches AND merges remote changes into your current branch
git pull

# Equivalent to: git fetch + git merge
# Use pull when you're ready to integrate changes
# Use fetch when you just want to see what changed first
```

## Working with Branches

Branches allow you to work on features or fixes without affecting the main codebase.

```bash
# List all branches (current branch highlighted)
git branch

# Create a new branch
git branch feature-name

# Switch to a branch
git checkout feature-name

# Create and switch in one command
git checkout -b feature-name

# Push your branch to GitHub
git push -u origin feature-name

# Delete a local branch (after merging)
git branch -d feature-name

# Delete a remote branch
git push origin --delete feature-name
```

### Typical Feature Workflow

1. Update main branch: `git checkout main && git pull`
2. Create feature branch: `git checkout -b feature/new-feature`
3. Make changes and commit regularly:
   ```bash
   git add .
   git commit -m "Implement part of new feature"
   ```
4. Push branch to GitHub: `git push -u origin feature/new-feature`
5. Open a Pull Request on GitHub
6. After approval, merge and clean up:
   ```bash
   git checkout main
   git pull
   git branch -d feature/new-feature
   git push origin --delete feature/new-feature
   ```

## Collaborating with Others

### Fork and Pull Request Model (for contributing to projects you don't own)

1. On GitHub, click "Fork" on a repository to create your copy
2. Clone your fork: `git clone https://github.com/your-username/forked-repo.git`
3. Add the original as "upstream": 
   ```bash
   git remote add upstream https://github.com/original-owner/original-repo.git
   ```
4. Create branches from your fork's main: `git checkout -b feature-name`
5. Make changes, commit, push to your fork
6. On GitHub, click "New Pull Request" comparing your fork's branch to original's main
7. Keep your fork updated: 
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   git push
   ```

### Direct Collaboration (for shared repositories)

1. Everyone clones the same repository
2. Create feature branches off main
3. Push branches to the shared repository
4. Open Pull Requests for review
5. Merge approved PRs

## Common Tasks and Troubleshooting

### Undoing Changes

```bash
# Unstage a file (keep changes in working directory)
git reset HEAD file-name

# Discard changes in working directory (revert to last commit)
git checkout -- file-name
# or (newer Git)
git restore file-name

# Uncommit last commit but keep changes staged
git reset --soft HEAD~1

# Uncommit last commit and unstage changes
git reset --mixed HEAD~1  # same as git reset HEAD~1

# Uncommit last commit and discard changes
git reset --hard HEAD~1  # USE WITH CAUTION - loses work!

# Amend last commit (forgot to stage something or fix message)
git add forgotten-file
git commit --amend
# Only do this if you haven't pushed yet!
```

### Resolving Merge Conflicts

When Git can't automatically merge changes:
1. Git marks conflicted files with `<<<<<<<`, `=======`, `>>>>>>>` markers
2. Edit the file to resolve conflicts (choose what to keep)
3. Mark as resolved: `git add file-name`
4. Continue: `git commit` (for merge) or `git rebase --continue` (for rebase)

### Authentication Issues

If you get authentication errors:
- For HTTPS: Ensure you're using a Personal Access Token, not your password
- For SSH: Ensure your SSH key is added to GitHub and ssh-agent is running
  ```bash
  # Start ssh-agent and add key
  eval $(ssh-agent)
  ssh-add ~/.ssh/id_ed25519
  ```
- Test connection: `ssh -T git@github.com`

### Large Files

Git isn't designed for large binary files. For files >100MB:
- Use Git LFS (Large File Storage): https://git-lfs.github.com/
- Install: `git lfs install`
- Track file types: `git lfs track "*.psd"`
- Commit the `.gitattributes` file

## Best Practices Summary

### Daily Workflow
1. Start day: `git checkout main && git pull`
2. Create branch: `git checkout -b feature/task-name`
3. Work: Make changes → `git add .` → `git commit -m "message"` (repeat)
4. Share work: `git push -u origin feature/task-name`
5. Request review: Open Pull Request on GitHub
6. After merge: Delete branch locally and remotely

### Commit Guidelines
- Commit early, commit often
- Each commit should be a single logical change
- Write clear, descriptive commit messages
- Never commit secrets (keys, passwords) - use environment variables or secret managers

### Branch Naming Conventions
- `feature/`: New features (`feature/user-login`)
- `fix/`: Bug fixes (`fix/typo-in-readme`)
- `docs/`: Documentation changes (`docs/api-update`)
- `refactor/: `Code restructuring (`refactor/database-layer`)
- `test/`: Adding or modifying tests (`test/user-auth-tests`)

### Repository Hygiene
- Keep main branch always deployable
- Delete branches after merging
- Tag releases: `git tag -a v1.0.0 -m "Release version 1.0.0"` then `git push origin v1.0.0`
- Use `.gitignore` to exclude build artifacts, dependencies, and IDE files

## Resources for Continued Learning

- Official Git Documentation: https://git-scm.com/doc
- GitHub Guides: https://guides.github.com/
- Interactive Git Tutorial: https://learngitbranching.js.org/
- Git Cheat Sheet: https://education.github.com/git-cheat-sheet-education.pdf
- Pro Git Book (free online): https://git-scm.com/book/en/v2

## Project-Specific Notes for Magicule

For this repository which contains both C++ (neutron) and Rust (proton) projects:

1. **Building before committing**: Consider compiling your code before committing to avoid breaking the build for others
2. **.gitignore importance**: Ensure build directories are ignored:
   - C++/CMake: `build/`, `*/build/`
   - Rust/Cargo: `target/`
3. **Testing locally**: Run tests before pushing:
   ```bash
   # For neutron
   cd neutron && mkdir -p build && cd build && cmake .. && make && ./neutron_tests
   
   # For proton
   cd proton && cargo test
   ```
4. **Documentation updates**: When changing APIs, update README or documentation in same commit

## Conclusion

Mastering terminal Git commands gives you powerful, precise control over your version control workflow. While it may seem slower than using a GUI at first, the terminal approach:
- Works consistently across all operating systems
- Is scriptable and automatable
- Provides clearer understanding of what Git is actually doing
- Is the standard in professional development environments

Start with the basic workflow (clone → modify → add → commit → push) and gradually explore more advanced features like branching, rebasing, and cherry-picking as you become comfortable. Remember that every expert was once a beginner—practice and repetition will make these commands second nature.

Happy coding!