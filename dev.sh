#!/bin/bash

# Script to clean up ports and start dev server safely

echo "🔍 Checking for processes on ports 17520 and 17521..."

# Kill processes on port 17520
if lsof -ti:17520 > /dev/null 2>&1; then
  echo "⚠️  Port 17520 is in use. Killing processes..."
  lsof -ti:17520 | xargs kill -9 2>/dev/null
  echo "✅ Port 17520 cleared"
else
  echo "✓ Port 17520 is available"
fi

# Kill processes on port 17521 (HMR)
if lsof -ti:17521 > /dev/null 2>&1; then
  echo "⚠️  Port 17521 is in use. Killing processes..."
  lsof -ti:17521 | xargs kill -9 2>/dev/null
  echo "✅ Port 17521 cleared"
else
  echo "✓ Port 17521 is available"
fi

echo ""
echo "🚀 Starting development server..."
echo ""

# Start the dev server
npm run tauri dev
