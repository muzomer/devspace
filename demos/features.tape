Output features.gif

Set FontSize 20
Set Theme TokyoNight
Set Padding 2
Set BorderRadius 8
Set TypingSpeed 75ms
Set Width 1200
Set Height 600

Hide
Type@0s "export DEVSPACE_REPOS_DIR=./repos-dir"
Enter
Type@0s "export DEVSPACE_WORKTREES_DIR=./worktrees-dir"
Enter
Sleep 2s
Show

Type@0s "ls -ltrh ./worktrees-dir/"
Enter
Sleep 2s

#######################
# Launch devspace     #
#######################

Type@0s "cd $(devspace)"
Sleep 1s
Enter

#############################################
# Show help popup
#############################################

Type "?"
Sleep 2s
Escape
Sleep 800ms

#############################################
# Navigate the worktrees list
#############################################

Type "j"
Sleep 500ms
Type "j"
Sleep 500ms
Type "k"
Sleep 500ms

#############################################
# Filter / search worktrees
#############################################

Type "i"
Sleep 800ms
Type@0s "dark"
Sleep 1s
Backspace
Backspace
Backspace
Backspace
Escape
Sleep 800ms

#############################################
# Create a new worktree (pick repo)
#############################################

Type "n"
Sleep 1s

# Move down to second-repo
Type "j"
Sleep 800ms
Enter
Sleep 800ms

# Type branch name
Type@0s "feature-in-second-repo"
Sleep 600ms
Enter
Sleep 1s

#############################################
# Show the new worktree selected in the list
#############################################

Sleep 800ms

#############################################
# Delete a worktree (soft delete with confirm)
#############################################

Type "d"
Sleep 800ms
Enter
Sleep 1200ms

#############################################
# Open the PR URL popup (show the styling,
# then cancel — no live GitHub needed)
#############################################

Type "p"
Sleep 1s
Type@0s "https://github.com/example/repo/pull/42"
Sleep 800ms
Escape
Sleep 800ms

#############################################
# Select a worktree — copies path and exits
#############################################

Type "k"
Sleep 400ms
Enter
Sleep 1s

Type@0s "pwd"
Enter
Sleep 1s

Type@0s "cd -"
Enter
Sleep 1s

Type@0s "ls -ldR worktrees-dir/*/*"
Enter
Sleep 3s
