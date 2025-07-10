#!/bin/bash

# =============================
# Laune App Launcher Script
# =============================

# Terminal colors for better readability
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Define paths
PROJECT_ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BACKEND_DIR="$PROJECT_ROOT/backend"
FRONTEND_DIR="$PROJECT_ROOT/frontend"

# Print header
echo -e "${BLUE}=================================${NC}"
echo -e "${BLUE}     Launching Laune App        ${NC}"
echo -e "${BLUE}=================================${NC}"

# Check for required tools
check_requirements() {
  echo -e "\n${YELLOW}Checking requirements...${NC}"
  
  # Check for cargo
  if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found! Please install Rust from https://rustup.rs/${NC}"
    exit 1
  else
    echo -e "${GREEN}✓ Rust/Cargo found${NC}"
  fi
  
  # Check for pnpm
  if ! command -v pnpm &> /dev/null; then
    echo -e "${YELLOW}⚠️ pnpm not found! Attempting to install pnpm...${NC}"
    npm install -g pnpm
    if [ $? -ne 0 ]; then
      echo -e "${RED}❌ Failed to install pnpm. Please install it manually with 'npm install -g pnpm'${NC}"
      exit 1
    fi
  else
    echo -e "${GREEN}✓ pnpm found${NC}"
  fi
}

# Start the backend server
start_backend() {
  echo -e "\n${YELLOW}Starting Rust backend server...${NC}"
  cd "$BACKEND_DIR" || { echo -e "${RED}❌ Backend directory not found!${NC}"; exit 1; }
  
  # Kill any existing backend process
  pkill -f "target/debug/laune-backend" > /dev/null 2>&1
  
  # Start the backend server in the background
  cargo run &
  BACKEND_PID=$!
  
  echo -e "${GREEN}✓ Backend server started with PID: $BACKEND_PID${NC}"
  echo -e "${GREEN}✓ Backend URL: http://localhost:8080${NC}"
  
  # Give the backend time to start up
  echo -e "${YELLOW}Waiting for backend to initialize...${NC}"
  sleep 3
  
  # Check if backend is responding
  if curl -s http://localhost:8080/health > /dev/null; then
    echo -e "${GREEN}✓ Backend health check passed${NC}"
  else
    echo -e "${YELLOW}⚠️ Backend may still be starting up...${NC}"
  fi
}

# Start the frontend server
start_frontend() {
  echo -e "\n${YELLOW}Starting React frontend server...${NC}"
  cd "$FRONTEND_DIR" || { echo -e "${RED}❌ Frontend directory not found!${NC}"; exit 1; }
  
  # Kill any existing frontend process
  pkill -f "vite" > /dev/null 2>&1
  
  # Ensure dependencies are installed
  if [ ! -d "node_modules" ]; then
    echo -e "${YELLOW}Installing frontend dependencies...${NC}"
    pnpm install
  fi
  
  # Start the frontend development server
  pnpm dev &
  FRONTEND_PID=$!
  
  echo -e "${GREEN}✓ Frontend server started with PID: $FRONTEND_PID${NC}"
  echo -e "${GREEN}✓ Frontend URL: http://localhost:3000${NC}"
  
  # Wait a moment for the server to start
  sleep 3
}

# Open the app in the default browser
open_app() {
  echo -e "\n${YELLOW}Opening Laune in your browser...${NC}"
  sleep 2
  
  # Use the appropriate open command based on OS
  if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    open http://localhost:3000
  elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    xdg-open http://localhost:3000
  elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    # Windows with Git Bash or similar
    start http://localhost:3000
  else
    echo -e "${YELLOW}⚠️ Cannot automatically open browser on this platform. Please open http://localhost:3000 manually.${NC}"
  fi
}

# Handle script termination
cleanup() {
  echo -e "\n${YELLOW}Stopping services...${NC}"
  pkill -f "target/debug/laune-backend" > /dev/null 2>&1
  pkill -f "vite" > /dev/null 2>&1
  echo -e "${GREEN}✓ Services stopped${NC}"
  exit 0
}

# Register the cleanup function to run on script termination
trap cleanup SIGINT SIGTERM EXIT

# Main execution
check_requirements
start_backend
start_frontend
open_app

echo -e "\n${GREEN}==================================${NC}"
echo -e "${GREEN}  Laune app is now running!      ${NC}"
echo -e "${GREEN}  Press Ctrl+C to stop all services${NC}"
echo -e "${GREEN}==================================${NC}"

# Keep the script running to maintain the child processes
wait
