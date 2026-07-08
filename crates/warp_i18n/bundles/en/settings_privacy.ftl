# Privacy settings page strings.
# Keys MUST start with `settings-privacy-`.

settings-privacy-title = Privacy
settings-privacy-safe-mode-title = Secret redaction
settings-privacy-safe-mode-description = When this setting is enabled, Warp will scan blocks, the contents of Warp Drive objects, and Oz prompts for potential sensitive information and prevent saving or sending this data to any servers. You can customize this list via regexes.
settings-privacy-custom-secret-title = Custom secret redaction
settings-privacy-custom-secret-description = Use regex to define additional secrets or data you'd like to redact. This will take effect when the next command runs. You can use the inline (?i) flag as a prefix to your regex to make it case-insensitive.
settings-privacy-telemetry-title = Help improve Warp
settings-privacy-telemetry-description = App analytics help us make the product better for you. We may collect certain console interactions to improve Warp's AI capabilities.
settings-privacy-telemetry-description-old = App analytics help us make the product better for you. We only collect app usage metadata, never console input or output.
settings-privacy-data-management-title = Manage your data
settings-privacy-data-management-description = At any time, you may choose to delete your Warp account permanently. You will no longer be able to use Warp.
settings-privacy-data-management-link = Visit the data management page
settings-privacy-policy-title = Privacy policy
settings-privacy-policy-link = Read Warp's privacy policy
settings-privacy-add-regex-pattern = Add regex pattern
settings-privacy-add-recommended-regex-failed = Failed to add recommended regex to custom secret regex list
settings-privacy-tab-personal = Personal
settings-privacy-tab-enterprise = Enterprise
settings-privacy-enterprise-not-modifiable = Enterprise secret redaction cannot be modified.
settings-privacy-no-enterprise-regexes = No enterprise regexes have been configured by your organization.
settings-privacy-zdr-label = ZDR
settings-privacy-crash-reports-description = Crash reports assist with debugging and stability improvements.
settings-privacy-regex-name-label = Name (optional)
settings-privacy-regex-pattern-label = Regex pattern
settings-privacy-regex-invalid = Invalid regex

# Recommended regex section / add-button labels
settings-privacy-recommended-header = Recommended
settings-privacy-add-all = Add all
settings-privacy-add-regex = Add regex

# Enterprise / org-managed states
settings-privacy-enabled-by-organization = Enabled by your organization.
settings-privacy-managed-by-organization = This setting is managed by your organization.
settings-privacy-zdr-tooltip = Your administrator has enabled zero data retention for your team. User generated content will never be collected.

# Secret display mode dropdown
settings-privacy-secret-display-mode = Secret visual redaction mode
settings-privacy-secret-display-mode-description = Choose how secrets are visually presented in the block list while keeping them searchable. This setting only affects what you see in the block list.

# Telemetry / docs link
settings-privacy-telemetry-docs-link = Read more about Warp's use of data

# Crash reporting toggle
settings-privacy-crash-reports-title = Send crash reports

# Cloud conversation storage
settings-privacy-cloud-conversations-title = Store AI conversations in the cloud
settings-privacy-cloud-conversations-enabled-desc = Agent conversations can be shared with others and are retained when you log in on different devices. This data is only stored for product functionality, and Warp will not use it for analytics.
settings-privacy-cloud-conversations-disabled-desc = Agent conversations are only stored locally on your machine, are lost upon logout, and cannot be shared. Note: conversation data for ambient agents are still stored in the cloud.

# Network log
settings-privacy-network-log-title = Network log console
settings-privacy-network-log-description = We've built a native console that allows you to view all communications from Warp to external servers to ensure you feel comfortable that your work is always kept safe.
settings-privacy-network-log-link = View network logging
