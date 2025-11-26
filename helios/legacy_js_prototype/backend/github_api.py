#!/usr/bin/env python3
"""
GitHub Portfolio - Backend API
Fetches GitHub repos and manages visibility settings
"""

import os
import requests
from typing import List, Dict, Optional
from datetime import datetime

class GitHubPortfolioAPI:
    def __init__(self, github_token: str):
        self.github_token = github_token
        self.api_base = "https://api.github.com"
        self.headers = {
            "Authorization": f"token {github_token}",
            "Accept": "application/vnd.github.v3+json"
        }
    
    def get_all_repos(self, username: str) -> List[Dict]:
        """Fetch all repositories for a user"""
        repos = []
        page = 1
        
        while True:
            url = f"{self.api_base}/user/repos"
            params = {
                "per_page": 100,
                "page": page,
                "sort": "updated",
                "direction": "desc"
            }
            
            response = requests.get(url, headers=self.headers, params=params)
            response.raise_for_status()
            
            page_repos = response.json()
            if not page_repos:
                break
            
            repos.extend(page_repos)
            page += 1
        
        return repos
    
    def get_repo_details(self, repo_name: str) -> Dict:
        """Get detailed information about a repository"""
        url = f"{self.api_base}/repos/{repo_name}"
        response = requests.get(url, headers=self.headers)
        response.raise_for_status()
        return response.json()
    
    def format_repo(self, repo: Dict) -> Dict:
        """Format repository data for display"""
        return {
            "id": repo["id"],
            "name": repo["name"],
            "full_name": repo["full_name"],
            "description": repo.get("description", ""),
            "url": repo["html_url"],
            "language": repo.get("language", "Unknown"),
            "stars": repo["stargazers_count"],
            "forks": repo["forks_count"],
            "visibility": repo["visibility"],  # "public" or "private"
            "created_at": repo["created_at"],
            "updated_at": repo["updated_at"],
            "topics": repo.get("topics", []),
            "default_branch": repo.get("default_branch", "main"),
            "archived": repo.get("archived", False),
            "fork": repo.get("fork", False)
        }
    
    def categorize_repos(self, repos: List[Dict], visibility_settings: Dict) -> Dict:
        """Categorize repos by visibility settings"""
        categorized = {
            "public": [],
            "private": [],
            "invite_only": []
        }
        
        for repo in repos:
            repo_name = repo["full_name"]
            visibility = visibility_settings.get(repo_name, {
                "type": repo["visibility"],  # Default to GitHub visibility
                "invite_code": None
            })
            
            formatted_repo = self.format_repo(repo)
            formatted_repo["display_visibility"] = visibility["type"]
            formatted_repo["invite_code"] = visibility.get("invite_code")
            
            if visibility["type"] == "invite_only":
                categorized["invite_only"].append(formatted_repo)
            elif repo["visibility"] == "private" or visibility["type"] == "private":
                categorized["private"].append(formatted_repo)
            else:
                categorized["public"].append(formatted_repo)
        
        return categorized

def main():
    # Example usage
    github_token = os.getenv("GITHUB_TOKEN")
    if not github_token:
        print("Error: GITHUB_TOKEN environment variable not set")
        return
    
    api = GitHubPortfolioAPI(github_token)
    
    print("Fetching all repositories...")
    repos = api.get_all_repos("Shivam-Bhardwaj")
    
    print(f"\nFound {len(repos)} repositories")
    
    # Example visibility settings (would come from database)
    visibility_settings = {
        # "username/repo": {"type": "public|private|invite_only", "invite_code": "optional"}
    }
    
    categorized = api.categorize_repos(repos, visibility_settings)
    
    print(f"\nPublic: {len(categorized['public'])}")
    print(f"Private: {len(categorized['private'])}")
    print(f"Invite-only: {len(categorized['invite_only'])}")

if __name__ == "__main__":
    main()

