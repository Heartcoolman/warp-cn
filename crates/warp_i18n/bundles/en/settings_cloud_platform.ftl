# Cloud platform settings page strings (Environments, Oz Cloud API Keys).
# Keys MUST start with `settings-platform-`.

settings-cloud-title = Cloud platform
settings-platform-new-api-key = New API key
settings-platform-save-your-key = Save your key
settings-platform-never = Never
settings-platform-no-api-keys = No API Keys
settings-platform-create-key-description = Create a key to manage external access to Warp
settings-platform-documentation-link = Documentation.
settings-platform-col-name = Name
settings-platform-col-key = Key
settings-platform-col-scope = Scope
settings-platform-col-created = Created
settings-platform-col-last-used = Last used
settings-platform-col-expires-at = Expires at
settings-platform-scope-personal = Personal
settings-platform-scope-team = Team
settings-platform-scope-agent = Agent

settings-environments-no-match = No environments match your search.
settings-environments-empty-title = You haven't set up any environments yet.
settings-environments-setup-prompt = Choose how you'd like to set up your environment:

settings-platform-api-key-secret-info = This secret key is shown only once. Copy and store it securely.
settings-platform-api-key-name-label = Name
settings-platform-api-key-type-label = Type
settings-platform-api-key-agent-label = Agent
settings-platform-api-key-expiration-label = Expiration
settings-platform-no-agents-available = No agents available. Create one first.
settings-platform-api-key-deleted = API key deleted
settings-platform-create-api-key-help-line1 = Create and manage API keys to allow cloud agents to access your Warp account.
settings-platform-create-api-key-help-line2-prefix = For more information, visit the

# Update environment form
settings-environments-form-setup-help = Setup commands run independently. Each command runs from the workspace root (/workspace). If a command depends on the previous one, combine them with &&.
settings-environments-form-description-label = Description
settings-environments-form-repos-label = Repo(s)
settings-environments-form-loading = Loading...
settings-environments-form-auth-with-github = Auth with GitHub
settings-environments-form-retry = Retry
settings-environments-form-repo-input-help = Type owner/repo and press Enter to add, or select from dropdown.
settings-environments-form-missing-repo = Missing a repo?
settings-environments-form-configure-on-github = Configure access on GitHub
settings-environments-form-no-repos-found = No repositories found

# Environments page (list / empty state / cards)
settings-environments-page-title = Environments
settings-environments-page-description = Environments define where your ambient agents run. Set one up in minutes via GitHub (recommended), Warp-assisted setup, or manual configuration.
settings-environments-list-search-placeholder = Search environments...
settings-environments-empty-button-loading = Loading...
settings-environments-empty-button-retry = Retry
settings-environments-empty-button-authorize = Authorize
settings-environments-empty-button-get-started = Get started
settings-environments-empty-button-launch-agent = Launch agent
settings-environments-empty-quick-setup-title = Quick setup
settings-environments-empty-quick-setup-badge = Suggested
settings-environments-empty-quick-setup-subtitle = Select the GitHub repositories you'd like to work with and we'll suggest a base image and config
settings-environments-empty-use-agent-title = Use the agent
settings-environments-empty-use-agent-subtitle = Choose a locally set up project and we'll help you set up an environment based on it
settings-environments-section-personal = Personal
settings-environments-section-shared-by-team = Shared by Warp and {$team}
settings-environments-section-shared-by-team-default = Shared by Warp and your team
settings-environments-card-env-id-prefix = Env ID
settings-environments-card-image-prefix = Image
settings-environments-card-repos-prefix = Repos
settings-environments-card-setup-commands-prefix = Setup commands
settings-environments-card-last-edited = Last edited: {$time}
settings-environments-card-last-used = Last used: {$time}
settings-environments-card-last-used-never = Last used: never
settings-environments-card-tooltip-share = Share
settings-environments-card-tooltip-edit = Edit
settings-environments-card-link-view-runs = View my runs
settings-environments-toast-updated = Successfully updated environment
settings-environments-toast-created = Successfully created environment
settings-environments-toast-deleted = Environment deleted successfully
settings-environments-toast-shared = Successfully shared environment
settings-environments-toast-share-failed = Failed to share environment with team
settings-environments-toast-create-not-logged-in = Unable to create environment: not logged in.
settings-environments-toast-save-not-exists = Unable to save: environment no longer exists.
settings-environments-toast-share-not-on-team = Unable to share environment: you are not currently on a team.
settings-environments-toast-share-not-synced = Unable to share environment: environment is not yet synced.

