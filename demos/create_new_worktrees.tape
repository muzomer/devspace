# Where should we write the GIF?
Output create_new_worktrees.gif

# Set up a 1200x600 terminal with 46px font.
Set FontSize 15
Set Width 1200
Set Height 800

####################################################################
# Go to the demos directory and print initial directory structure  #
####################################################################
Hide
Type "cd /var/tmp/demo/"
Enter
Show

Sleep 1s
Type "tree"
Enter
Sleep 1s

# Env DEVSPACE_WORKTREES_DIR "./worktrees_dir/"
# Env DEVSPACE_REPOS_DIR "./repositories_dir/"

Type "export DEVSPACE_WORKTREES_DIR='./worktrees_dir/'"
Enter
Type "export DEVSPACE_REPOS_DIR='./repositories_dir/'"
Enter

Sleep 2s

########################
# Start devspace       #
########################

Type "devspace"
Sleep 2s
Enter
Sleep 2s

########################
# First worktree       #
########################

Ctrl+D
Sleep 1s
Ctrl+N
Sleep 1s
Enter
Sleep 1s
Type "add-new-button-to-the-ui"
Enter
Sleep 2s

# Quit devspace
Enter

# Show directory structure after creation of the first worktree
Type "tree"
Enter
Sleep 3s

########################
# Second worktree      #
########################

# Start devspace
Type "devspace"
Sleep 2s
Enter
Sleep 2s
Ctrl+D
Sleep 1s
Enter
Sleep 1s
Type "add-backend-api-for-the-new-button"
Enter
Sleep 2s

# Quit devspace
Enter

# Show directory structure after creation of the second worktree
Type "tree"
Enter
Sleep 3s

###############################################
# Go to the directory of one of the worktree  #
###############################################

Type "devspace"
Sleep 2s
Enter
Sleep 2s
Ctrl+N
Sleep 1s
Enter
Sleep 1s

Type "cd "
Paste

Sleep 2s
Enter
Sleep 2s

Type "pwd"

Sleep 1s
Enter
Sleep 2s
