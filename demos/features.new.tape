Output features.gif

Set FontSize 14
Set Theme TokyoNight
Set Padding 2
Set BorderRadius 8
Set TypingSpeed 75ms

Hide
Type "export DEVSPACE_REPOS_DIR=./repos-dir"
Enter
Type "export DEVSPACE_WORKTREES_DIR=./worktrees-dir"
Enter
Show

#######################
# Launch devspace     #
#######################

Type "devspace"
Enter
Sleep 2s

#############################################
# Create first worktree — filter repo picker
#############################################

Type "n"
Sleep 1s

# Filter for backend-repo
Type "back"
Sleep 800ms
Enter
Sleep 800ms

# Hint shows "Will be created from HEAD" — type branch name
Type "feat-user-auth"
Sleep 600ms
Enter
Sleep 2s

#########################################
# Copy path to clipboard and exit
#########################################

Type "q"
Sleep 1s
