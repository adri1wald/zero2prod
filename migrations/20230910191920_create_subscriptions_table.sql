CREATE TABLE "Subscription" (
  "id" uuid NOT NULL,
  "email" TEXT NOT NULL UNIQUE,
  "name" TEXT NOT NULL,
  "created_at" TIMESTAMPTZ NOT NULL,

  CONSTRAINT "Subscription_pkey" PRIMARY KEY ("id")
);
