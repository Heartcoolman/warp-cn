# Code review module UI strings (PR/diff review).
# Keys MUST start with `code-review-`. Group by file: -view-, -comment-, -comment-list-, -diff-menu-, -diff-selector-, -git-, -git-commit-, -git-pr-, -git-push-.

# diff_selector.rs (1)
code-review-diff-selector-uncommitted-changes = Uncommitted changes

# diff_menu.rs (1)
code-review-diff-menu-search-placeholder = Search diff sets or branches to compare…

# comment_rendering.rs (1)
code-review-comment-fallback-title = Review Comment

# comment_list_view.rs (10)
code-review-comment-list-cancel = Cancel
code-review-comment-list-no-non-outdated = No non-outdated comments to send
code-review-comment-list-ai-disabled = AI must be enabled to send comments to Agent
code-review-comment-list-ai-credits-required = Agent code review requires AI credits
code-review-comment-list-all-terminals-busy = All terminals are busy
code-review-comment-list-send-to-agent-tooltip = Send diff comments to Agent
code-review-comment-list-send-to-cli-tooltip = Send diff comments to { $label }
code-review-comment-list-send-to-agent-button = Send to Agent
code-review-comment-list-menu-copy-text = Copy text
code-review-comment-list-menu-view-in-github = View in GitHub
code-review-comment-list-menu-remove = Remove

# git_dialog/mod.rs (12 strings: user_facing_git_error + dialog title/buttons + branch/changes labels)
code-review-git-error-no-changes = No changes to commit.
code-review-git-error-no-staged-changes = No staged changes to commit.
code-review-git-error-identity-not-configured = Git identity not configured. Set user.name and user.email.
code-review-git-error-pull-before-push = Remote has new changes — pull before pushing.
code-review-git-error-no-remote = No remote configured for this branch.
code-review-git-error-auth-failed = Authentication failed. Check your Git credentials.
code-review-git-error-network = Network error. Check your connection.
code-review-git-error-repo-not-found = Remote repository not found.
code-review-git-error-gh-cli-missing = GitHub CLI (gh) not installed. See https://cli.github.com/.
code-review-git-error-gh-not-authenticated = GitHub CLI not authenticated. Run `gh auth login`.
code-review-git-error-git-operation-in-progress = Another git operation is in progress. Finish or abort it first.
code-review-git-error-generic = Git operation failed.
code-review-git-button-confirm = Confirm
code-review-git-button-cancel = Cancel
code-review-git-title-commit = Commit your changes
code-review-git-title-publish-branch = Publish branch
code-review-git-title-push-changes = Push changes
code-review-git-title-create-pr = Create pull request

# git_dialog/push.rs (3)
code-review-git-push-button-publish = Publish
code-review-git-push-button-push = Push
code-review-git-push-loading-label = Pushing…
code-review-git-push-publishing-label = Publishing…

# git_dialog/pr.rs (4)
code-review-git-pr-button-create = Create PR
code-review-git-pr-loading-label = Creating…
code-review-git-pr-title-placeholder = PR title
code-review-git-pr-body-placeholder = PR body (optional)

# git_dialog/commit.rs (8)
code-review-git-commit-generating-placeholder = Generating commit message…
code-review-git-commit-fallback-placeholder = Type a commit message
code-review-git-commit-and-push = Commit and push
code-review-git-commit-and-publish = Commit and publish
code-review-git-commit-button = Commit
code-review-git-commit-and-create-pr = Commit and create PR
code-review-git-commit-tooltip-enter-message = Enter a commit message
code-review-git-commit-toast-committed = Changes successfully committed.
code-review-git-commit-toast-committed-pushed = Changes committed and pushed.
code-review-git-commit-loading-label = Committing…

# git_dialog/push.rs + pr.rs (4)
code-review-git-push-toast-published = Branch successfully published.
code-review-git-push-toast-pushed = Changes successfully pushed.
code-review-git-pr-toast-open-link = Open PR
code-review-git-pr-toast-created = PR successfully created.

