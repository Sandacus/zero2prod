-- migrations/20260324152053_add_status_to_subscriptions
ALTER TABLE subscriptions ADD COLUMN status TEXT NULL;
