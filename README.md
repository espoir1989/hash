#solana测试网撸毛

###领水
https://www.testnetfaucet.org/
https://faucet.quicknode.com/solana/testnet

###构建
cargo build --release

###配置
cd target/relese
solana-keygen new -o id1.json
solana-keygen new -o id2.json
solana-keygen pubkey id1.json
solana-keygen pubkey id2.json
用生成的地址去领水

###运行
./hash


