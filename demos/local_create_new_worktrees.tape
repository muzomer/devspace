# Where should we write the GIF?
Output create_new_worktrees.gif

# Set up a 1200x600 terminal with 46px font.
Set FontSize 15
Set Width 1200
Set Height 800

# Hide

Type "bash"
Enter

Type "rm -rvf /var/tmp/repositories_dir/ /var/tmp/worktrees_dir/"
Enter

Type "mkdir /var/tmp/repositories_dir /var/tmp/worktrees_dir && for demo_repo in backend-repo frontend-repo infra-repo; do git init /var/tmp/repositories_dir/$demo_repo ; cd /var/tmp/repositories_dir/$demo_repo; git commit --allow-empty -m 'initial commit'; done"
Enter

Type "cd $HOME/projects/devspace && cargo install --path . && clear"
Enter

Sleep 1s

# Show

Type "devspace --repos-dir /var/tmp/repositories_dir/ --worktrees-dir /var/tmp/worktrees_dir/"
Enter
Sleep 2s

# First worktree
Ctrl+D

Type "frontend"
Sleep 2s
Enter

Type "add-new-button-in-ui"
Enter
Sleep 2s

# Second worktree
Ctrl+D

Sleep 2s

Type "backend"
Enter
Sleep 2s

Type "feat-new-endpoint-for-the-button"
Enter
Sleep 2s

# Ctrl+D

# Sleep 1s

# Backspace 7

# Sleep 1s

# Type "infra"

# Sleep 2s

# Enter

# Type "feat-new-endpoint-for-the-button"

# Enter

# Sleep 3s