# git_dialog/mod.rs (file count plural)
code-review-git-files-count = { $count } { $count ->
    [one] file
   *[other] files
}

# code_review_view.rs (main view, ~50 strings)
code-review-view-hide-file-navigation = Hide file navigation
code-review-view-show-file-navigation = Show file navigation
code-review-view-tooltip-view-changes = View changes
code-review-view-disabled-remote = Diffs only work for local workspaces.
code-review-view-disabled-non-git = Diffs only work for git repositories.
code-review-view-disabled-wsl = Diffs don't currently work in WSL.
code-review-view-discard-tooltip-git-busy = Cannot discard changes while a git operation (merge, rebase, etc.) is in progress
code-review-view-discard-tooltip-no-changes = No changes to discard
code-review-view-discard-title-uncommitted = Discard uncommitted changes?
code-review-view-discard-title-file-uncommitted = Discard all uncommitted changes to file?
code-review-view-discard-title-all-against-branch = Discard all changes?
code-review-view-discard-title-file-against-branch = Discard all changes to file?
code-review-view-discard-body-all-uncommitted = You're about to discard all local changes that haven't been committed.
code-review-view-discard-body-file-uncommitted = This will restore this file to the last committed version and discard local edits.
code-review-view-discard-body-all-against-branch = You're about to discard all committed and uncommitted changes.
code-review-view-discard-body-file-against-branch = This will restore this file to the main branch version and discard all committed and uncommitted edits.
code-review-view-discard-body-file-against-named-branch = This will reset this file to the { $branch } branch version and discard all committed and uncommitted edits.
code-review-view-tooltip-maximize = Maximize
code-review-view-tooltip-restore = Restore
code-review-view-button-commit = Commit
code-review-view-button-push = Push
code-review-view-button-undo = Undo
code-review-view-button-retry = Retry
code-review-view-button-discard-changes = Discard changes
code-review-view-button-cancel = Cancel
code-review-view-title = Code review
code-review-view-button-initialize-codebase = Initialize codebase
code-review-view-tooltip-initialize-codebase = Enables codebase indexing and WARP.md
code-review-view-button-open-repository = Open repository
code-review-view-tooltip-open-repository = Navigate to a repo and initialize it for coding
code-review-view-uncommitted-changes-row = Uncommitted changes
code-review-view-discard-changes-button = Discard changes
code-review-view-tooltip-open-file = Open file
code-review-view-tooltip-add-file-diff-context = Add file diff as context
code-review-view-tooltip-copy-file-path = Copy file path
code-review-view-loading-open-changes = Loading open changes...
code-review-view-toast-comments-sent = Comments sent to agent
code-review-view-toast-comments-error = Could not submit comments to the agent
code-review-view-stash-changes = Stash changes
code-review-view-diff-removed = Diff removed
code-review-view-cannot-attach-terminal-busy = Cannot attach context when terminal is running
code-review-view-cannot-attach-input-unavailable = Cannot attach diff while input is not available
code-review-view-button-create-pr = Create PR
code-review-view-button-publish = Publish
code-review-view-tooltip-no-changes-to-commit = No changes to commit
code-review-view-tooltip-no-git-actions = No git actions available
code-review-view-menu-commit = Commit
code-review-view-menu-push = Push
code-review-view-menu-publish = Publish
code-review-view-menu-create-pr = Create PR
code-review-view-menu-add-diff-context = Add diff set as context
code-review-view-menu-show-comment = Show saved comment
code-review-view-menu-add-comment = Add comment
code-review-view-menu-discard-all = Discard all
code-review-view-header-reviewing = Reviewing code changes
code-review-view-toast-repo-changed = Code Review: Repository changed. Branch list cleared.
code-review-view-repo-initialized-with-file = Repo is initialized with a { $file_name } file.
code-review-view-unsaved-file-tooltip = This file has unsaved changes. { $shortcut } to save
