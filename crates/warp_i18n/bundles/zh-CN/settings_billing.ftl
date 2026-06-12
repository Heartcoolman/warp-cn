# 计费与用量设置页字符串。
# Keys MUST start with `settings-billing-`.

settings-billing-title = 计费与用量
settings-billing-no-usage-history = 无使用历史
settings-billing-kick-off-agent = 启动智能体任务即可在此查看使用历史。
settings-billing-tab-overview = 概览
settings-billing-tab-usage-history = 使用历史
settings-billing-overage-description = 当达到此美元限额时，Warp 将阻止使用高级模型。每月自动重置。
settings-billing-overage-note = 请注意，临近所选限额产生的 AI 额度可能会超出几美元。

# 超额 / 链接 / 切换标题
settings-billing-overage-usage-link = 查看超额用量详情
settings-billing-overage-toggle-admin-header = 启用高级模型超额用量
settings-billing-overage-toggle-user-header-enabled = 高级模型超额用量已启用
settings-billing-overage-toggle-user-header-disabled = 高级模型超额用量未启用
settings-billing-overage-toggle-description = 在超出套餐限额后继续使用高级模型。用量按 20 美元增量计费，最高至消费上限，余额会在预定计费日扣除。
settings-billing-overage-toggle-user-description = 请联系团队管理员启用超额，以获得更多 AI 用量。

# 排序菜单
settings-billing-sort-name-a-z = A 到 Z
settings-billing-sort-name-z-a = Z 到 A
settings-billing-sort-usage-asc = 用量升序
settings-billing-sort-usage-desc = 用量降序
settings-billing-sort-by = 排序
settings-billing-usage-column = 用量
settings-billing-team-total = 团队合计

# 自动充值警告
settings-billing-auto-reload-exceed-warning = 自动充值已禁用，下次充值会超出你的月度消费上限。请提高上限以使用自动充值。
settings-billing-auto-reload-delinquent-warning = 因计费问题被限制。请更新支付方式以购买附加额度。
settings-billing-restricted-billing-usage-warning = 由于近期充值失败，自动充值已禁用。请更新支付方式后重试。

# 企业说明
settings-billing-enterprise-callout-header = 用量报告暂受限
settings-billing-enterprise-callout-admin-prefix = 此视图暂未完整支持企业额度用量统计。如需精确的支出追踪，请
settings-billing-enterprise-callout-admin-link = 访问管理员面板
settings-billing-enterprise-callout-non-admin = 此视图暂未完整支持企业额度用量统计。如需详细的用量报告，请联系团队管理员。

# 附加额度
settings-billing-addon-description = 附加额度以预付套餐方式购买，每个计费周期滚存，一年后过期。购买越多，单价越优惠。基础套餐额度用尽后，会消耗附加额度。
settings-billing-addon-team-description = 已购买的附加额度由整个团队共享。
settings-billing-addon-header = 附加额度

# 云端智能体试用
settings-billing-ambient-trial-title = 云端智能体试用

# 消费上限
settings-billing-overage-limit-label = 超额消费上限
settings-billing-monthly-limit-label = 月度消费上限
settings-billing-monthly-overage-spend-label = 月度超额消费上限
settings-billing-overage-limit-helper = 设定超出套餐金额的月度超额消费上限
settings-billing-monthly-spend-helper = 设定每月用于附加额度的消费上限
settings-billing-monthly-spend-limit = 月度消费上限
settings-billing-purchased-this-month = 本月已购
settings-billing-not-set = 未设置
settings-billing-load-more = 加载更多

# 按钮 / 操作
settings-billing-new-agent-button = 新建智能体
settings-billing-buy-more-button = 购买更多
settings-billing-buy-button = 购买
settings-billing-buying-button = 购买中…
settings-billing-one-time-purchase = 一次性购买
settings-billing-auto-reload = 自动充值
settings-billing-auto-reload-description = 启用后，当附加额度余额降至 100 时，自动充值会自动购买 {$amount} 额度。
settings-billing-sign-up = 注册
settings-billing-plan = 套餐
settings-billing-plan-free = 免费版
settings-billing-credits-header = 额度

# 提示框
settings-billing-toast-update-failed = 更新工作区设置失败
settings-billing-toast-purchase-success = 已成功购买附加额度

# 剩余额度
settings-billing-one-credit-remaining = 剩余 1 额度
settings-billing-credits-remaining = 剩余 {$count} 额度
settings-billing-one-credit = 1 额度
settings-billing-n-credits = {$count} 额度

# 充值超限内联提示
settings-billing-reload-exceed = 充值会超出你的月度上限。
settings-billing-increase-your-limit = 提高上限
settings-billing-to-continue = " 以继续。"

# 套餐升级行
settings-billing-switch-build-plan = 切换到 Build 套餐
settings-billing-upgrade-build-plan = 升级到 Build 套餐
settings-billing-build-plan-suffix = " 以购买附加额度。"
settings-billing-contact-ae = 请联系你的客户经理获取更多附加额度。
settings-billing-contact-admin = 请联系团队管理员购买附加额度。

# 合计 / 计费日 / 受限
settings-billing-total-overages = 超额合计
settings-billing-usage-resets-on = 用量将于 {$date} 重置
settings-billing-prorated-self = 你的额度上限按比例计算，因为你在计费周期中途加入。
settings-billing-prorated-other = 此用户的额度上限按比例计算，因为该用户在计费周期中途加入。
settings-billing-restricted-due-to-billing = 因计费问题被限制
settings-billing-account-credits-info = 这是你账户的 {$duration} AI 额度上限。
settings-billing-last-30-days = 最近 30 天

# 套餐卡 / 升级
settings-billing-action-upgrade-plan = 升级套餐
settings-billing-action-stripe-portal = 生成 Stripe 计费门户链接
settings-billing-action-unknown = 未知原因
settings-billing-manage-billing = 管理计费
settings-billing-open-admin-panel = 打开管理员面板
settings-billing-compare-plans = 对比套餐
settings-billing-bring-your-own-key = 自带密钥
settings-billing-contact-support = 联系支持
settings-billing-no-default = 你选择的

# 套餐升级文案
settings-billing-manage-billing-suffix = " 以恢复 AI 功能。"
settings-billing-contact-admin-billing = 请联系团队管理员解决计费问题。
settings-billing-flexible-pricing-suffix = " 以获得更灵活的计费模式。"
settings-billing-or-byok = " 或 "
settings-billing-byok-increased-suffix = " 以获得更多 AI 功能。"
settings-billing-upgrade-turbo = 升级到 Turbo 套餐
settings-billing-upgrade-lightspeed = 升级到 Lightspeed 套餐
settings-billing-more-ai-usage-suffix = " 以获取更多 AI 用量。"
settings-billing-upgrade-max = 升级到 Max
settings-billing-more-credits-suffix = " 以获取更多 AI 额度。"
settings-billing-switch-business = 切换到 Business
settings-billing-business-feature-suffix = " 以获得 SSO 等安全特性，并自动应用零数据保留。"
settings-billing-upgrade-enterprise = 升级到 Enterprise
settings-billing-enterprise-suffix = " 以获取定制限额与专属支持。"
settings-billing-contact-support-suffix = " 以获取更多 AI 用量。"
settings-billing-build-byok-credits-suffix = " 以获得更多额度和更多模型。"
settings-billing-usage-history-empty-hint = 启动一个智能体任务后，即可在此查看用量历史。
