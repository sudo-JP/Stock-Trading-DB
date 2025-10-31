# SQL Schema Design

## Overview
This document describes the schema design for the trading backend database. It outlines each table, its purpose, key fields, relationships, and indexes. The design emphasizes consistency, performance, and maintainability.

## Account
**Purpose:** Store account information from Alpaca API.

**Columns:**
- account_id: TEXT, PRIMARY KEY — Unique account identifier
- currency: TEXT, NOT NULL — Account currency
- buying_power: DECIMAL, NOT NULL — Buying power of the account
- cash: DECIMAL, NOT NULL — Cash balance (must be non-negative)
- portfolio_value: DECIMAL, NOT NULL — Total portfolio value
- equity: DECIMAL, NOT NULL — Account equity
- unrealized_pl: DECIMAL, NOT NULL — Unrealized profit/loss
- realized_pl: DECIMAL, NOT NULL — Realized profit/loss
- status: TEXT, NOT NULL — Account status
- last_update: TIMESTAMPTZ, NOT NULL — Last updated timestamp in UTC

**Indexes:**
- Primary key on `account_id`
- Optional index on `last_update` for quick queries by update time

**Relationships:**
- Can link to `Order` and `Position` tables via `account_id`

**Notes:**
- Cash must always be non-negative
- Currency should be stored in uppercase

