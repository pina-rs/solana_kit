---
solana_kit_helius: minor
---

Add unsigned checkout creation, project plan changes and pending-intent renewals
behind a smaller checkout feature. Signup composes it with date parsing.
Support named plans or explicit prices, optional contact
data and positive quantities. Share checkout preparation with unified signup.
Keep payment input failures in PaymentError and SignupError::Payment, retaining
original service errors. Reject mismatched renewal identities, dot-segment IDs and zero totals;
propagate preview failures without retrying initialization or submitting funds.
