# lesson6

## 第一部分： 为template添加Benchmark

为template模块的do_something添加benchmark用例，并且将benchmark运行结果转换为对应的权重定义；

### 修改pallets-template

在/pallets/template/src/lib.rs中添加/修改如下代码

```rust
#![cfg_attr(not(feature = "std"), no_std)]
--- snip ---
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    pub use crate::weights::WeightInfo;
// --- snip ---
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        ype WeightInfo: WeightInfo;
    }
// --- snip ---
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::do_something(*something))]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
// --- snip ---
            Ok(())
        }
// --- snip ---
    }
}

```

### 修改mock

在/pallets/template/src/mock.rs中添加/修改如下代码

```rust
// --- snip ---
impl pallet_template::Config for Test {
	type Event = Event;
	type WeightInfo = pallet_template::weights::SubstrateWeight<Test>;
}
// --- snip ---
```

### 修改runtime

在/runtime/src/lib.rs中添加/修改如下代码

```rust
// --- snip ---
impl pallet_template::Config for Runtime {
    type Event = Event;
    type WeightInfo = pallet_template::weights::SubstrateWeight<Runtime>;
}
// --- snip ---
```

###  编译 benchmark

```bash
cd node 
cargo build --release --features runtime-benchmarks
```

### 运行测试，并生成weights文件

 ```bash
 ./target/release/node-template benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_template --extrinsic do_something --steps 20 --repeat 50 --template=.maintain/frame-weight-template.hbs --output=./pallets/template/src/weights.rs
 ```

![Snipaste_2021-10-07_23-31-29.png](https://i.loli.net/2021/10/07/uOWA49BUCTGqVrw.png)

### 运行test

```bash
argo test -p pallet-template --all-features
```

![Snipaste_20.png](https://i.loli.net/2021/10/07/NirDZFUVeH5Sp4n.png)

### 重新编译node

```bash
cargo build --release
```



## 第二部分： 生成ChainSpec文件

使用编译好的node-template节点，生成Chain Spec文件。

### 生成customSpec.json

```bash
./target/release/node-template build-spec --disable-default-bootnode --chain local > customSpec.json
```

###  生成Node01和Node02密钥

Node01

```bash
subkey generate --scheme sr25519

Secret phrase:       avoid fiscal stuff nose swift section famous treat blouse mad pitch ladder
  Secret seed:       0xffff237ecb20b767f8c96bcfaaef30ff1e173e5a48e5664d2d5a3bfdf795b856
  Public key (hex):  0x46cf644fa3667b6a76190eddddf92af41a09195ee35b1ff2688f516376df8c7b
  Account ID:        0x46cf644fa3667b6a76190eddddf92af41a09195ee35b1ff2688f516376df8c7b
  Public key (SS58): 5DfYpMmVNfrQbv2TDkCVSQAc4bYs4UNrp5EghhWTxgN8BMLg
  SS58 Address:      5DfYpMmVNfrQbv2TDkCVSQAc4bYs4UNrp5EghhWTxgN8BMLg
  
  
subkey inspect --scheme ed25519 "avoid fiscal stuff nose swift section famous treat blouse mad pitch ladder"

Secret phrase:       avoid fiscal stuff nose swift section famous treat blouse mad pitch ladder
  Secret seed:       0xffff237ecb20b767f8c96bcfaaef30ff1e173e5a48e5664d2d5a3bfdf795b856
  Public key (hex):  0x983aa3ad16c1db5f0cedf29c01e4c15aaf499860eaa55bd611ea92f0db5841f3
  Account ID:        0x983aa3ad16c1db5f0cedf29c01e4c15aaf499860eaa55bd611ea92f0db5841f3
  Public key (SS58): 5FWJZ9poJqWYTfxN4ECR5ZaBMcwMvdzFyDbB4pkJJPXTFrVY
  SS58 Address:      5FWJZ9poJqWYTfxN4ECR5ZaBMcwMvdzFyDbB4pkJJPXTFrVY
```

Node2

```bash
subkey generate --scheme sr25519

Secret phrase:       deal alley inherit mobile welcome globe fun link skill start cat they
  Secret seed:       0x073b2d7880c534576effedb33b358738d03d9d9990bdc6d10db9b5c447e0845d
  Public key (hex):  0x1c423e22f76e124fa56bed1fefee7d0ee868a81519ba3ee884ba2c810a44ac64
  Account ID:        0x1c423e22f76e124fa56bed1fefee7d0ee868a81519ba3ee884ba2c810a44ac64
  Public key (SS58): 5ChksmZFWXUTD9XarqqRPtnwkznUHXHsDvjnGnGQdXksJRhS
  SS58 Address:      5ChksmZFWXUTD9XarqqRPtnwkznUHXHsDvjnGnGQdXksJRhS
  
  
subkey inspect --scheme ed25519 "deal alley inherit mobile welcome globe fun link skill start cat they"

Secret phrase:       deal alley inherit mobile welcome globe fun link skill start cat they
  Secret seed:       0x073b2d7880c534576effedb33b358738d03d9d9990bdc6d10db9b5c447e0845d
  Public key (hex):  0xc072b2c00193efcf6dd73164f873cd6fb9ffdafdd42819df5e2cb7c8fa404d49
  Account ID:        0xc072b2c00193efcf6dd73164f873cd6fb9ffdafdd42819df5e2cb7c8fa404d49
  Public key (SS58): 5GR387mctUry6gsxGHGdBhQWzZ7icCGSwgGcJgkcAyR6QCam
  SS58 Address:      5GR387mctUry6gsxGHGdBhQWzZ7icCGSwgGcJgkcAyR6QCam
```

### 修改customSpec.json文件

将arua的authorities部分，更换为node01、node02的sr25519 类型的SS58 Address

将grandpa的authorities部分，更换为node01、node02的ed25519 类型的SS58 Address

将sudo部分，更换为node01的sr25519 类型的SS58 Address

### 将customSpec.json转换为原生的chain spec文件

```bash
./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```



## 第三部分： 通过ChainSpec，部署公开测试网络

启动node01节点

```bash
./target/release/node-template \
  --base-path /tmp/node01 \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode01 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0'
```

启动node02节点

```bash
./target/release/node-template \
  --base-path /tmp/node02 \
  --chain ./customSpecRaw.json \
  --port 31334 \
  --ws-port 19945 \
  --rpc-port 19933 \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode02 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<Peer ID> \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0'
```

###  上传Node01、Node02密钥

创建node01_aura.json 文件，填写如下内容

```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": [
    "aura",
    // sr25519 Secret phrase
    "",
    // sr25519 Public key (hex)
    ""
  ]
}
```

创建node01_gran.json 文件，填写如下内容：

```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": [
    "gran",
    // ed25519 Secret phrase
    "",
    // ed25519 Public key (hex)
    ""
  ]
}
```

将node01的密钥上传至MyNode01节点

```bash
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@<HOME PATH>/substrate-node-template/node01_aura.json"
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@<HOME PATH>/substrate-node-template/node01_gran.json"
```

node02的密钥需上传至MyNode02节点，其他操作步骤与node01同理。

![Snipaste_2021-10-07_23-39-05.png](https://i.loli.net/2021/10/07/WbVrwlKOe4LIhai.png)

### 重启节点，网络同步出块

节点在添加 GRANDPA 密钥后需要重启，此时区块便可达到最终一致。

![Snipaste_2021-10-07_22-40-04.png](https://i.loli.net/2021/10/07/n8GfaUwjLlkmIep.png)