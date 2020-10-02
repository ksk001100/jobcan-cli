# jobcan cli

## install

```bash
$ git clone https://github.com/ksk001100/jobcan-cli
$ cd jobcan-cli
$ cargo install --path .
```

## usage
環境変数`JOBCAN_EMAIL`と`JOBCAN_PASSWORD`にそれぞれジョブカンのログインメールアドレスとパスワードを設定するか、オプション引数`--email`と`--password`を指定してください。
また、環境変数よりオプション引数のほうが優先されます。

```bash
$ jobcan --help
Name
        jobcan

Author:
        ksk001100 <hm.pudding0715@gmail.com>

Description:
        ジョブカン勤怠管理の打刻 CLI

Usage:
        jobcan
        jobcan --email(-e) [email]
        jobcan --password(-p) [password]


Commands:
        status : jobcan status
                 jobcan status(s) --email(-e) [email]
                 jobcan status(s) --password(-p) [password]

        pto    : jobcan pto [start_date] [end_date] [reason]
                 jobcan pto(p) [start_date] [end_date] [reason] --email(-e) [email]
                 jobcan pto(p) [start_date] [end_date] [reason] --password(-p) [password]

Version:
        0.1.0



$ jobcan
未出勤 -> 勤務中

$ jobcan --email test@test.com --password hogefuga
勤務中 -> 退室中

$ jobcan status
ステータス : 退室中

$ jobcan -e test@test.com -p hogefuga
退室中 -> 勤務中

$ jobcan s -e test@test.com -p hogefuga
ステータス : 勤務中

$ jobcan pto "2020-10-10" "2020-10-10"
有給休暇申請 : 2020-10-10 ~ 2020-10-10

$ jobcan p "2020-07-20" "2020-07-23" "夏休み"
有給休暇申請 : 2020-07-20 ~ 2020-07-23
```
