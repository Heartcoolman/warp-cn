# Teams settings page strings.
# Keys MUST start with `settings-teams-`.

settings-teams-title = Teams

# Constants / placeholders / button labels
settings-teams-team-name-placeholder = Team name
settings-teams-create-team-description = When you create a team, you can collaborate on agent-driven development by sharing cloud agent runs, environments, automations, and artifacts. You can also create a shared knowledge store for teammates and agents alike.
settings-teams-leave-team-button = Leave team
settings-teams-delete-team-button = Delete team
settings-teams-create-button = Create
settings-teams-approve-domains-placeholder = Domains, comma separated
settings-teams-emails-placeholder = Emails, comma separated
settings-teams-approve-domains-button = Set
settings-teams-send-email-invites-button = Invite

# Validation / instructions
settings-teams-invalid-domains-instructions = Some of the provided domains are invalid, or have already been added.
settings-teams-invite-link-toggle-instructions = As an admin, you can choose whether to enable or disable the ability for team members to invite others by invitation link.
settings-teams-invite-link-domain-restrictions-instructions = Only allow users with emails at specific domains to join your team through the invite link.
settings-teams-invite-by-email-expiry-instructions = Email invitations are valid for 7 days.
settings-teams-invalid-emails-instructions = Some of the provided email addresses are invalid, already invited, or members of the team.
settings-teams-offline = You are offline.

# Member-limit messages
settings-teams-limit-hit-admin = You've reached the team member limit for your plan. Upgrade to add more teammates.
settings-teams-limit-hit-admin-not-auto-upgradeable = You've reached the team member limit for your plan. Contact support@warp.dev to add more teammates.
settings-teams-limit-hit-non-admin = You've reached the team member limit for your plan. Contact a team admin to add more teammates.

# Delinquent (payment) messages
settings-teams-delinquent-admin-non-self-serve = Team invites have been restricted due to a payment issue. Please contact support@warp.dev to restore access.
settings-teams-delinquent-non-admin = Team invites have been restricted due to a payment issue. Please contact a team admin to restore access.
settings-teams-delinquent-admin-self-serve-line1 = Team invites have been restricted due to a subscription payment issue.
settings-teams-delinquent-admin-self-serve-line2-prefix = Please
settings-teams-delinquent-admin-self-serve-line2-link = update your payment information
settings-teams-delinquent-admin-self-serve-line2-suffix = to restore access.

# Team-limit-exceeded messages
settings-teams-limit-exceeded-admin-not-auto-upgradeable = You've exceeded the team member limit for your plan. Please contact support@warp.dev to upgrade your team.
settings-teams-limit-exceeded-non-admin = You've exceeded the team member limit for your plan. Contact a team admin to upgrade your team.
settings-teams-limit-exceeded-admin-upgradeable = You've exceeded the team member limit for your plan. Upgrade to add more teammates.

# Editor / dialogs
settings-teams-new-team-name-placeholder = Your new team name
settings-teams-transfer-ownership-title = Transfer team ownership?

