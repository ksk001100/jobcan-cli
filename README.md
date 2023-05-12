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
        ジョブカン CLI

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

```

### 打刻
```bash
$ jobcan
未出勤 -> 勤務中

$ jobcan --email test@test.com --password hogefuga
勤務中 -> 退室中

$ jobcan -e test@test.com -p hogefuga
退室中 -> 勤務中
```

### 打刻ステータス確認
```bash
$ jobcan status
ステータス : 退室中

$ jobcan s -e test@test.com -p hogefuga
ステータス : 勤務中
```

### 有給休暇申請
```bash
$ jobcan pto "2020-10-10" "2020-10-10"
有給休暇申請 : 2020-10-10 ~ 2020-10-10

$ jobcan p "2020-07-20" "2020-07-23" "夏休み"
有給休暇申請 : 2020-07-20 ~ 2020-07-23
```

### トラブルシューティング

#### M1 Macでビルド時にエラーが発生する場合

> **Warning**
> Apple M1 環境でビルドする場合、`Xcode command line tools`をインストールしていないと、ビルド実行時に以下のエラーが発生します

```
error: could not compile `libc` due to previous error
```

そのため、ビルドする前に以下コマンドを実行する必要があります

```
xcode-select --install
```

#### ビルド時のフェッチに失敗する場合

`cargo`がデフォルトで`ssh-agent`を使用してフェッチしているので、Rust のインストール直後にビルドすると、以下のようにエラーが発生することがあります

```
cargo install --path .
...
Caused by:
  ERROR: You're using an RSA key with SHA-1, which is no longer allowed. Please use a newer client or a different key type.
  Please see https://github.blog/2021-09-01-improving-git-protocol-security-github/ for more information.

  ; class=Ssh (23); code=Eof (-20)
```

許容されている暗号化方式で SSH 鍵を再作成する方法もありますが、以下のコマンドを実行して git 経由でフェッチするように変更することもできます

```
cat <<EOF >> ~/.cargo/config
[net]
git-fetch-with-cli = true
EOF
```
