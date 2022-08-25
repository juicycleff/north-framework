# Contribution Guidelines

## Commit messages

Please use [conventional commits](https://www.conventionalcommits.org) (also known as semantic commits) to ensure consistent and descriptive commit messages when submitting PRs.

- `fix`: bug fixes, e.g. fix crash due to deprecated method.
- `feat`: new features, e.g. add new method to the module.
- `refactor`: code refactor, e.g. migrate from class components to hooks.
- `docs`: changes into documentation, e.g. add usage example for the module..
- `chore`: tooling changes, e.g. change CI config.

Our pre-commit hooks verify that your commit message matches this format when committing.

## General guidelines

Every example should follow a number of guidelines:

- Can be minimal or of complex/production-ready.
- Easily extensible.


To analyse coding style, run the following:

```sh
flutter analyze lib test
```

To fix formatting errors, run the following:

```sh
flutter format --set-exit-if-changed lib test
```

### Linting

[Very Good Analysis](https://verygood.ventures/blog/introducing-very-good-analysis)

Our pre-commit hooks verify that the linter and tests pass when committing.

When you're sending a pull request:

- Prefer small pull requests focused on one change.
- Verify that linters and tests are passing.
- Review the documentation to make sure it looks good.
- Follow the pull request template when opening a pull request.
- For pull requests that change the API or implementation, discuss with maintainers first by opening an issue.


## Task management

At Wakflo we use [Linear](https://notion.app/wakflo/team/MOBILE/active) for ticket-tracking. Each cycle name in Linear defines the sprint goals.

## Setting up the project

### Environment variables
Create a `.Env` File at the root directory of the project and add the following variables, replace the `[]` with the actual values.
```text

```

## Workflow

We use a slightly modified version of [Gitflow branching model](https://nvie.com/posts/a-successful-git-branching-model/) - we use short-lived feature branches to minimize deviation and deploy to production directly from the `master` branch. This sort of worflow is an extension of [trunk-based](https://www.atlassian.com/continuous-delivery/continuous-integration/trunk-based-development) development.

We have coined this workflow: `Trunk-Based Gitflow`.

### Branches

- `main`: The production branch of the repo. It is always in production-ready state. The release version tags are directly on this branch and represent the respective deployments in production.
- `development`: The main development branch. All features are merged to this branch via a Pull Request. This branch is always automatically deployed to the development preview locations such as [https://dev-api.wakflo.me](https://dev-api.wakflo.me) and [https://dev-mainframe.wakflo.me](https://dev-mainframe.wakflo.me). The development preview deployments all use a special `wakflo-dev` backend database and use the Solana devnet as the blockchain source.
- `staging`: An optional staging branch for when a feature needs to be on production credentials for testing but is not ready for merging into master. For example, a new smart contract deployed on mainnet.
- `feature/*`: The building blocks of our workflow. Each Linear ticket should have its own feature branch, which mentions the ticket ID.
- `hotfix/*`: When something goes wrong in production, a hotfix is needed. This is created on the fly and merged directly into both `master` and `develop` branch.

### Begin Work

As you begin on a ticket, you should use Linear's button that copies the git branch name. For example `feature/web-821-as-a-developer-i-want-more-informative`. Then you should create the branch this way: `git checkout -b feature/web-821-as-a-developer-i-want-more-informative` - this creates a new feature branch where you may start committing code.

### **Long-living Feature Development**

Often times, development on a new feature can span over many sprints. Because we subscribe to trunk-based development, commits that are part of the feature will get merged into our main trunk (`develop`) which will eventually make its way into production (`master`). These work-in-progress features should not get exposed to our end-users. Therefore, in order to exclude these parts of the app from reaching production, we use feature-flags to remove code that should not yet get outputted. This is made possible by using the [ifdef-loader](https://www.npmjs.com/package/ifdef-loader) webpack loader. Please read its documentation for more details on how to use it.


### Commits

We follow the [Conventional Commits Specification](https://www.conventionalcommits.org/en/V1.0.0/) to standardize our commit messages, making it easier to automate releases and changelogs based on commit types and scoping.

For the scope, we enforce using the Linear ticket ID for better standardization. For example, for the commit that created this documentation, the message is `docs(WEB-821): update readme and contributing.md`

Commit messages are actively linted by a pre-commit hook.

### Pull Requests

Once your feature is complete and ready for review, you may push to origin and submit a pull request to `develop`. Please note that this should be the only branch that you submit pull requests to. The title of your pull request should include the Linear ticket ID for ease of automating the state of your tickets. This is usually the default title given by Github if your commit messages have followed convention.

Your code will now be reviewed. Once a reviewer approves the PR, you or the reviewer may merge it into the `develop` branch. We strictly merge using the `no-ff` method (also known as merge commit.) Your contribution is now complete!

### Squashing Commits

From time to time, you may be required to squash commits if there are too many for a Pull Request. We are not extremely strict about squashing but the rule of thumb is that each commit should pass tests and be deployable. That usually means `wip` commits should be squashed. There are many ways to do this. An online search would suggest interactive rebasing but we have frequently found that doing a soft reset and commit is easier to follow and results in fewer errors. Here's an example of 3 commits being squshed into one:

```
git reset --soft HEAD~3
git commit -m "docs(WEB-821): update readme and contributing.md"
```

### Integrating Target Branch Changes

Whether due to merge conflicts or a deviation from dependent features due to length of time, you will find yourself from time to time needing to incorporate changes others have made to the `develop` branch into your feature branch.

While the naive approach is to merge develop back into the feature branch, this is not recommended due to the messiness that it could cause for the commit history. We want to maximize readability of our commit history.

At Wakflo we prefer to use `git rebase`. This simulates that your commits all happen after the current `HEAD` of the `develop` branch and results in a much cleaner git history in the long run.

If there are conflicts during rebasing, the _Interactive Rebase_ mode is activated. Each commit is replayed on top of `HEAD` one by one. After you solve the conflict in each commit, you should execute `git rebase --continue` to go on to the next one, until all conflicts are resolved and the entire commit history of your feature branch has been replayed on top of the latest commit in the `develop` branch.

### Release

Once the `develop` branch is ready for a release to production, the repo maintainer can create a release by running the following command at the root of the repo:

`cargo make release`

This will automatically bump the correct version based on the type of commits in this release, updates the `CHANGELOG.md`, create a new `tag`, add a new release commit, and creates a pull request to `master` with the latest version changes as its PR Description. This is why it's important to follow the conventional commit specification so that our release details can be as accurate and automated as possible.

If manual control of what version number to bump is desired, run one of the following commands ([Semantic Versioning](https://semver.org/) is followed here):

- `git tag [version_number]` - for all releases.

If a mistake was made, you can revert the commit and delete the tag by running:

```
git reset --hard HEAD~1
git tag -d <tag-name>
```

If everything looks good, you can create a pull request to `main` with the latest version changes as its PR Description.