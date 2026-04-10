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
# Create first worktree
#############################################

Type "n"
Sleep 1s

# Go to second-repo
Type "j"
Sleep 800ms
Enter
Sleep 800ms

# Hint shows "Will be created from HEAD" — type branch name
Type@0s "feature-in-second-repo"
Sleep 600ms
Enter
Sleep 600ms

#############################################
# Create second worktree from PR (manual)
#############################################

Type "p"
Sleep 1s

Type@0s "https://github.com/muzomer/third-repo/pull/2"
Sleep 1s

Enter
Sleep 1s

Enter
Sleep 2s

Enter
Sleep 2s

#############################################
# Navigate the worktrees list
#############################################

Type "k"
Sleep 500ms

#############################################
# Show help popup
#############################################

Type "?"
Sleep 2s
Escape

#############################################
# Create third worktree from PR (auto-clone)
# P = Shift+P — skips all confirmation prompts
#############################################

Type "P"
Sleep 1s

Type@0s "https://github.com/muzomer/second-repo/pull/1"
Sleep 500ms
Enter
Sleep 1s

#############################################
# Filter / search worktrees
#############################################

Type "i"
Sleep 1s
Type@0s "dev"
Sleep 800ms
Backspace
Backspace
Backspace
Escape
Sleep 1s

#############################################
# Delete a worktree (soft delete with confirm)
#############################################

Type "j"
Sleep 400ms
Type "d"
Sleep 800ms
Enter
Sleep 1500ms

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
Sleep 4s
