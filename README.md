# CosmWasm Starter Pack

This is a template to build smart contracts in Rust to run inside a
[Cosmos SDK](https://github.com/cosmos/cosmos-sdk) module on all chains that enable it.
To understand the framework better, please read the overview in the
[cosmwasm repo](https://github.com/CosmWasm/cosmwasm/blob/master/README.md),
and dig into the [cosmwasm docs](https://www.cosmwasm.com).
This assumes you understand the theory and just want to get coding.

## Creating a new repo from template

Assuming you have a recent version of Rust and Cargo installed
(via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and cargo-run-script.
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:

**Latest**

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME
```

For cloning minimal code repo:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME -d minimal=true
```

You will now have a new folder called `PROJECT_NAME` (I hope you changed that to something else)
containing a simple working contract and build system that you can customize.

## Create a Repo

After generating, you have a initialized local git repo, but no commits, and no remote.
Go to a server (eg. github) and create a new upstream repo (called `YOUR-GIT-URL` below).
Then run the following:

```sh
# this is needed to create a valid Cargo.lock file (see below)
cargo check
git branch -M main
git add .
git commit -m 'Initial Commit'
git remote add origin YOUR-GIT-URL
git push -u origin main
```

## CI Support

We have template configurations for both [GitHub Actions](.github/workflows/Basic.yml)
and [Circle CI](.circleci/config.yml) in the generated project, so you can
get up and running with CI right away.

One note is that the CI runs all `cargo` commands
with `--locked` to ensure it uses the exact same versions as you have locally. This also means
you must have an up-to-date `Cargo.lock` file, which is not auto-generated.
The first time you set up the project (or after adding any dep), you should ensure the
`Cargo.lock` file is updated, so the CI will test properly. This can be done simply by
running `cargo check` or `cargo unit-test`.

## Using your project

Once you have your custom repo, you should check out [Developing](./Developing.md) to explain
more on how to run tests and develop code. Or go through the
[online tutorial](https://docs.cosmwasm.com/) to get a better feel
of how to develop.

[Publishing](./Publishing.md) contains useful information on how to publish your contract
to the world, once you are ready to deploy it on a running blockchain. And
[Importing](./Importing.md) contains information about pulling in other contracts or crates
that have been published.

Please replace this README file with information about your specific project. You can keep
the `Developing.md` and `Publishing.md` files as useful references, but please set some
proper description in the README.







------------------------------------------------------------------------------------------------
wasmd keys add Alice
wasmd keys add Bob

- name: Alice
  type: local
  address: wasm1u9p429pnjqc0pcfd7chrm3059rg8prhajqpa9d
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"Aih/gsNrjiM3Q2Yv/ukjYV5CkqQRUmvHFcFAEqLSwA1T"}'
  mnemonic: ""


**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

aerobic essence elevator hard vague milk daughter real weekend fantasy hour vote nerve wish trick length olive idea corn danger garment actor dignity muscle

- name: Bob
  type: local
  address: wasm1q5ehwud7px94qxpsa8r5l2a9shqm9vrqqg88e6
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AhAiZzzn09PllgIcvWcTCXQBrH7hCR5KMtHXpGU8at4A"}'
  mnemonic: ""


**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

divert arch scorpion enable neck honey include meadow metal follow flat destroy panic boost festival intact angry suggest water health horse debris depart engine


----------------------------------------------------------------------------------------
GET ADDRESS

wasmd keys show Alice -a


SEND TOKEN from my_wallet
wasmd tx bank send my_wallet <Alice_ADDRESS> 100000000stake --chain-id localnet --broadcast-mode block --gas auto --gas-adjustment 1.5 --fees 5000stake

Check balance
wasmd query bank balances <Alice_ADDRESS>









--------------------------------------------------------------------------------------------------------=
Dưới đây là danh sách tổng hợp các dòng lệnh CLI quan trọng để thao tác với người dùng (Alice, Bob) trong hệ thống staking dựa trên Cosmos SDK (với wasmd). Các lệnh này bao gồm tạo tài khoản, gửi token, staking, và kiểm tra thông tin.

1. Tài khoản và quản lý ví
Tạo ví người dùng
Tạo ví mới cho Alice hoặc Bob:

bash
Copy code
wasmd keys add Alice
wasmd keys add Bob
Xem danh sách các ví
bash
Copy code
wasmd keys list
Xem thông tin địa chỉ ví
Lấy địa chỉ ví:

bash
Copy code
wasmd keys show Alice -a
wasmd keys show Bob -a
Khôi phục ví từ cụm từ khóa (mnemonic)
bash
Copy code
wasmd keys add Alice --recover
2. Quản lý token
Gửi token giữa các tài khoản
Gửi 100000 stake từ my_wallet đến tài khoản Alice:

bash
Copy code
wasmd tx bank send my_wallet $(wasmd keys show Alice -a) 100000stake --chain-id localnet --fees 500stake --gas auto -y
Gửi 200000 stake từ my_wallet đến tài khoản Bob:

bash
Copy code
wasmd tx bank send my_wallet $(wasmd keys show Bob -a) 200000stake --chain-id localnet --fees 500stake --gas auto -y
Kiểm tra số dư tài khoản
Kiểm tra số dư ví Alice:

bash
Copy code
wasmd query bank balances $(wasmd keys show Alice -a)
Kiểm tra số dư ví Bob:

bash
Copy code
wasmd query bank balances $(wasmd keys show Bob -a)
3. Validator và staking
Tạo validator (dành cho người quản trị)
bash
Copy code
wasmd tx staking create-validator \
  --amount=100000stake \
  --pubkey=$(wasmd tendermint show-validator) \
  --moniker="my-node" \
  --chain-id=localnet \
  --commission-rate="0.10" \
  --commission-max-rate="0.20" \
  --commission-max-change-rate="0.01" \
  --min-self-delegation="1" \
  --from=my_wallet \
  --fees=500stake \
  --gas auto -y
Xem danh sách validator
bash
Copy code
wasmd query staking validators --chain-id localnet --output json
4. Staking (delegate, undelegate)
Alice và Bob stake token
Alice stake 10000 stake vào validator:

bash
Copy code
wasmd tx staking delegate \
  <VALIDATOR_ADDRESS> 10000stake \
  --from Alice \
  --chain-id localnet \
  --fees 500stake \
  --gas auto -y
Bob stake 20000 stake vào validator:

bash
Copy code
wasmd tx staking delegate \
  <VALIDATOR_ADDRESS> 20000stake \
  --from Bob \
  --chain-id localnet \
  --fees 500stake \
  --gas auto -y
Kiểm tra danh sách staking của validator
bash
Copy code
wasmd query staking delegations-to <VALIDATOR_ADDRESS> --chain-id localnet --output json
Kiểm tra staking của từng người dùng
Alice:

bash
Copy code
wasmd query staking delegations $(wasmd keys show Alice -a) --chain-id localnet --output json
Bob:

bash
Copy code
wasmd query staking delegations $(wasmd keys show Bob -a) --chain-id localnet --output json
Unstake (undelegate) token
Alice rút 5000 stake:

bash
Copy code
wasmd tx staking undelegate \
  <VALIDATOR_ADDRESS> 5000stake \
  --from Alice \
  --chain-id localnet \
  --fees 500stake \
  --gas auto -y
Bob rút 10000 stake:

bash
Copy code
wasmd tx staking undelegate \
  <VALIDATOR_ADDRESS> 10000stake \
  --from Bob \
  --chain-id localnet \
  --fees 500stake \
  --gas auto -y
5. Phần thưởng staking
Kiểm tra phần thưởng staking của từng người dùng
Alice:

bash
Copy code
wasmd query distribution rewards $(wasmd keys show Alice -a) --chain-id localnet --output json
Bob:

bash
Copy code
wasmd query distribution rewards $(wasmd keys show Bob -a) --chain-id localnet --output json
Claim phần thưởng staking
Alice claim phần thưởng:

bash
Copy code
wasmd tx distribution withdraw-rewards <VALIDATOR_ADDRESS> \
  --from Alice \
  --chain-id localnet \
  --fees 500stake \
  --gas auto -y
Bob claim phần thưởng:

bash
Copy code
wasmd tx distribution withdraw-rewards <VALIDATOR_ADDRESS> \
  --from Bob \
  --chain-id localnet \
  --fees 500stake \
  --gas auto -y
6. Truy vấn thông tin blockchain
Kiểm tra thông tin node
bash
Copy code
wasmd status
Kiểm tra thông tin block
bash
Copy code
wasmd query block <BLOCK_HEIGHT>
Kiểm tra giao dịch
bash
Copy code
wasmd query tx <TX_HASH>
Lưu ý
<VALIDATOR_ADDRESS>: Địa chỉ validator, ví dụ wasmvaloper1tkrrhp6navpnw222kegjh6pjjcnwvws5kpxt6c. (lay bang cau lenh: wasmd query staking validators --chain-id localnet --output json
)
<CHAIN_ID>: Tên của chain (ở đây là localnet).
Điều chỉnh phí giao dịch (--fees) và gas (--gas) nếu cần thiết.
Nếu bạn cần thêm bất kỳ lệnh nào khác, hãy cho tôi biết! 😊

============================================================================================================


Dưới đây là danh sách các bước và câu lệnh tổng hợp để triển khai (deploy) một staking contract và kiểm tra thông tin liên quan:

1. Khởi tạo blockchain (localnet)
Trước khi deploy, cần khởi tạo blockchain với wasmd.

Câu lệnh:
bash
Copy code
wasmd init my-node --chain-id localnet
wasmd keys add my_wallet
wasmd keys add Alice
wasmd keys add Bob
wasmd add-genesis-account $(wasmd keys show my_wallet -a) 1000000000stake
wasmd add-genesis-account $(wasmd keys show Alice -a) 500000stake
wasmd add-genesis-account $(wasmd keys show Bob -a) 500000stake
wasmd gentx my_wallet 100000stake --chain-id localnet
wasmd collect-gentxs
wasmd validate-genesis
2. Chạy blockchain
Khởi động blockchain localnet.

bash
Copy code
wasmd start
3. Chuẩn bị file hợp đồng
Compile hợp đồng với Docker optimizer.
Câu lệnh:
bash
Copy code
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.16.0
Sau khi chạy xong, file .wasm đã được tối ưu hóa sẽ nằm trong thư mục artifacts/.

4. Upload và deploy hợp đồng
Bước 1: Tải file hợp đồng lên blockchain
bash
Copy code
wasmd tx wasm store artifacts/staking_contract.wasm --from my_wallet --chain-id localnet --broadcast-mode block --gas auto --gas-adjustment 1.5 --fees 5000stake -y
Bước 2: Lấy mã code_id
Sau khi upload thành công, kiểm tra code_id của hợp đồng:

bash
Copy code
wasmd query wasm list-code --chain-id localnet --output json | jq
Bước 3: Khởi tạo hợp đồng (Instantiate)
bash
Copy code
wasmd tx wasm instantiate <CODE_ID> '{"denom":"stake", "validator":"<VALIDATOR_ADDRESS>"}' --from my_wallet --label "Staking Contract" --chain-id localnet --broadcast-mode block --gas auto --gas-adjustment 1.5 --fees 5000stake -y
Thay <CODE_ID> bằng ID của hợp đồng và <VALIDATOR_ADDRESS> bằng địa chỉ validator.

Bước 4: Lấy địa chỉ contract
bash
Copy code
wasmd query wasm list-contract-by-code <CODE_ID> --chain-id localnet --output json
5. Gửi staking và kiểm tra
Bước 1: Alice và Bob staking
Alice và Bob gửi stake tới hợp đồng:

bash
Copy code
wasmd tx staking delegate <VALIDATOR_ADDRESS> 10000stake --from Alice --chain-id localnet --gas auto --gas-adjustment 1.5 --fees 500stake -y
wasmd tx staking delegate <VALIDATOR_ADDRESS> 20000stake --from Bob --chain-id localnet --gas auto --gas-adjustment 1.5 --fees 500stake -y
Bước 2: Kiểm tra stake của mỗi người
Kiểm tra thông tin staking của từng người:

bash
Copy code
wasmd query staking delegations $(wasmd keys show Alice -a) --chain-id localnet --output json | jq
wasmd query staking delegations $(wasmd keys show Bob -a) --chain-id localnet --output json | jq
Bước 3: Kiểm tra danh sách tất cả người staking
bash
Copy code
wasmd query staking delegations-to <VALIDATOR_ADDRESS> --chain-id localnet --output json | jq
Bước 4: Kiểm tra lợi nhuận staking
Lợi nhuận staking của mỗi người có thể được kiểm tra bằng lệnh:

bash
Copy code
wasmd query distribution rewards $(wasmd keys show Alice -a) --chain-id localnet --output json | jq
wasmd query distribution rewards $(wasmd keys show Bob -a) --chain-id localnet --output json | jq
6. Rút phần thưởng staking
Nếu muốn rút phần thưởng staking:

bash
Copy code
wasmd tx distribution withdraw-rewards <VALIDATOR_ADDRESS> --from Alice --chain-id localnet --gas auto --fees 500stake -y
wasmd tx distribution withdraw-rewards <VALIDATOR_ADDRESS> --from Bob --chain-id localnet --gas auto -fees 500stake -y


