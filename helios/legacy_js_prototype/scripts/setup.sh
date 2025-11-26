#!/bin/bash
#
# Setup script for GitHub Portfolio
# Usage: ./scripts/setup.sh
#

set -e

echo "🚀 Setting up GitHub Portfolio..."

# Check for required tools
command -v node >/dev/null 2>&1 || { echo "Error: Node.js not installed"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "Error: npm not installed"; exit 1; }
command -v python3 >/dev/null 2>&1 || { echo "Error: Python3 not installed"; exit 1; }

# Install frontend dependencies
echo "📦 Installing frontend dependencies..."
npm install

# Install backend dependencies
echo "📦 Installing backend dependencies..."
cd backend
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt 2>/dev/null || pip install requests python-dotenv

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file..."
    cat > .env << EOF
# GitHub
GITHUB_TOKEN=your_github_personal_access_token
GITHUB_USERNAME=your_username

# Authentication
NEXTAUTH_URL=http://localhost:3000
NEXTAUTH_SECRET=$(openssl rand -base64 32)
GITHUB_CLIENT_ID=your_github_oauth_app_client_id
GITHUB_CLIENT_SECRET=your_github_oauth_app_client_secret

# Database
DATABASE_URL=postgresql://user:password@localhost:5432/github_portfolio

# Environment
NODE_ENV=development
EOF
    echo "⚠️  Please update .env with your actual values"
fi

echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Update .env file with your GitHub credentials"
echo "2. Run: npm run dev"
echo "3. Open: http://localhost:3000"

