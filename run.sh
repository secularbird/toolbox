#!/bin/bash
# Quick start script for Disk Scanner

echo "🚀 Starting Disk Space Scanner..."
echo ""

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Start the app
echo "🎯 Launching application..."
npm run tauri dev
