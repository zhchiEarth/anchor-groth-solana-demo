## Build and Deploy
```shell
anchor build && anchor deploy
```

已经部署到了 devnet. 上面的不需要操作.

programs_id: 3dg1wKkhN4WexPV1z9Q9hJnSoqB5j3pCboBGm36UC129

## KEY

将 env.example填上自己的私钥, 修改 env.example 为 .env

```shell
mv env.example .env
```

## Test
替换 cli/proof 下的文件, 替换成你要测试的参数，运行下面的命令即可测试.

```shell
cargo run -p cli
```