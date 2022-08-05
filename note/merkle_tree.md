# merkle tree
1. 又名哈希树，哈希函数组成的二进制树，用于块中交易摘要。
树通过生成整个交易集的数字指纹来存储块中的所有交易，从叶子节点开始计重复计算节点的散列对来创建Merkle树，知道计算出树根（Merkle Root），该树根存储在区块头中。
如果任何一个事务发生更改都会导致跟哈希改变，会认为该区块无效。

## MPT（Merkle-Patricia Trie）
融合trie tree和merkle tree的tree, 用于快速验证交易用，存在block header中，表示block body一系列交易列表的摘要信息。
整个以太坊系统中只有一棵状态树，记录整个以太坊系统的所有账户状态。状态树采用Merkel-Patrica(MPT)树。

叶子节点和分支节点可以保存value, 扩展节点保存key；
没有公共的key就成为2个叶子节点；key1=[1,2,3] key2=[2,2,3]
有公共的key需要提取为一个扩展节点；key1=[1,2,3] key2=[1,3,3] => ex-node=[1],下一级分支node的key
如果公共的key也是一个完整的key，数据保存到下一级的分支节点中；key1=[1,2] key2=[1,2,3] =>ex-node=[1,2],下一级分支node的key; 下一级分支=[3],上一级key对应的value

[图解](./mpt.jpeg)

https://zhuanlan.zhihu.com/p/46702178

# SMT（Sparse Merkle Tree）
稀疏默克尔树。

