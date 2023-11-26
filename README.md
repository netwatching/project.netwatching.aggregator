# Docker Template Repository

Welcome to NetWatching's Docker Template Repository! This template streamlines the setup of Docker projects with best practices and essential workflows pre-configured.

## Features

- **Dockerfile Health Check**: Ensures the presence of a health check in the Dockerfile.
- **Automated Docker Build and Push**: Builds and pushes Docker images to GitHub Container Registry (GHCR) for `amd64` and `arm64` architectures on tagged commits.
- **Conventional Commits**: Strict adherence to the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format is enforced for all commit messages.
- **Release Please Integration**: Automated versioning and release note generation via [Release Please](https://github.com/googleapis/release-please).
- **Dependabot**: Configurable automatic dependency updates through `dependabot.yml`.

## Getting Started

1. **Create Your Repository**: Use this template to create a new repository on GitHub.
2. **Configure Repository Settings**:
   - Enable branch protection rules on the default branch to prevent direct pushes and force pushes. All changes should go through `feat/` or `fix/` branches.
   - Disable rebase and squash merging.
   - Enable automatic deletion of head branches post-merge.
   - Require contributors to review pull requests before merging. At least 2 developers should review code before merges are allowed.
   - Only allow pull requests which pass all checks (no workflows have to pass).
3. **Initialize Your Project**:
   - Clone your new repository.
   - Run the `init.ps1` script for initial setup, including Release Please configuration and README file generation (a bash version of this script will follow soon).

## Important Notes

- **Commit Messages**: Commit messages must follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format.
- **Tagging and Releases**: Do not create tags or releases manually. Release Please will handle versioning and release notes.
- **Dependabot Customization**: Adjust the `dependabot.yml` in your repository to match your project's language and dependencies.

## Contribution Guidelines

- **Work on Branches**: Make changes in feature (`feat/`) or bugfix (`fix/`) branches.
- **Follow Conventional Commits**: Ensure all commits adhere to the conventional commits format.
- **Use Pull Requests**: Merge changes into the main branch via pull requests.

## License

Licensed under the [Apache License](https://www.apache.org/licenses/LICENSE-2.0). See the `LICENSE` file for details.

---

For support or to report issues, open an issue in the repository or contact the NetWatching team.
