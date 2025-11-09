#!/bin/bash

# Test script for sync server

BASE_URL="http://localhost:3000"

echo "🧪 Testing Sync Server"
echo "====================="
echo ""

# Test health check
echo "1️⃣  Testing health check..."
curl -s "$BASE_URL/health" | jq .
echo ""

# Create a reminder
echo "2️⃣  Creating reminder..."
RESPONSE=$(curl -s -X POST "$BASE_URL/api/reminders" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Test Meeting",
    "description": "This is a test",
    "time": "2024-11-10T10:00:00",
    "completed": false,
    "category": "work",
    "frequency": "once"
  }')
echo "$RESPONSE" | jq .
REMINDER_ID=$(echo "$RESPONSE" | jq -r '.id')
echo ""

# Get all reminders
echo "3️⃣  Getting all reminders..."
curl -s "$BASE_URL/api/reminders" | jq .
echo ""

# Get single reminder
echo "4️⃣  Getting reminder by ID..."
curl -s "$BASE_URL/api/reminders/$REMINDER_ID" | jq .
echo ""

# Update reminder
echo "5️⃣  Updating reminder..."
curl -s -X PUT "$BASE_URL/api/reminders/$REMINDER_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Meeting",
    "description": "This has been updated",
    "time": "2024-11-10T14:00:00",
    "completed": true,
    "category": "work",
    "frequency": "once"
  }' | jq .
echo ""

# Verify update
echo "6️⃣  Verifying update..."
curl -s "$BASE_URL/api/reminders/$REMINDER_ID" | jq .
echo ""

# Delete reminder
echo "7️⃣  Deleting reminder..."
curl -s -X DELETE "$BASE_URL/api/reminders/$REMINDER_ID" | jq .
echo ""

# Verify deletion
echo "8️⃣  Verifying deletion..."
curl -s "$BASE_URL/api/reminders/$REMINDER_ID" | jq .
echo ""

echo "✅ Tests completed!"
