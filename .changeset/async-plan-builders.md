---
solana_kit_core: minor
solana_kit_client: minor
---

# Await caller-owned message construction during planning

Add concrete asynchronous construction and update futures at actual candidate
boundaries. Preserve the shared synchronous planning traversal, exact packing,
transactional failures, local futures and conditional native Send. Execution
boundaries await planning before signing or sending. Publication remains disabled.
