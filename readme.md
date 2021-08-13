# Octoack

A tool to query GitHub repos and send Slack notifications when new PRs are created by a specified list of users.



## Setup

1. Create Slack app

2. Edit config octoack.toml add Slack webhook URL, users, repo

3. Create sqlite database: octoack.sqlite

```
CREATE TABLE notices (url TEXT, timestamp INTEGER);
```

4. Use cron to run on your desired frequency



**Dependencies** - If running on Linux you will need to install

```
apt install sqlite3 libsqlite3-dev libssl-dev
```