# update_environment_form.rs
settings-environments-update-form-name-placeholder = Environment name
settings-environments-update-form-docker-image-placeholder = e.g. python:3.11, node:20-alpine
settings-environments-update-form-repos-placeholder-authed = Enter repos (owner/repo format)
settings-environments-update-form-repos-placeholder-unauthed = Paste repo URL(s)
settings-environments-update-form-button-create = Create
settings-environments-update-form-button-save = Save
settings-environments-update-form-button-create-environment = Create environment
settings-environments-update-form-button-save-environment = Save environment
settings-environments-update-form-button-edit-environment = Edit environment
settings-environments-update-form-button-delete-environment = Delete environment
settings-environments-update-form-error-load-github-repos = Couldn't load GitHub repos. You can paste repo URL(s), or retry.
settings-environments-update-form-error-load-github-repos-with-error = Failed to load GitHub repos: {$error}
settings-environments-update-form-error-load-github-repositories = Failed to load GitHub repositories
settings-environments-update-form-error-suggest-docker-image = Failed to suggest a Docker image
settings-environments-update-form-error-suggest-docker-image-with-error = Failed to suggest a Docker image: {$error}
settings-environments-update-form-error-suggest-unknown-response = Unknown response from suggestCloudEnvironmentImage
settings-environments-update-form-share-with-team = Share with team
settings-environments-update-form-personal-warning = Personal environments cannot be used with external integrations or team API keys. For the best experience, use shared environments.
settings-environments-update-form-label-name = Name
settings-environments-update-form-label-setup-commands = Setup command(s)
settings-environments-update-form-label-docker-image-reference = Docker image reference
settings-environments-update-form-suggest-image-generating = Generating…
settings-environments-update-form-suggest-image-button = Suggest image
settings-environments-update-form-suggest-image-tooltip = Warp will suggest a Docker image based on your selected repositories.
settings-environments-update-form-button-authenticate = Authenticate
settings-environments-update-form-grant-github-access = You need to grant access to your GitHub repos to suggest a Docker image
settings-environments-update-form-button-launch-agent = Launch agent
settings-environments-update-form-no-good-match = We couldn't find a good match. We recommend using a custom Docker image for these repos.

# delete_environment_confirmation_dialog.rs
settings-environments-delete-dialog-title = Delete environment?
settings-environments-delete-dialog-description = Are you sure you want to remove the {$name} environment?
settings-environments-delete-dialog-cancel = Cancel
settings-environments-delete-dialog-confirm = Delete environment

# transfer_ownership_confirmation_modal.rs
settings-environments-transfer-ownership-modal-description = Are you sure you want to transfer team ownership to {$email}? You will no longer be the owner and will not be able to take any administrative actions for this team.
settings-environments-transfer-ownership-modal-cancel = Cancel
settings-environments-transfer-ownership-modal-confirm = Transfer

# agent_assisted_environment_modal.rs
settings-environments-agent-assisted-modal-add-repo = Add repo
settings-environments-agent-assisted-modal-cancel = Cancel
settings-environments-agent-assisted-modal-create-environment = Create environment
settings-environments-agent-assisted-modal-section-selected-repos = Selected repos
settings-environments-agent-assisted-modal-section-available-repos = Available indexed repos
settings-environments-agent-assisted-modal-loading-indexed-repos = Loading locally indexed repos…
settings-environments-agent-assisted-modal-no-indexed-repos = No locally indexed repos found yet. Index a repo, then try again.
settings-environments-agent-assisted-modal-unavailable = Local repo selection is unavailable in this build.
settings-environments-agent-assisted-modal-description-indexed = Select locally indexed repos to provide context for the environment creation agent.
settings-environments-agent-assisted-modal-description-default = Select repos to provide context for the environment creation agent.
settings-environments-agent-assisted-modal-title = Select repos for your environment
settings-platform-custom-endpoint-description = Provide your endpoint details below. You can add as many models from the endpoint as you'd like and can also provide aliases for the model picker in your input.
settings-platform-no-api-keys-search-match = No API keys match your search
