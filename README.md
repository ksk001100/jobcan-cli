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
Name:
	jobcan

Author:
	ksk001100 <hm.pudding0715@gmail.com>

Description:
	ジョブカン勤怠管理の打刻CLI

Usage:
	jobcan
	jobcan --email(-e) [email]
	jobcan --password(-p) [password]

Version:
	0.1.0

$ jobcan
未出勤 -> 勤務中

$ jobcan --email test@test.com --password hogefuga
勤務中 -> 退室中

$ jobcan -e test@test.com -p hogefuga
退室中 -> 勤務中
```