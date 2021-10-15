# Octoack

A tool to query GitHub repos and send Slack notifications when new PRs are created by a specified list of users.

GitHub has an [official Slack integration bot](https://github.com/integrations/slack) that allows posting similar updates, however it does not allow for filtering by user. See open [issue #1215](https://github.com/integrations/slack/issues/1215). If you want notices from a small repository and don't mind all PRs, or filterable by label, then their bot is a better an option.

If you want to watch a large repository, and filter by user, then continue on.


## Setup

1. Create Slack app

2. Edit config octoack.toml add Slack webhook URL, users, repo

3. Create sqlite database: octoack.sqlite

```sql
CREATE TABLE notices (url TEXT, timestamp INTEGER);
```

4. Use cron to run on your desired frequency



**Dependencies** - If running on Linux you will need to install

```
apt install sqlite3 libsqlite3-dev libssl-dev pkg-config
```


## Sample Config

The configuration file is in TOML format.


```toml
# Slack app Webhook URL
# Create new Slack app, and copy URL from "Incoming Webhooks"

slack_url="https://hooks.slack.com/services/ABCDEFGHIJ/1234567890"

# Users are github username, slack username
users = [
	{ github = "mkaz", slack = "mkaz" },
	{ github = "foo", slack = "bar" },
]

# Repositories
repos = [ "owner/repo", "owner/repo2" ]
```