# Toast messages
settings-teams-toast-failed-send-invite = Failed to send invite
settings-teams-toast-toggled-invite-links = Toggled invite links
settings-teams-toast-failed-toggle-invite-links = Failed to toggle invite links
settings-teams-toast-reset-invite-links = Reset invite links
settings-teams-toast-failed-reset-invite-links = Failed to reset invite links
settings-teams-toast-deleted-invite = Deleted invite
settings-teams-toast-failed-delete-invite = Failed to delete invite
settings-teams-toast-failed-add-domain-restriction = Failed to add domain restriction
settings-teams-toast-failed-delete-domain-restriction = Failed to delete domain restriction
settings-teams-toast-toggled-team-discoverability = Toggled team discoverability
settings-teams-toast-failed-toggle-team-discoverability = Failed to toggle team discoverability
settings-teams-toast-joined-team = Successfully joined team
settings-teams-toast-joined-team-named = Successfully joined { $team }
settings-teams-toast-failed-join-team = Failed to join team
settings-teams-toast-transferred-ownership = Successfully transferred team ownership
settings-teams-toast-failed-transfer-ownership = Failed to transfer team ownership
settings-teams-toast-updated-member-role = Successfully updated team member role
settings-teams-toast-failed-update-member-role = Failed to update team member role
settings-teams-toast-error-leaving-team = Error leaving team
settings-teams-toast-left-team = Successfully left team
settings-teams-toast-renamed-team = Successfully renamed team
settings-teams-toast-failed-rename-team = Failed to rename team
settings-teams-toast-link-copied = Link copied to clipboard!
settings-teams-toast-invite-on-the-way = Your invite is on the way!
settings-teams-toast-invites-on-the-way = Your { $count } invites are on the way!
settings-teams-toast-failed-generate-upgrade-link = Failed to generate upgrade link. Please contact us at feedback@warp.dev
settings-teams-toast-failed-generate-billing-link = Failed to generate billing link. Please contact us at feedback@warp.dev

# Member action menu labels
settings-teams-action-cancel-invite = Cancel invite
settings-teams-action-transfer-ownership = Transfer ownership
settings-teams-action-demote-from-admin = Demote from admin
settings-teams-action-promote-to-admin = Promote to admin
settings-teams-action-remove-from-team = Remove from team
settings-teams-action-remove-domain = Remove domain

# Section / sub-headers
settings-teams-section-team-members = Team members
settings-teams-section-team-members-caps = Team Members
settings-teams-section-invite-by-link = Invite by Link
settings-teams-section-invite-by-email = Invite by Email
settings-teams-section-restrict-by-domain = Restrict by domain
settings-teams-section-make-team-discoverable = Make team discoverable
settings-teams-section-create-a-team = Create a team
settings-teams-section-or-join-existing = Or, join an existing team within your company
settings-teams-subsection-invite-team-members = Invite team members
settings-teams-subsection-by-link = By link
settings-teams-subsection-by-email = By email
settings-teams-subsection-by-discovery = By discovery

# Plan / billing
settings-teams-contact-support = Contact support
settings-teams-manage-billing = Manage billing
settings-teams-open-admin-panel = Open admin panel
settings-teams-upgrade-build = Upgrade to Build
settings-teams-upgrade-turbo = Upgrade to Turbo plan
settings-teams-upgrade-lightspeed = Upgrade to Lightspeed plan
settings-teams-compare-plans = Compare plans
settings-teams-free-plan-usage-limits = Free plan usage limits
settings-teams-plan-usage-limits = Plan usage limits
settings-teams-plan-shared-notebooks = Shared Notebooks
settings-teams-plan-shared-workflows = Shared Workflows
settings-teams-manage-plan = Manage plan
settings-teams-reset-links = Reset links

# Invite-link / discover
settings-teams-failed-load-invite-link = Failed to load invite link.
settings-teams-allow-same-domain-find = Allow Warp users with the same email domain as you to find and join the team.
settings-teams-allow-domain-find = Allow Warp users with an @{ $domain } email to find and join the team.
settings-teams-join-and-collaborate = Join this team and start collaborating on workflows, notebooks, and more.
settings-teams-contact-admin-request-access = Contact Admin to request access
settings-teams-invite-link-unavailable-instructions = Your team’s visibility settings prevent you from creating invite links.
settings-teams-invite-link-update-visibility-prefix = { " " }Update team visibility in the { " " }
settings-teams-workspace-admin-panel-link = workspace admin panel
settings-teams-invite-link-update-visibility-suffix = .
settings-teams-section-join-existing = Join an existing team within your company
settings-teams-no-joinable-teams-header = There are currently no joinable teams.
settings-teams-no-teams-to-join-description = Contact an admin to join a team to gain access to more features.
settings-teams-admin-panel-cta-link = Visit the admin panel
settings-teams-admin-panel-cta-trailing = to manage and create new teams.
