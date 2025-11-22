#!/bin/bash

# Script to clean up ports and start dev server safely

echo "🔍 Checking for processes on ports 1420 and 1421..."

# Kill processes on port 1420
if lsof -ti:1420 > /dev/null 2>&1; then
  echo "⚠️  Port 1420 is in use. Killing processes..."
  lsof -ti:1420 | xargs kill -9 2>/dev/null
  echo "✅ Port 1420 cleared"
else
  echo "✓ Port 1420 is available"
fi

# Kill processes on port 1421 (HMR)
if lsof -ti:1421 > /dev/null 2>&1; then
  echo "⚠️  Port 1421 is in use. Killing processes..."
  lsof -ti:1421 | xargs kill -9 2>/dev/null
  echo "✅ Port 1421 cleared"
else
  echo "✓ Port 1421 is available"
fi

echo ""
echo "🚀 Starting development server..."
echo ""

# Start the dev server
npm run tauri dev
