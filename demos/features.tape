Output features.gif

Set FontSize 14
#Set Width 160
#Set Height 40
Set Theme TokyoNight
#Set Padding 12
Set BorderRadius 8
Set TypingSpeed 75ms

Hide
Type "export DEVSPACE_REPOS_DIR=/repositories_dir"
Enter
Type "export DEVSPACE_WORKTREES_DIR=/worktrees_dir"
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

#############################################
# Create second worktree
#############################################

Type "n"
Sleep 1s

Type "front"
Sleep 800ms
Enter
Sleep 800ms

Type "feat-new-button-ui"
Sleep 600ms
Enter
Sleep 2s

#############################################
# Create third worktree
#############################################

Type "n"
Sleep 1s

Type "infra"
Sleep 800ms
Enter
Sleep 800ms

Type "chore-upgrade-k8s"
Sleep 600ms
Enter
Sleep 2s

####################################################
# Navigate the list — status indicators are visible
####################################################

Down
Sleep 400ms
Down
Sleep 400ms
Up
Sleep 500ms

####################################
# Fuzzy filter worktrees
####################################

Type "i"
Sleep 400ms

Type "feat"
Sleep 1.5s

# Clear and show all again
Backspace
Backspace
Backspace
Backspace
Sleep 400ms
Escape
Sleep 600ms

####################################
# Context-sensitive help popup
####################################

Type "?"
Sleep 2.5s
Escape
Sleep 600ms

#############################################
# PR workflow — open the PR URL input popup
#############################################

Type "p"
Sleep 2s
Escape
Sleep 600ms

####################################
# Delete a worktree with confirmation
####################################

Down
Down
Sleep 400ms
Type "d"
Sleep 1.2s
Enter
Sleep 1.5s

#########################################
# Quit
#########################################

Type "q"
Sleep 1s
